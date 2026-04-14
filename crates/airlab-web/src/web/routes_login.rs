use crate::web::{self, Error, Result, remove_token_cookie};
use crate::web_config;
use airlab_lib::ctx::Ctx;
use airlab_lib::model::ModelManager;
use airlab_lib::model::user::{UserBmc, UserForCreate, UserForLogin, UserForUpdate};
use airlab_lib::pwd::{self, ContentToHash};
use airlab_lib::token::generate_web_token;
use axum::Json;
use axum::extract::{Json as eJson, Path, State};
use axum::routing::{get, post};
use axum::{Router, response::Html};
use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::io;
use tokio::task;
use tower_cookies::Cookies;
use tracing::{debug, warn};
use uuid::Uuid;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/login", post(api_login_handler))
        .route("/api/v1/auth/login3", post(api_login_handler_v3))
        .route("/api/v1/auth/login", post(api_login_handler_v2))
        .route("/api/v1/auth/signup", post(api_signup_handler))
        .route(
            "/api/v1/auth/reset-password",
            post(api_reset_password_handler),
        )
        .route(
            "/api/v1/auth/reset-password/",
            post(api_reset_password_handler),
        )
        .route("/reset-password", get(api_reset_pwd_form_handler))
        .route("/api/v1/auth/check/{email}", get(api_login_check_handler))
        .route(
            "/api/v1/auth/password-recovery/{email}",
            post(api_recover_pwd_handler),
        )
        .route("/api/v1/logoff", post(api_logoff_handler))
        .with_state(mm)
}

async fn api_reset_pwd_form_handler(State(_mm): State<ModelManager>) -> Result<Html<String>> {
    debug!("HANDLER - api_get_main_handler");
    let web_folder = web_config()?.WEB_FOLDER.clone();
    let html_content =
        task::spawn_blocking(move || fs::read_to_string(format!("{}/index.html", web_folder)))
            .await
            .map_err(|err| io::Error::other(format!("Task panicked: {err}")))??;

    Ok(Html(html_content))
}

#[derive(Deserialize, Clone, Debug)]
pub struct LoginContent {
    username: String,
    password: String,
}

