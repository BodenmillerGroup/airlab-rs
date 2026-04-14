#![allow(clippy::module_name_repetitions)]
use crate::envs::get_env;
use crate::envs::{get_env_b64u_as_u8s, get_env_parse};
use std::sync::OnceLock;

pub fn core_config() -> crate::envs::Result<&'static CoreConfig> {
    static INSTANCE: OnceLock<crate::envs::Result<CoreConfig>> = OnceLock::new();

    match INSTANCE.get_or_init(CoreConfig::load_from_env).as_ref() {
        Ok(config) => Ok(config),
        Err(err) => Err(*err),
    }
}

#[allow(non_snake_case, dead_code)]
pub struct CoreConfig {
    pub DB_URL: String,
}

impl CoreConfig {
    fn load_from_env() -> crate::envs::Result<Self> {
        Ok(Self {
            DB_URL: get_env("SERVICE_DB_URL")?,
        })
    }
}

pub fn auth_config() -> crate::envs::Result<&'static AuthConfig> {
    static INSTANCE: OnceLock<crate::envs::Result<AuthConfig>> = OnceLock::new();

    match INSTANCE.get_or_init(AuthConfig::load_from_env).as_ref() {
        Ok(config) => Ok(config),
        Err(err) => Err(*err),
    }
}

#[allow(non_snake_case)]
pub struct AuthConfig {
    pub PWD_KEY: Vec<u8>,

    pub TOKEN_KEY: Vec<u8>,
    pub TOKEN_DURATION_SEC: f64,
}

impl AuthConfig {
    fn load_from_env() -> crate::envs::Result<Self> {
        Ok(Self {
            PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,

            TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
            TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,
        })
    }
}
