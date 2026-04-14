use airlab_lib::envs;
use airlab_lib::model;
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Model(model::Error),
    #[from]
    Env(envs::Error),
    #[from]
    Io(std::io::Error),
    #[from]
    Migrate(sqlx::migrate::MigrateError),
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
        let error = Error::from(model::Error::EntityNotFound {
            entity: "provider",
            id: 7,
        });

        assert_eq!(
            error.to_string(),
            "Model(EntityNotFound { entity: \"provider\", id: 7 })"
        );
    }
}
