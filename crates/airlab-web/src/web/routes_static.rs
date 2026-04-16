use crate::log::log_route_not_found;
use crate::web_config;
use axum::handler::HandlerWithoutStateExt;
use axum::http::Uri;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{MethodRouter, any_service};
use tower_http::services::ServeDir;
use tracing::warn;

async fn handle_404(uri: Uri) -> Response {
    warn!("handle_404: Resource not found");
    log_route_not_found(uri.to_string());
    Redirect::to("/").into_response()
}

pub fn serve_dir() -> crate::Result<MethodRouter> {
    Ok(any_service(
        ServeDir::new(&web_config()?.WEB_FOLDER).fallback(handle_404.into_service()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    #[serial]
    async fn static_fallback_redirects_to_root() -> TestResult {
        crate::web::test_support::init_web_test_env();
        let response = serve_dir()?
            .oneshot(
                axum::http::Request::builder()
                    .uri("/missing")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER);
        assert_eq!(
            response
                .headers()
                .get(axum::http::header::LOCATION)
                .and_then(|value| value.to_str().ok()),
            Some("/")
        );

        Ok(())
    }
}
