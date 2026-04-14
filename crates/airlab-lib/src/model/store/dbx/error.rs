use derive_more::From;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    TxnCantCommitNoOpenTxn,
    CannotBeginTxnWithTxnFalse,
    CannotCommitTxnWithTxnFalse,
    NoTxn,

    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_uses_debug_output() {
        assert_eq!(
            Error::TxnCantCommitNoOpenTxn.to_string(),
            "TxnCantCommitNoOpenTxn"
        );
        assert_eq!(Error::NoTxn.to_string(), "NoTxn");
    }

    #[test]
    fn result_alias_uses_dbx_error() {
        let result: Result<()> = Err(Error::CannotBeginTxnWithTxnFalse);
        assert!(matches!(result, Err(Error::CannotBeginTxnWithTxnFalse)));
    }
}
