use crate::log::log_request;
use crate::web;
use crate::web::mw_auth::CtxW;
use axum::Json;
use axum::body::Body;
use axum::extract::Extension;
use axum::http::Request;
use axum::http::header::HeaderValue;
use axum::http::{Method, Uri};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;
use tracing::debug;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct RequestLogContext {
    pub request_id: Uuid,
    pub started_at: Instant,
}

impl RequestLogContext {
    fn new(request_id: Uuid) -> Self {
        Self {
            request_id,
            started_at: Instant::now(),
        }
    }
}

pub async fn mw_request_track(mut req: Request<Body>, next: Next) -> Response {
    let request_id = Uuid::new_v4();
    let req_ctx = RequestLogContext::new(request_id);
    req.extensions_mut().insert(req_ctx);

    let mut res = next.run(req).await;
    if let Ok(value) = HeaderValue::from_str(&request_id.to_string()) {
        res.headers_mut().insert("x-request-id", value);
    }
    res
}

pub async fn mw_reponse_map(
    ctx: core::result::Result<CtxW, web::Error>,
    req_ctx: Option<Extension<RequestLogContext>>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    let ctx = ctx.ok().map(|ctx| ctx.0);

    debug!("RES_MAPPER - mw_reponse_map");
    let request_log_context = req_ctx.map(|e| e.0);
    let request_id = request_log_context.map_or_else(Uuid::new_v4, |c| c.request_id);
    let duration_ms = request_log_context.map_or(0, |c| c.started_at.elapsed().as_millis());
    let response_status = res.status().as_u16();

    let web_error = res.extensions().get::<Arc<web::Error>>();
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error.as_ref().map(|(status_code, _)| {
        let client_error_body = json!({});

        debug!("CLIENT ERROR BODY:\n{client_error_body}");

        (*status_code, Json(client_error_body)).into_response()
    });
    let client_error = client_status_error.unzip().1;
    let final_status = error_response
        .as_ref()
        .map_or(response_status, |mapped| mapped.status().as_u16());
    log_request(
        request_id,
        req_method,
        uri,
        final_status,
        duration_ms,
        ctx,
        web_error.map(Arc::as_ref),
        client_error,
    );

    debug!("\n");

    error_response.unwrap_or(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::Error;
    use axum::Router;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn request_track_adds_request_id_header() -> TestResult {
        let app = Router::new()
            .route("/hello", axum::routing::get(|| async { StatusCode::OK }))
            .layer(axum::middleware::from_fn(mw_request_track));

        let response = app
            .oneshot(Request::builder().uri("/hello").body(Body::empty())?)
            .await?;

        assert!(response.headers().contains_key("x-request-id"));
        Ok(())
    }

    #[tokio::test]
    async fn response_map_turns_web_error_into_client_error_status() {
        let mut response = Error::LoginFailUsernameNotFound.into_response();
        response
            .extensions_mut()
            .insert(Arc::new(Error::LoginFailUsernameNotFound));

        let mapped = mw_reponse_map(
            Err(Error::CtxExt(
                crate::web::mw_auth::CtxExtError::CtxNotInRequestExt,
            )),
            None,
            Uri::from_static("/api/v1/login"),
            Method::POST,
            response,
        )
        .await;

        assert_eq!(mapped.status(), StatusCode::FORBIDDEN);
    }
}
