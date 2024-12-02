#![allow(clippy::module_name_repetitions)]
use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case, dead_code)]
pub struct CoreConfig {
    pub DB_URL: String,

    pub AIRLAB_WEB_FOLDER: String,
    pub HISTOCAT_WEB_FOLDER: String,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<Self> {
        Ok(Self {
            DB_URL: get_env("SERVICE_DB_URL")?,

            AIRLAB_WEB_FOLDER: get_env("SERVICE_AIRLAB_WEB_FOLDER")?,
            HISTOCAT_WEB_FOLDER: get_env("SERVICE_HISTOCAT_WEB_FOLDER")?,
        })
    }
}
