#![allow(clippy::missing_errors_doc)]
mod config;
mod error;
mod log;
mod web;
mod web_util;

use crate::web::mw_auth::mw_ctx_resolve;
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{
    routes_clone, routes_conjugate, routes_fallback, routes_group, routes_login, routes_lot,
    routes_member, routes_panel, routes_panel_element, routes_protein, routes_provider,
    routes_species, routes_static, routes_tag, routes_user, routes_validation,
    routes_validation_file, routes_ws,
};
use axum::{middleware, Router};
use config::web_config;
use lib_core::model::ModelManager;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::debug;
use tracing_subscriber::EnvFilter;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mm = ModelManager::new().await?;

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .merge(routes_user::routes(mm.clone()))
        .merge(routes_group::routes(mm.clone()))
        .merge(routes_protein::routes(mm.clone()))
        .merge(routes_panel_element::routes(mm.clone()))
        .merge(routes_validation::routes(mm.clone()))
        .merge(routes_validation_file::routes(mm.clone()))
        .merge(routes_panel::routes(mm.clone()))
        .merge(routes_member::routes(mm.clone()))
        .merge(routes_conjugate::routes(mm.clone()))
        .merge(routes_clone::routes(mm.clone()))
        .merge(routes_lot::routes(mm.clone()))
        .merge(routes_species::routes(mm.clone()))
        .merge(routes_provider::routes(mm.clone()))
        .merge(routes_tag::routes(mm.clone()))
        .merge(routes_fallback::routes(mm.clone()))
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .merge(routes_ws::routes())
        .fallback_service(routes_static::serve_dir());

    #[allow(clippy::unwrap_used)]
    let listener = TcpListener::bind(&format!(
        "{}:{}",
        &web_config().HOST_ADDR,
        &web_config().HOST_PORT
    ))
    .await
    .unwrap();
    debug!("LISTENING - {:?}\n", listener.local_addr());
    #[allow(clippy::unwrap_used)]
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