async fn api_login_handler_v3(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    eJson(payload): eJson<LoginContent>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_login_handler");

    let LoginContent { username, password } = payload;
    let pwd_clear = password.clone();

    let pwd_clear = pwd_clear.replace("%21", "!");
    let root_ctx = Ctx::root_ctx();

    let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;

    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };

    pwd::validate_pwd(
        &ContentToHash {
            salt: user.pwd_salt,
            content: pwd_clear.clone(),
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    let token = generate_web_token(&user.username, user.token_salt)?;
    web::set_token_cookie(&cookies, &user.username, user.token_salt)?;
    let body = Json(json!({
        "token": token.to_string()
    }));

    Ok(body)
}

async fn api_login_handler_v2(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    payload: String,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_login_handler");

    let username = payload
        .split('&')
        .collect::<Vec<&str>>()
        .first()
        .map_or("", |a| {
            a.split('=').collect::<Vec<&str>>().last().map_or("", |b| b)
        });
    let username = username.replace("%40", "@");

    let pwd_clear = payload
        .split('&')
        .collect::<Vec<&str>>()
        .last()
        .map_or_else(String::new, |a| {
            a.split('=')
                .collect::<Vec<&str>>()
                .last()
                .map_or_else(String::new, |b| (*b).to_string())
        });
    let pwd_clear = pwd_clear.replace("%21", "!");
    let root_ctx = Ctx::root_ctx();

    let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;

    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };

    pwd::validate_pwd(
        &ContentToHash {
            salt: user.pwd_salt,
            content: pwd_clear.clone(),
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    let token = generate_web_token(&user.username, user.token_salt)?;
    web::set_token_cookie(&cookies, &user.username, user.token_salt)?;
    let body = Json(json!({
        "token": token.to_string(),
        "mfaRequired": user.mfa_enabled
    }));

    Ok(body)
}

async fn api_recover_pwd_handler(
    State(mm): State<ModelManager>,
    _cookies: Cookies,
    Path(username): Path<String>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_recover_pwd_handler");
    let uuid = Uuid::new_v4();
    let u2u = UserForUpdate {
        reset_token: Some(uuid.into()),
        ..Default::default()
    };
    let root_ctx = Ctx::root_ctx();
    let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;

    let reset_token = u2u
        .reset_token
        .as_ref()
        .map_or_else(String::new, std::clone::Clone::clone);

    UserBmc::update(&root_ctx, &mm, user.id, u2u).await?;

    let fail_body = Json(json!({
        "result": {
            "success":false
        }
    }));
    let config = web_config()?;

    let from = match format!(
        "{} <{}>",
        &config.EMAIL_FROM_NAME, &config.EMAIL_FROM_ADDRESS
    )
    .parse()
    {
        Ok(o) => o,
        Err(e) => {
            warn!("Cannot create the from field: {e:?}");
            return Ok(fail_body);
        }
    };
    let to = match username.to_string().parse() {
        Ok(o) => o,
        Err(e) => {
            warn!("Cannot create the to field: {e:?}");
            return Ok(fail_body);
        }
    };
    let email = match Message::builder()
        .from(from)
        .to(to)
        .subject("AirLab reset password request")
        .multipart(MultiPart::alternative_plain_html(
            String::from("broken link"),
            format!(
                "<a href='{}?token={}'>Reset password</a>",
                &config.RESET_PWD_URL, reset_token
            ),
        )) {
        Ok(o) => o,
        Err(e) => {
            warn!("Cannot create email: {e}");
            return Ok(fail_body);
        }
    };

    let creds = Credentials::new(
        config.EMAIL_FROM_ADDRESS.clone(),
        config.EMAIL_TOKEN.clone(),
    );

    let result = match SmtpTransport::relay(&config.EMAIL_ADDRESS) {
        Ok(o) => {
            let mailer = o.credentials(creds).build();
            match mailer.send(&email) {
                Ok(_) => {
                    warn!("Email sent successfully!");
                    true
                }
                Err(e) => {
                    warn!("Could not send email: {e:?}");
                    false
                }
            }
        }
        Err(e) => {
            warn!("Cannot send email: {e:?}");
            false
        }
    };

    let body = Json(json!({
        "result": {
            "success": result
        }
    }));

    Ok(body)
}

async fn api_login_check_handler(
    State(mm): State<ModelManager>,
    _cookies: Cookies,
    Path(username): Path<String>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_login_check_handler");
    let root_ctx = Ctx::root_ctx();
    let body = match UserBmc::first_by_username::<UserForLogin>(&root_ctx, &mm, &username).await? {
        Some(_) => Json(json!({
                "exists": true
        })),
        None => Json(json!({
                "exists": false
        })),
    };
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct ResetPayload {
    #[serde(rename = "newPassword")]
    new_password: String,
    token: String,
}

async fn api_reset_password_handler(
    State(mm): State<ModelManager>,
    _cookies: Cookies,
    Json(payload): Json<ResetPayload>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_reset_password_handler");
    let ResetPayload {
        new_password,
        token,
    } = payload;

    if !token.is_empty() {
        let root_ctx = Ctx::root_ctx();
        let user: UserForLogin = UserBmc::first_by_token(&root_ctx, &mm, &token)
            .await?
            .ok_or(Error::LoginFailUsernameNotFound)?;
        let user_id = user.id;
        warn!("Resetting password for user {}; token: {}", user_id, token);

        UserBmc::update_pwd(&root_ctx, &mm, user_id, &new_password).await?;

        let u2u = UserForUpdate {
            reset_token: Some(String::new()),
            ..Default::default()
        };
        UserBmc::update(&root_ctx, &mm, user_id, u2u).await?;
    }
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct SignupPayload {
    email: String,
    name: String,
    password: String,
}

async fn api_signup_handler(
    State(mm): State<ModelManager>,
    _cookies: Cookies,
    Json(payload): Json<SignupPayload>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_signup_handler");
    let config = web_config()?;

    let SignupPayload {
        email,
        name,
        password,
    } = payload;
    let root_ctx = Ctx::root_ctx();
    let email = email.to_lowercase();
    let ufc = UserForCreate {
        username: Some(email.clone()),
        email: email.clone(),
        name: Some(name.clone()),
        pwd_clear: None,
    };
    let user_id = UserBmc::create(&root_ctx, &mm, ufc).await?;
    UserBmc::update_pwd(&root_ctx, &mm, user_id, &password).await?;

    let fail_body = Json(json!({
        "result": {
            "success": false
        }
    }));

    let from = match format!(
        "{} <{}>",
        &config.EMAIL_FROM_NAME, &config.EMAIL_FROM_ADDRESS
    )
    .parse()
    {
        Ok(o) => o,
        Err(e) => {
            warn!("Cannot create from field: {e:?}");
            return Ok(fail_body);
        }
    };
    let to = match format!("{name} <{email}>").parse() {
        Ok(o) => o,
        Err(e) => {
            warn!("Cannot create to field: {e:?}");
            return Ok(fail_body);
        }
    };

    let email = match Message::builder()
        .from(from)
        .to(to)
        .subject("AirLab sign-up confirmation")
        .body(String::from("Thanks for signing up with AirLab"))
    {
        Ok(o) => o,
        Err(e) => {
            warn!("Cannot create email: {e:?}");
            return Ok(fail_body);
        }
    };

    let creds = Credentials::new(
        config.EMAIL_FROM_ADDRESS.clone(),
        config.EMAIL_TOKEN.clone(),
    );

    let result = match SmtpTransport::relay(&config.EMAIL_ADDRESS) {
        Ok(o) => {
            let mailer = o.credentials(creds).build();

            match mailer.send(&email) {
                Ok(_) => {
                    warn!("Email sent successfully!");
                    true
                }
                Err(e) => {
                    warn!("Could not send email: {e:?}");
                    false
                }
            }
        }
        Err(e) => {
            warn!("Could not send email: {e:?}");
            false
        }
    };

    let body = Json(json!({
        "result": {
            "success": result
        }
    }));

    Ok(body)
}

async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_login_handler");

    let LoginPayload {
        username,
        pwd: pwd_clear,
    } = payload;
    let root_ctx = Ctx::root_ctx();
    warn!("Username: {username}");

    let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;

    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };

    pwd::validate_pwd(
        &ContentToHash {
            salt: user.pwd_salt,
            content: pwd_clear.clone(),
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    web::set_token_cookie(&cookies, &user.username, user.token_salt)?;

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_logoff_handler(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
    debug!("HANDLER - api_logoff_handler");
    let should_logoff = payload.logoff;

    if should_logoff {
        remove_token_cookie(&cookies);
    }

    let body = Json(json!({
        "result": {
            "logged_off": should_logoff
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
    logoff: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use airlab_lib::ctx::Ctx;
    use airlab_lib::model::user::UserBmc;
    use tower::ServiceExt;
    use tower_cookies::CookieManagerLayer;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn login_check_route_returns_seeded_user() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = routes((*mm).clone()).layer(CookieManagerLayer::new());

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/auth/check/demo1%40uzh.ch")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("\"exists\":true"));

        Ok(())
    }

    #[tokio::test]
    async fn reset_password_form_serves_index_html() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = routes((*mm).clone()).layer(CookieManagerLayer::new());

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/reset-password")
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

    #[tokio::test]
    async fn login_check_route_returns_false_for_unknown_user() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = routes((*mm).clone()).layer(CookieManagerLayer::new());

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/auth/check/missing%40example.test")
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("\"exists\":false"));

        Ok(())
    }

    #[tokio::test]
    async fn login_route_sets_auth_cookie_for_valid_credentials() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let ctx = Ctx::root_ctx();
        UserBmc::update_pwd(&ctx, &mm, 1, "secret123").await?;
        let app = routes((*mm).clone()).layer(CookieManagerLayer::new());
        let payload = json!({
            "username": "demo1@uzh.ch",
            "pwd": "secret123"
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/login")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(payload.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let set_cookie = response
            .headers()
            .get(axum::http::header::SET_COOKIE)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| std::io::Error::other("login should set cookie"))?;
        assert!(set_cookie.contains(crate::web::AUTH_TOKEN));

        Ok(())
    }

    #[tokio::test]
    async fn logoff_route_reports_logged_off() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = routes((*mm).clone()).layer(CookieManagerLayer::new());
        let payload = json!({ "logoff": true });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/logoff")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .header(axum::http::header::COOKIE, "auth-token=abc")
                    .body(axum::body::Body::from(payload.to_string()))?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = crate::web::test_support::response_body_string(response).await?;
        assert!(body.contains("\"logged_off\":true"));

        Ok(())
    }
}
