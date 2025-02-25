#![allow(clippy::module_name_repetitions)]
use airlab_lib::envs::{get_env, get_env_parse};
use std::sync::OnceLock;

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct WebConfig {
    pub WEB_FOLDER: String,
    pub EMAIL_FROM_ADDRESS: String,
    pub EMAIL_FROM_NAME: String,
    pub EMAIL_TOKEN: String,
    pub EMAIL_ADDRESS: String,
    pub LOG_AGGR_URL: String,
    pub RESET_PWD_URL: String,
    pub DATA_PATH: String,
    pub HOST_ADDR: String,
    pub HOST_PORT: String,
    pub SUPER_USER: String,
    pub SUPER_USER_PWD: String,
    pub SETUP_DEMO_GROUP: bool,
}

impl WebConfig {
    fn load_from_env() -> airlab_lib::envs::Result<Self> {
        Ok(Self {
            HOST_PORT: get_env("SERVICE_HOST_PORT")?,
            HOST_ADDR: get_env("SERVICE_HOST_ADDR")?,
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
            EMAIL_FROM_ADDRESS: get_env("SERVICE_EMAIL_FROM_ADDRESS")?,
            EMAIL_FROM_NAME: get_env("SERVICE_EMAIL_FROM_NAME")?,
            EMAIL_TOKEN: get_env("SERVICE_EMAIL_TOKEN")?,
            EMAIL_ADDRESS: get_env("SERVICE_EMAIL_ADDRESS")?,
            LOG_AGGR_URL: get_env("SERVICE_LOG_AGGR_URL")?,
            RESET_PWD_URL: get_env("SERVICE_RESET_PWD_URL")?,
            DATA_PATH: get_env("SERVICE_DATA_PATH")?,
            SUPER_USER: get_env("SUPER_USER")?,
            SUPER_USER_PWD: get_env("SUPER_USER_PWD")?,
            SETUP_DEMO_GROUP: get_env_parse("SETUP_DEMO_GROUP")?,
        })
    }
}
