#![allow(clippy::module_name_repetitions)]
mod error;

pub use self::error::{Error, Result};

use crate::b64::{b64u_decode_to_string, b64u_encode};
use crate::config::auth_config;
use crate::time::{now_utc, now_utc_plus_sec_str, parse_utc};
use hmac::{Hmac, Mac};
use sha2::Sha512;
use std::fmt::Display;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Token {
    pub ident: String,
    pub exp: String,
    pub sign_b64u: String,
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(Error::InvalidFormat);
        }
        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64u_decode_to_string(ident_b64u).map_err(|_| Error::CannotDecodeIdent)?,

            exp: b64u_decode_to_string(exp_b64u).map_err(|_| Error::CannotDecodeExp)?,

            sign_b64u: sign_b64u.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

pub fn generate_web_token(user: &str, salt: Uuid) -> Result<Token> {
    let config = &auth_config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: Uuid) -> Result<()> {
    let config = &auth_config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}

fn _generate_token(ident: &str, duration_sec: f64, salt: Uuid, key: &[u8]) -> Result<Token> {
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: Uuid, key: &[u8]) -> Result<()> {
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

    if new_sign_b64u != origin_token.sign_b64u {
        return Err(Error::SignatureNotMatching);
    }

    let origin_exp = parse_utc(&origin_token.exp).map_err(|_| Error::ExpNotIso)?;
    let now = now_utc();

    if origin_exp < now {
        return Err(Error::Expired);
    }

    Ok(())
}

fn _token_sign_into_b64u(ident: &str, exp: &str, salt: Uuid, key: &[u8]) -> Result<String> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));

    let mut hmac_sha512 =
        Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::HmacFailNewFromSlice)?;

    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = b64u_encode(result_bytes);

    Ok(result)
}
