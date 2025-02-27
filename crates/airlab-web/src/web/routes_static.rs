use crate::web_config;
use axum::handler::HandlerWithoutStateExt;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{MethodRouter, any_service};
use reqwest::Client;
use serde::Serialize;
use tower_http::services::ServeDir;
use tracing::warn;

#[derive(Serialize)]
struct Msg {
    route_not_found: String,
}

async fn handle_404() -> Response {
    warn!("handle_404: Resource not found");
    let url = &web_config().LOG_AGGR_URL;
    let client = Client::new();
    let msg = Msg {
        route_not_found: "not found".into(),
    };
    match client
        .post(url)
        .header("X-P-META-meta1", "value1")
        .header("X-P-TAG-tag1", "airlab-web-service")
        .header("Authorization", "Basic YWRtaW46YWRtaW4=")
        .header("Content-Type", "application/json")
        .json(&msg)
        .send()
        .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("Cannot post the message to {url}: {e}");
        }
    };
    Redirect::to("/").into_response()
}

pub fn serve_dir() -> MethodRouter {
    any_service(ServeDir::new(&web_config().WEB_FOLDER).fallback(handle_404.into_service()))
}
