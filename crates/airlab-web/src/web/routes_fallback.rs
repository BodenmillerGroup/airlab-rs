use crate::web::Result;
use crate::web::mw_auth::CtxW;
use crate::web_config;
use airlab_lib::model::ModelManager;
use axum::extract::State;
use axum::routing::get;
use axum::{Router, response::Html};
use std::fs;
use std::io;
use tokio::task;
#[allow(unused_imports)]
use tracing::{debug, warn};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/main/{*all}", get(api_get_main_handler))
        .with_state(mm)
}

async fn api_get_main_handler(State(_mm): State<ModelManager>, _ctx: CtxW) -> Result<Html<String>> {
    debug!("HANDLER - api_get_main_handler");
    let web_folder = web_config()?.WEB_FOLDER.clone();
    let html_content =
        task::spawn_blocking(move || fs::read_to_string(format!("{}/index.html", web_folder)))
            .await
            .map_err(|err| io::Error::other(format!("Task panicked: {err}")))??;

    Ok(Html(html_content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn main_route_serves_index_html() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/main/app")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            crate::web::test_support::response_body_string(response).await?,
            "<html>airlab-test</html>"
        );

        Ok(())
    }
}
