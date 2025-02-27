#![allow(clippy::module_name_repetitions)]
use crate::Result;
use crate::web::{self, ClientError};
use crate::web_config;
use airlab_lib::ctx::Ctx;
use axum::http::{Method, Uri};
use reqwest::Client;
use serde::Serialize;
use serde_json::{Value, json};
use serde_with::skip_serializing_none;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use uuid::Uuid;

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    web_error: Option<&web::Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = web_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(web_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(serde_json::Value::take));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        http_path: uri.to_string(),
        http_method: req_method.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    debug!("REQUEST LOG LINE:\n{}", json!(log_line));
    let post_url = &web_config().LOG_AGGR_URL;
    let client = Client::new();
    let _ = client
        .post(post_url)
        .header("X-P-META-meta1", "value1")
        .header("X-P-TAG-tag1", "airlab-web-service")
        .header("Authorization", "Basic YWRtaW46YWRtaW4=")
        .header("Content-Type", "application/json")
        .json(&log_line)
        .send()
        .await
        .is_ok();

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,
    user_id: Option<i32>,
    http_path: String,
    http_method: String,
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
