use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    CtxCannotNewRootCtx,
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
            Error::CtxCannotNewRootCtx.to_string(),
            "CtxCannotNewRootCtx"
        );
    }

    #[test]
    fn result_alias_uses_ctx_error() {
        let result: Result<()> = Err(Error::CtxCannotNewRootCtx);
        assert!(matches!(result, Err(Error::CtxCannotNewRootCtx)));
    }
}
