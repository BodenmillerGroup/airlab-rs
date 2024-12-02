use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tracing::debug;

pub fn routes() -> Router {
    Router::new().route("/ws/*path", get(websocket_handler))
}

async fn websocket_handler(ws: WebSocketUpgrade, Path(pth): Path<String>) -> impl IntoResponse {
    debug!("HANDLER - api_get_websocket_handler: {}", pth);
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut _socket: WebSocket) {
    //client does not expect anything
}
