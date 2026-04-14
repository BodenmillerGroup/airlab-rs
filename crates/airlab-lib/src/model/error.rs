use crate::model::store::dbx;
use crate::pwd;
use derive_more::From;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    ListLimitOverMax {
        max: i64,
        actual: i64,
    },

    CountFail,

    CantCreateModelManagerProvider(String),

    #[from]
    Env(crate::envs::Error),

    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

    #[from]
    Dbx(dbx::Error),

    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),

    #[from]
    ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError),

    #[from]
    Pwd(pwd::Error),

    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
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
        let error = Error::EntityNotFound {
            entity: "provider",
            id: 7,
        };

        assert_eq!(
            error.to_string(),
            "EntityNotFound { entity: \"provider\", id: 7 }"
        );
    }

    #[test]
    fn result_alias_uses_model_error() {
        let result: Result<()> = Err(Error::CountFail);
        assert!(matches!(result, Err(Error::CountFail)));
    }
}
