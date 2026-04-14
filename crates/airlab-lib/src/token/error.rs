use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    HmacFailNewFromSlice,

    Env(crate::envs::Error),

    InvalidFormat,
    CannotDecodeIdent,
    CannotDecodeExp,
    CannotFormatExp,
    SignatureNotMatching,
    ExpNotIso,
    Expired,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<crate::envs::Error> for Error {
    fn from(_: crate::envs::Error) -> Self {
        Self::InvalidFormat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_uses_debug_name() {
        assert_eq!(Error::Expired.to_string(), "Expired");
        assert_eq!(Error::InvalidFormat.to_string(), "InvalidFormat");
    }

    #[test]
    fn result_alias_uses_error_type() {
        let result: Result<()> = Err(Error::SignatureNotMatching);
        assert!(matches!(result, Err(Error::SignatureNotMatching)));
    }
}
