use crate::web::mw_auth::CtxW;
use crate::web::Result;
use crate::web_config;
use airlab_lib::model::ModelManager;
use axum::extract::State;
use axum::routing::get;
use axum::{response::Html, Router};
use std::fs;
use tokio::task;
#[allow(unused_imports)]
use tracing::{debug, warn};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/main/*all", get(api_get_main_handler))
        .with_state(mm)
}

async fn api_get_main_handler(State(_mm): State<ModelManager>, _ctx: CtxW) -> Result<Html<String>> {
    debug!("HANDLER - api_get_main_handler");
    let html_content = task::spawn_blocking(|| {
        fs::read_to_string(format!("{}/index.html", &web_config().WEB_FOLDER))
    })
    .await
    .expect("Task panicked")?;

    Ok(Html(html_content))
}
