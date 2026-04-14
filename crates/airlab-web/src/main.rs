#![allow(clippy::missing_errors_doc)]
mod config;
mod error;
mod log;
mod search_shadow;
mod util;
mod web;

use crate::search_shadow::SearchState;
use crate::web::mw_auth::mw_ctx_resolve;
use crate::web::mw_res_map::{mw_reponse_map, mw_request_track};
use crate::web::{
    routes_fallback, routes_group, routes_json, routes_login, routes_search, routes_static,
    routes_telemetry, routes_user, routes_validation_file, routes_ws,
};
use airlab_lib::model::ModelManager;
use airlab_lib::model::user::{UserBmc, UserForCreate, UserForUpdate};
use axum::{Router, middleware};
use config::web_config;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
#[allow(unused_imports)]
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

pub use self::error::{Error, Result};

async fn setup_admin_user(mm: &ModelManager) -> Result<()> {
    let ctx = airlab_lib::ctx::Ctx::root_ctx();
    let users = UserBmc::list(&ctx, mm, None, None).await?;
    if users.is_empty() {
        let config = web_config()?;
        // only if no users exists
        let super_user = UserForCreate {
            username: Some(config.SUPER_USER.clone()),
            pwd_clear: None,
            email: config.SUPER_USER.clone(),
            name: Some(config.SUPER_USER.clone()),
        };
        let user_id = UserBmc::create(&ctx, mm, super_user).await?;
        UserBmc::update_pwd(&ctx, mm, user_id, &config.SUPER_USER_PWD).await?;
        let update = UserForUpdate {
            is_active: Some(true),
            is_admin: Some(true),
            ..Default::default()
        };
        UserBmc::update(&ctx, mm, user_id, update).await?;
        if config.SETUP_DEMO_GROUP {
            util::setup_demo_group(&ctx, mm, user_id).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mm = ModelManager::new().await?;
    sqlx::migrate!().run(mm.db()).await?;

    setup_admin_user(&mm).await?;
    let search_state = SearchState::new(mm.clone());

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .merge(routes_user::routes(mm.clone()))
        .merge(routes_group::routes(mm.clone()))
        .merge(routes_fallback::routes(mm.clone()))
        .merge(routes_json::routes(search_state.clone()))
        .merge(routes_validation_file::routes(mm.clone()))
        .merge(routes_telemetry::routes(mm.clone()))
        .merge(routes_search::routes(search_state))
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(middleware::from_fn(mw_request_track))
        .layer(CookieManagerLayer::new())
        .merge(routes_ws::routes())
        .fallback_service(routes_static::serve_dir()?);

    let config = web_config()?;
    let listener =
        TcpListener::bind(&format!("{}:{}", &config.HOST_ADDR, &config.HOST_PORT)).await?;
    info!("LISTENING - {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service()).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use airlab_lib::ctx::Ctx;
    use airlab_lib::model::user::{User, UserBmc};

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn setup_admin_user_does_not_create_super_user_when_users_exist() -> TestResult {
        crate::web::test_support::init_web_test_env();
        let mm = airlab_lib::_dev_utils::init_test().await;

        let ctx = Ctx::root_ctx();
        let before = UserBmc::list(&ctx, &mm, None, None).await?.len();

        setup_admin_user(&mm).await?;

        let after = UserBmc::list(&ctx, &mm, None, None).await?.len();
        let user: Option<User> =
            UserBmc::first_by_username::<User>(&ctx, &mm, &web_config()?.SUPER_USER).await?;

        assert_eq!(before, after);
        assert!(user.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn setup_admin_user_is_idempotent_when_users_exist() -> TestResult {
        crate::web::test_support::init_web_test_env();
        let mm = airlab_lib::_dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        setup_admin_user(&mm).await?;
        let before = UserBmc::list(&ctx, &mm, None, None).await?.len();
        setup_admin_user(&mm).await?;
        let after = UserBmc::list(&ctx, &mm, None, None).await?.len();

        assert_eq!(before, after);
        Ok(())
    }
}
