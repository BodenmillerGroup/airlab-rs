#![allow(clippy::module_name_repetitions)]
use airlab_lib::envs::{get_env, get_env_parse};
#[cfg(not(test))]
use std::sync::OnceLock;

pub fn web_config() -> airlab_lib::envs::Result<&'static WebConfig> {
    #[cfg(test)]
    {
        let config = Box::new(WebConfig::load_from_env()?);
        Ok(Box::leak(config))
    }

    #[cfg(not(test))]
    {
        static INSTANCE: OnceLock<airlab_lib::envs::Result<WebConfig>> = OnceLock::new();

        match INSTANCE.get_or_init(WebConfig::load_from_env).as_ref() {
            Ok(config) => Ok(config),
            Err(err) => Err(*err),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct WebConfig {
    pub WEB_FOLDER: String,
    pub EMAIL_FROM_ADDRESS: String,
    pub EMAIL_FROM_NAME: String,
    pub EMAIL_TOKEN: String,
    pub EMAIL_ADDRESS: String,
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
            RESET_PWD_URL: get_env("SERVICE_RESET_PWD_URL")?,
            DATA_PATH: get_env("SERVICE_DATA_PATH")?,
            SUPER_USER: get_env("SUPER_USER")?,
            SUPER_USER_PWD: get_env("SUPER_USER_PWD")?,
            SETUP_DEMO_GROUP: get_env_parse("SETUP_DEMO_GROUP")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn set_required_env() {
        for (key, value) in [
            ("SERVICE_HOST_PORT", "3000"),
            ("SERVICE_HOST_ADDR", "127.0.0.1"),
            ("SERVICE_WEB_FOLDER", "/tmp/airlab-web-config-tests"),
            ("SERVICE_EMAIL_FROM_ADDRESS", "noreply@example.test"),
            ("SERVICE_EMAIL_FROM_NAME", "Airlab"),
            ("SERVICE_EMAIL_TOKEN", "token"),
            ("SERVICE_EMAIL_ADDRESS", "smtp.example.test"),
            ("SERVICE_LOG_AGGR_URL", "https://logs.example.test"),
            ("SERVICE_RESET_PWD_URL", "https://example.test/reset"),
            ("SERVICE_DATA_PATH", "/tmp/airlab-data-config-tests"),
            ("SUPER_USER", "admin@example.test"),
            ("SUPER_USER_PWD", "secret"),
            ("SETUP_DEMO_GROUP", "false"),
        ] {
            // SAFETY: test env vars are serialized with `serial`.
            unsafe { std::env::set_var(key, value) };
        }
    }

    #[test]
    #[serial]
    fn load_from_env_reads_expected_values() -> airlab_lib::envs::Result<()> {
        set_required_env();

        let config = WebConfig::load_from_env()?;

        assert_eq!(config.HOST_ADDR, "127.0.0.1");
        assert_eq!(config.HOST_PORT, "3000");
        assert_eq!(config.SUPER_USER, "admin@example.test");
        assert!(!config.SETUP_DEMO_GROUP);
        Ok(())
    }

    #[test]
    #[serial]
    fn load_from_env_fails_when_required_value_missing() {
        set_required_env();
        // SAFETY: test env vars are serialized with `serial`.
        unsafe { std::env::remove_var("SERVICE_HOST_PORT") };

        let error = WebConfig::load_from_env().expect_err("missing env should fail");

        assert!(error.to_string().contains("SERVICE_HOST_PORT"));
    }
}
