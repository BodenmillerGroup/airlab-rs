#![allow(clippy::module_name_repetitions)]
mod error;
mod hmac_hasher;

pub use self::error::{Error, Result};

use crate::auth_config;
use crate::pwd::hmac_hasher::hmac_sha512_hash;
use uuid::Uuid;

pub struct ContentToHash {
    pub content: String,
    pub salt: Uuid,
}

pub fn hash_pwd(to_hash: &ContentToHash) -> Result<String> {
    let key = &auth_config()?.PWD_KEY;

    let hashed = hmac_sha512_hash(key, to_hash)?;

    Ok(format!("#01#{hashed}"))
}

pub fn validate_pwd(enc_content: &ContentToHash, pwd_ref: &str) -> Result<()> {
    let pwd = hash_pwd(enc_content)?;

    if pwd == pwd_ref {
        Ok(())
    } else {
        Err(Error::NotMatching)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    fn sample_content() -> TestResult<ContentToHash> {
        Ok(ContentToHash {
            content: "secret".into(),
            salt: uuid::Uuid::parse_str("11111111-1111-1111-1111-111111111111")?,
        })
    }

    #[test]
    fn hash_pwd_prefixes_version_marker() -> TestResult {
        _dev_utils::init_test_env();

        let hashed = hash_pwd(&sample_content()?)?;

        assert!(hashed.starts_with("#01#"));
        Ok(())
    }

    #[test]
    fn validate_pwd_accepts_matching_hash() -> TestResult {
        _dev_utils::init_test_env();
        let content = sample_content()?;
        let hashed = hash_pwd(&content)?;

        let result = validate_pwd(&content, &hashed);

        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn validate_pwd_rejects_non_matching_hash() -> TestResult {
        _dev_utils::init_test_env();

        let result = validate_pwd(&sample_content()?, "#01#not-the-right-hash");

        assert!(matches!(result, Err(Error::NotMatching)));
        Ok(())
    }
}
