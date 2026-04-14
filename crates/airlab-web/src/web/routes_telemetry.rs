use crate::log::log_frontend_error;
use crate::web::Result;
use airlab_lib::model::ModelManager as MM;
use axum::Router;
use axum::extract::{Json as eJson, State};
use axum::routing::post;
use serde::Deserialize;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
#[allow(unused_imports)]
use tracing::{debug, info, warn};

#[derive(Deserialize, Debug)]
struct FrontendError {
    message: String,
    stack: Option<String>,
    context: Option<Value>,
    url: String,
    ua: String,
    ts: i64,
}

pub fn routes(mm: MM) -> Router {
    Router::new()
        .route(
            "/api/telemetry/frontend-error",
            post(api_post_telemetry_handler),
        )
        .with_state(mm)
}

async fn api_post_telemetry_handler(
    State(_mm): State<MM>,
    eJson(e): eJson<FrontendError>,
) -> Result<()> {
    info!("HANDLER - api_post_telemetry_handler: {:?}", e);
    tracing::error!(
      target = "frontend",
      message = %e.message,
      url = %e.url,
      context = ?e.context,
      stack = ?e.stack
    );
    log_frontend_error(
        e.message.clone(),
        e.stack.clone(),
        e.context.clone(),
        e.url.clone(),
        e.ua.clone(),
        e.ts,
    );
    append_frontend_error_tsv("/tmp/airlab_telemetry.tsv", &e);
    Ok(())
}

fn append_frontend_error_tsv(path: &str, e: &FrontendError) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        // Replace tabs/newlines so you don't corrupt TSV structure
        let clean = |s: &str| s.replace('\t', " ").replace('\n', "\\n").replace('\r', "");

        let _ = writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}",
            e.ts,
            clean(&e.message),
            clean(e.stack.as_deref().unwrap_or("")),
            clean(&e.url),
            clean(&e.ua),
            clean(
                &e.context
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default()
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[test]
    fn append_frontend_error_tsv_sanitizes_tabs_and_newlines() -> TestResult {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("airlab-telemetry-{}.tsv", std::process::id()));
        let event = FrontendError {
            message: "bad\tmessage".into(),
            stack: Some("line1\nline2".into()),
            context: Some(serde_json::json!({"k":"v"})),
            url: "https://example.test".into(),
            ua: "agent\rvalue".into(),
            ts: 123,
        };

        append_frontend_error_tsv(path.to_string_lossy().as_ref(), &event);

        let content = std::fs::read_to_string(&path)?;
        assert!(content.contains("bad message"));
        assert!(content.contains("line1\\nline2"));
        assert!(!content.contains('\r'));
        Ok(())
    }

    #[tokio::test]
    async fn telemetry_route_accepts_frontend_error() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = routes((*mm).clone());
        let payload = serde_json::json!({
            "message": "frontend crash",
            "stack": "stack",
            "context": {"page":"home"},
            "url": "https://example.test",
            "ua": "browser",
            "ts": 123
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/telemetry/frontend-error")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(payload.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        Ok(())
    }
}
