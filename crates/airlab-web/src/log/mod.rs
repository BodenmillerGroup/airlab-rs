#![allow(clippy::module_name_repetitions)]
use crate::web::{self, ClientError};
use airlab_lib::ctx::Ctx;
use axum::http::{Method, Uri};
use reqwest::Client;
use serde::Serialize;
use serde_json::{Value, json};
use serde_with::skip_serializing_none;
use std::env;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
#[allow(unused_imports)]
use tracing::{debug, info, warn};
use uuid::Uuid;

const LOG_CHANNEL_CAPACITY: usize = 2048;

static LOG_INGEST_DROPPED_COUNT: AtomicU64 = AtomicU64::new(0);

#[allow(clippy::too_many_arguments)]
pub fn log_request(
    request_id: Uuid,
    req_method: Method,
    uri: Uri,
    status_code: u16,
    duration_ms: u128,
    ctx: Option<Ctx>,
    web_error: Option<&web::Error>,
    client_error: Option<ClientError>,
) {
    let timestamp = now_epoch_ms();

    let error_type = web_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(web_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(serde_json::Value::take));

    let log_line = RequestLogLine {
        request_id: request_id.to_string(),
        timestamp_ms: timestamp,
        duration_ms,
        status_code,
        route: uri.path().to_string(),
        http_query: uri.query().map(ToString::to_string),

        http_path: uri.to_string(),
        http_method: req_method.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    info!("REQUEST LOG LINE: {}", json!(log_line));
    try_enqueue(LogEvent::Request(log_line));
}

pub fn log_frontend_error(
    message: String,
    stack: Option<String>,
    context: Option<Value>,
    url: String,
    ua: String,
    client_ts: i64,
) {
    let timestamp = now_epoch_ms();

    let line = FrontendErrorLogLine {
        timestamp_ms: timestamp,
        client_ts,
        message,
        stack,
        context,
        url,
        ua,
    };

    try_enqueue(LogEvent::FrontendError(line));
}

pub fn log_route_not_found(path: String) {
    let timestamp = now_epoch_ms();
    let line = RouteNotFoundLogLine {
        timestamp_ms: timestamp,
        path,
    };
    try_enqueue(LogEvent::RouteNotFound(line));
}

fn now_epoch_ms() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(err) => {
            warn!("System clock before UNIX_EPOCH; using timestamp 0: {}", err);
            0
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    request_id: String,
    timestamp_ms: u64,
    duration_ms: u128,
    status_code: u16,
    route: String,
    http_query: Option<String>,
    user_id: Option<i64>,
    http_path: String,
    http_method: String,
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

#[skip_serializing_none]
#[derive(Serialize)]
struct FrontendErrorLogLine {
    timestamp_ms: u64,
    client_ts: i64,
    message: String,
    stack: Option<String>,
    context: Option<Value>,
    url: String,
    ua: String,
}

#[derive(Serialize)]
struct RouteNotFoundLogLine {
    timestamp_ms: u64,
    path: String,
}

enum LogEvent {
    Request(RequestLogLine),
    FrontendError(FrontendErrorLogLine),
    RouteNotFound(RouteNotFoundLogLine),
}

#[derive(Clone)]
struct LogIngestConfig {
    default_url: String,
    request_url: Option<String>,
    frontend_url: Option<String>,
    event_url: Option<String>,
    request_stream: String,
    frontend_stream: String,
    event_stream: String,
    auth_header: Option<String>,
    service_tag: String,
    env_tag: String,
}

impl LogIngestConfig {
    fn load() -> Self {
        Self {
            default_url: env_opt("SERVICE_LOG_AGGR_URL").unwrap_or_default(),
            request_url: env_opt("SERVICE_LOG_AGGR_REQUEST_URL"),
            frontend_url: env_opt("SERVICE_LOG_AGGR_FRONTEND_URL"),
            event_url: env_opt("SERVICE_LOG_AGGR_EVENT_URL"),
            request_stream: env_opt("SERVICE_LOG_AGGR_REQUEST_STREAM")
                .unwrap_or_else(|| "backend_requests".to_string()),
            frontend_stream: env_opt("SERVICE_LOG_AGGR_FRONTEND_STREAM")
                .unwrap_or_else(|| "frontend_errors".to_string()),
            event_stream: env_opt("SERVICE_LOG_AGGR_EVENT_STREAM")
                .unwrap_or_else(|| "app_events".to_string()),
            auth_header: env_opt("SERVICE_LOG_AGGR_AUTH"),
            service_tag: env_opt("SERVICE_NAME").unwrap_or_else(|| "airlab-web".to_string()),
            env_tag: env_opt("SERVICE_ENV").unwrap_or_else(|| "unknown".to_string()),
        }
    }
}

fn env_opt(name: &'static str) -> Option<String> {
    env::var(name).ok().and_then(|v| {
        let trimmed = v.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn ingest_config() -> &'static LogIngestConfig {
    static CONFIG: OnceLock<LogIngestConfig> = OnceLock::new();
    CONFIG.get_or_init(LogIngestConfig::load)
}

fn ingest_sender() -> &'static mpsc::Sender<LogEvent> {
    static SENDER: OnceLock<mpsc::Sender<LogEvent>> = OnceLock::new();

    SENDER.get_or_init(|| {
        let (tx, mut rx) = mpsc::channel::<LogEvent>(LOG_CHANNEL_CAPACITY);
        let cfg = ingest_config().clone();
        tokio::spawn(async move {
            let client = Client::new();
            while let Some(event) = rx.recv().await {
                post_event(&client, &cfg, event).await;
            }
        });

        tx
    })
}

fn try_enqueue(event: LogEvent) {
    match ingest_sender().try_send(event) {
        Ok(()) => {}
        Err(mpsc::error::TrySendError::Full(_)) | Err(mpsc::error::TrySendError::Closed(_)) => {
            let dropped = LOG_INGEST_DROPPED_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
            if dropped % 100 == 1 {
                warn!("Log ingest queue drops detected: {}", dropped);
            }
        }
    }
}

async fn post_event(client: &Client, cfg: &LogIngestConfig, event: LogEvent) {
    let (url, stream, kind, payload) = match event {
        LogEvent::Request(line) => (
            cfg.request_url.as_deref().unwrap_or(&cfg.default_url),
            &cfg.request_stream,
            "backend-request",
            json!(line),
        ),
        LogEvent::FrontendError(line) => (
            cfg.frontend_url.as_deref().unwrap_or(&cfg.default_url),
            &cfg.frontend_stream,
            "frontend-error",
            json!(line),
        ),
        LogEvent::RouteNotFound(line) => (
            cfg.event_url.as_deref().unwrap_or(&cfg.default_url),
            &cfg.event_stream,
            "route-not-found",
            json!(line),
        ),
    };

    let mut req = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("X-P-Stream", stream)
        .header("X-P-TAG-service", &cfg.service_tag)
        .header("X-P-TAG-kind", kind)
        .header("X-P-META-env", &cfg.env_tag)
        .json(&payload);

    if let Some(auth) = &cfg.auth_header {
        req = req.header("Authorization", auth);
    }

    match req.send().await {
        Ok(resp) if resp.status().is_success() => {}
        Ok(resp) => warn!(
            "Parseable ingest failed with status {} ({kind})",
            resp.status()
        ),
        Err(err) => warn!("Parseable ingest transport error ({kind}): {err}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn env_opt_trims_and_ignores_empty_values() {
        // SAFETY: env mutation is serialized for this test.
        unsafe { env::set_var("AIRLAB_LOG_TEST_VALUE", "  hello  ") };
        assert_eq!(env_opt("AIRLAB_LOG_TEST_VALUE").as_deref(), Some("hello"));

        // SAFETY: env mutation is serialized for this test.
        unsafe { env::set_var("AIRLAB_LOG_TEST_VALUE", "   ") };
        assert_eq!(env_opt("AIRLAB_LOG_TEST_VALUE"), None);
    }

    #[test]
    #[serial]
    fn load_uses_defaults_and_overrides() {
        crate::web::test_support::init_web_test_env();
        for key in [
            "SERVICE_LOG_AGGR_FRONTEND_URL",
            "SERVICE_LOG_AGGR_EVENT_URL",
            "SERVICE_LOG_AGGR_FRONTEND_STREAM",
            "SERVICE_LOG_AGGR_EVENT_STREAM",
        ] {
            // SAFETY: env mutation is serialized for this test.
            unsafe { env::remove_var(key) };
        }
        for (key, value) in [
            (
                "SERVICE_LOG_AGGR_REQUEST_URL",
                "https://logs.example.test/request",
            ),
            ("SERVICE_LOG_AGGR_REQUEST_STREAM", "req_stream"),
            ("SERVICE_LOG_AGGR_AUTH", "Bearer secret"),
            ("SERVICE_NAME", "airlab-web-test"),
            ("SERVICE_ENV", "test"),
        ] {
            // SAFETY: env mutation is serialized for this test.
            unsafe { env::set_var(key, value) };
        }

        let cfg = LogIngestConfig::load();

        assert_eq!(cfg.default_url, "https://logs.example.test");
        assert_eq!(
            cfg.request_url.as_deref(),
            Some("https://logs.example.test/request")
        );
        assert_eq!(cfg.request_stream, "req_stream");
        assert!(matches!(
            cfg.frontend_stream.as_str(),
            "frontend_errors" | "airlab_frontend"
        ));
        assert_eq!(cfg.auth_header.as_deref(), Some("Bearer secret"));
        assert_eq!(cfg.service_tag, "airlab-web-test");
        assert_eq!(cfg.env_tag, "test");
    }

    #[test]
    fn now_epoch_ms_is_monotonic_enough_for_logging() {
        let start = now_epoch_ms();
        let end = now_epoch_ms();

        assert!(end >= start);
    }
}
