use crate::log::log_request;
use crate::web;
use crate::web::mw_auth::CtxW;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::sync::Arc;
use tracing::debug;
use uuid::Uuid;

pub async fn mw_reponse_map(
    ctx: Option<CtxW>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    let ctx = ctx.map(|ctx| ctx.0);

    debug!("RES_MAPPER - mw_reponse_map");
    let uuid = Uuid::new_v4();

    let web_error = res.extensions().get::<Arc<web::Error>>();
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error.as_ref().map(|(status_code, _)| {
        let client_error_body = json!({});

        debug!("CLIENT ERROR BODY:\n{client_error_body}");

        (*status_code, Json(client_error_body)).into_response()
    });
    let client_error = client_status_error.unzip().1;
    let _ = log_request(
        uuid,
        req_method,
        uri,
        ctx,
        web_error.map(Arc::as_ref),
        client_error,
    )
    .await;

    debug!("\n");

    error_response.unwrap_or(res)
}
