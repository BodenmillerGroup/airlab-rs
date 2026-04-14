use axum::Router;
use axum::extract::Path;
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use tracing::debug;

pub fn routes() -> Router {
    Router::new().route("/ws/{*path}", get(websocket_handler))
}

async fn websocket_handler(ws: WebSocketUpgrade, Path(pth): Path<String>) -> impl IntoResponse {
    debug!("HANDLER - api_get_websocket_handler: {}", pth);
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut _socket: WebSocket) {}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn websocket_route_is_registered() -> TestResult {
        let response = routes()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/ws/demo")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_ne!(response.status(), axum::http::StatusCode::NOT_FOUND);
        Ok(())
    }
}
