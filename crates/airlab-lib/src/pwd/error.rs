use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    KeyFail,

    NotMatching,

    Env(crate::envs::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<crate::envs::Error> for Error {
    fn from(_: crate::envs::Error) -> Self {
        Self::KeyFail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_uses_debug_output() {
        assert_eq!(Error::KeyFail.to_string(), "KeyFail");
        assert_eq!(Error::NotMatching.to_string(), "NotMatching");
    }

    #[test]
    fn result_alias_uses_pwd_error() {
        let result: Result<()> = Err(Error::NotMatching);
        assert!(matches!(result, Err(Error::NotMatching)));
    }
}
