use crate::b64::b64u_encode;
use crate::pwd::{ContentToHash, Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha512;

pub fn hmac_sha512_hash(key: &[u8], to_hash: &ContentToHash) -> Result<String> {
    let ContentToHash { content, salt } = to_hash;

    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFail)?;

    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    let hmac_result = hmac_sha512.finalize();

    let result = b64u_encode(hmac_result.into_bytes());

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    fn sample_content() -> TestResult<ContentToHash> {
        Ok(ContentToHash {
            content: "secret".into(),
            salt: uuid::Uuid::parse_str("11111111-1111-1111-1111-111111111111")?,
        })
    }

    #[test]
    fn hmac_hash_is_stable_for_same_inputs() -> TestResult {
        let content = sample_content()?;

        let first = hmac_sha512_hash(b"0123456789abcdef", &content)?;
        let second = hmac_sha512_hash(b"0123456789abcdef", &content)?;

        assert_eq!(first, second);
        Ok(())
    }

    #[test]
    fn hmac_hash_changes_with_key() -> TestResult {
        let content = sample_content()?;

        let first = hmac_sha512_hash(b"0123456789abcdef", &content)?;
        let second = hmac_sha512_hash(b"fedcba9876543210", &content)?;

        assert_ne!(first, second);
        Ok(())
    }
}
