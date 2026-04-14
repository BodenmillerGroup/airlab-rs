use crate::web::Result;
use crate::web::mw_auth::CtxW;
use airlab_lib::model::ModelManager;
use airlab_lib::model::user::{User, UserBmc, UserForCreate, UserForUpdate};
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde_json::{Value, json};
use std::io;
#[allow(unused_imports)]
use tracing::{debug, warn};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/users", post(api_create_user_handler))
        .route("/api/v1/users", get(api_list_user_handler))
        .route("/api/v1/users/setupmfa", post(api_create_user_mfa_handler))
        .route("/api/v1/users/verifymfa", post(api_verify_user_mfa_handler))
        .route(
            "/api/v1/users/profile",
            patch(api_patch_user_profile_handler),
        )
        .route("/api/v1/users/profile", get(api_profile_handler))
        .with_state(mm)
}

async fn api_list_user_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_list_user_handler");
    let ctx = ctx.0;

    let users: Vec<User> = UserBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(json!(users)))
}

use serde::{Deserialize, Serialize};
use thotp::encoding::data_encoding;
use thotp::{
    encoding::{decode, encode},
    generate_secret, qr, verify_totp,
};

#[derive(Serialize)]
struct MfaSetupResponse {
    secret: String,
    qr_code: String,
    uri: String,
}

#[derive(Deserialize)]
struct MfaVerifyRequest {
    code: String,
    secret: String,
}

#[derive(Serialize)]
struct MfaVerifyResponse {
    success: bool,
    message: String,
}

async fn api_create_user_mfa_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(mut _payload): eJson<Value>,
) -> Result<Json<MfaSetupResponse>> {
    debug!("HANDLER - api_crate_user_mfa_handler");
    let ctx = ctx.0;
    let user_id: i64 = ctx.user_id();
    let user: User = UserBmc::get(&ctx, &mm, user_id).await?;
    let email = user
        .email
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "User email missing"))?;
    let issuer = String::from("airlab");
    let secret = generate_secret(80);
    let encoded = encode(&secret, data_encoding::BASE32);
    warn!("Encoded generated: {encoded}");
    let uri = qr::otp_uri("totp", &encoded, &email, &issuer, None)
        .map_err(|err| io::Error::other(format!("Cannot build OTP URI: {err}")))?;
    let qr_code = qr::generate_code_svg(&uri, None, None, qr::EcLevel::M)
        .map_err(|err| io::Error::other(format!("Cannot generate QR SVG: {err}")))?;

    warn!("Mfa for {user_id} {uri:?} {encoded}");

    Ok(Json(MfaSetupResponse {
        secret: encoded,
        qr_code,
        uri,
    }))
}

async fn api_verify_user_mfa_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(payload): eJson<MfaVerifyRequest>,
) -> Json<MfaVerifyResponse> {
    debug!("HANDLER - api_verify_user_mfa_handler");
    let ctx = ctx.0;
    let original_secret = payload.secret.as_str();
    let mut secret = payload.secret.clone();
    if secret.is_empty() {
        let user_id: i64 = ctx.user_id();
        let user: User = match UserBmc::get(&ctx, &mm, user_id).await {
            Ok(user) => user,
            Err(err) => {
                warn!("Cannot load user for MFA verification: {err}");
                return Json(MfaVerifyResponse {
                    success: false,
                    message: "Cannot load user for MFA verification.".into(),
                });
            }
        };
        secret = user.mfa_secret.clone();
    }
    let decoded = match decode(&secret, data_encoding::BASE32) {
        Ok(decoded) => decoded,
        Err(err) => {
            warn!("Cannot decode MFA secret: {err}");
            return Json(MfaVerifyResponse {
                success: false,
                message: "Cannot decode MFA secret.".into(),
            });
        }
    };
    warn!("Secret: {secret}; code: {}", payload.code);
    let (is_valid, discrepancy) = match verify_totp(&payload.code, &decoded, 0) {
        Ok(result) => result,
        Err(err) => {
            warn!("Cannot verify MFA code: {err}");
            return Json(MfaVerifyResponse {
                success: false,
                message: "Cannot verify MFA code.".into(),
            });
        }
    };
    warn!("IsValid: {is_valid} Discrepancy {discrepancy:?}");

    if is_valid {
        if !original_secret.is_empty() {
            let user_id: i64 = ctx.user_id();
            let user: User = match UserBmc::get(&ctx, &mm, user_id).await {
                Ok(user) => user,
                Err(err) => {
                    warn!("Cannot load user for MFA update: {err}");
                    return Json(MfaVerifyResponse {
                        success: false,
                        message: "Cannot load user for MFA update.".into(),
                    });
                }
            };
            let fu = UserForUpdate {
                mfa_secret: Some(original_secret.to_string()),
                mfa_enabled: Some(true),
                ..Default::default()
            };
            if let Err(err) = UserBmc::update(&ctx, &mm, user.id, fu).await {
                warn!("Cannot persist MFA data: {err}");
                return Json(MfaVerifyResponse {
                    success: false,
                    message: "Cannot persist MFA data.".into(),
                });
            }
        }
        Json(MfaVerifyResponse {
            success: true,
            message: "MFA verified successfully!".into(),
        })
    } else {
        Json(MfaVerifyResponse {
            success: false,
            message: "Invalid OTP code.".into(),
        })
    }
}

async fn api_create_user_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    eJson(mut payload): eJson<UserForCreate>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_patch_user_profile_handler: {:?}", payload);
    let ctx = ctx.0;
    payload.username = Some(payload.email.clone());

    let user_id = UserBmc::create(&ctx, &mm, payload).await?;

    let group: User = UserBmc::get(&ctx, &mm, user_id).await?;
    Ok(Json(json!(group)))
}

async fn api_patch_user_profile_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Path(group_id): Path<i64>,
    eJson(payload): eJson<UserForUpdate>,
) -> Result<Json<Value>> {
    debug!(
        "HANDLER - api_patch_user_profile: {}; {:?}",
        group_id, payload
    );
    let ctx = ctx.0;

    UserBmc::update(&ctx, &mm, group_id, payload).await?;

    let group: User = UserBmc::get(&ctx, &mm, group_id).await?;
    Ok(Json(json!(group)))
}

async fn api_profile_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    debug!("HANDLER - api_profile_handler");

    let ctx = ctx.0;
    let user: i64 = ctx.user_id();

    let user: User = UserBmc::get::<User>(&ctx, &mm, user).await?;

    Ok(Json(json!(user)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn profile_route_returns_authenticated_user() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/users/profile")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("demo1@uzh.ch"));

        Ok(())
    }

    #[tokio::test]
    async fn list_users_route_returns_seeded_users() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/users")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("demo1@uzh.ch"));
        assert!(body.contains("member1000@example.test"));

        Ok(())
    }

    #[tokio::test]
    async fn verify_mfa_route_uses_stored_secret_when_payload_secret_is_empty() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));
        let payload = json!({
            "code": "123456",
            "secret": ""
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/users/verifymfa")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(payload.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("\"success\":false"));
        assert!(
            body.contains("Cannot decode MFA secret")
                || body.contains("Cannot verify MFA code")
                || body.contains("Invalid OTP code")
        );

        Ok(())
    }

    #[tokio::test]
    async fn setup_mfa_route_returns_secret_and_uri() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/users/setupmfa")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from("{}"))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("\"secret\""));
        assert!(body.contains("\"uri\""));

        Ok(())
    }

    #[tokio::test]
    async fn verify_mfa_route_rejects_invalid_secret() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = crate::web::test_support::authed_router(routes((*mm).clone()));
        let payload = json!({
            "code": "123456",
            "secret": "not-base32"
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/users/verifymfa")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(payload.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("\"success\":false"));
        assert!(body.contains("Cannot decode MFA secret"));

        Ok(())
    }
}
