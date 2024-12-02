use crate::web::{self, remove_token_cookie, Error, Result};
use crate::web_config;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::Json;
use axum::{response::Html, Router};
use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lib_auth::pwd::{self, ContentToHash};
use lib_core::ctx::Ctx;
use lib_core::model::user::{UserBmc, UserForCreate, UserForLogin, UserForUpdate};
use lib_core::model::ModelManager;
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs;
use tokio::task;
use tower_cookies::Cookies;
use tracing::{debug, warn};
use uuid::Uuid;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/login", post(api_login_handler))
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
        .route("/api/v1/auth/check/:email", get(api_login_check_handler))
        .route(
            "/api/v1/auth/password-recovery/:email",
            post(api_recover_pwd_handler),
        )
        .route("/api/v1/logoff", post(api_logoff_handler))
        .with_state(mm)
}

async fn api_reset_pwd_form_handler(State(_mm): State<ModelManager>) -> Result<Html<String>> {
    debug!("HANDLER - api_get_main_handler");
    #[allow(clippy::expect_used)] // FIXME
    let html_content = task::spawn_blocking(|| {
        fs::read_to_string(format!("{}/index.html", &web_config().WEB_FOLDER))
    })
    .await
    .expect("Task panicked")?;

    Ok(Html(html_content))
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

    web::set_token_cookie(&cookies, &user.username, user.token_salt)?;

    let _body = Json(json!({
        "result": {
            "success": true
        }
    }));
    let body = Json(json!({
        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOjMyNSwiaXNBZG1pbiI6dHJ1ZSwiaWF0IjoxNjc3NzcyODE1LCJleHAiOjE2NzgzNzc2MTUsImlzcyI6IkFpckxhYiIsInN1YiI6IjMyNSJ9.J_5677bwyHF9xKZvPb3IPRd_1f3tu3U2wlMCmmWqNvY"
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
        email: None,
        name: None,
        reset_token: Some(uuid.into()),
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

    let from = match format!(
        "{} <{}>",
        &web_config().EMAIL_FROM_NAME,
        &web_config().EMAIL_FROM_ADDRESS
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
                &web_config().RESET_PWD_URL,
                reset_token
            ),
            /*
            format!(
                "token: {}: <a href='{}'>Reset password</a>",
                reset_token,
                &web_config().RESET_PWD_URL
            ),
            */
        )) {
        Ok(o) => o,
        Err(e) => {
            panic!("Cannot create email: {e}");
        }
    };

    let creds = Credentials::new(
        web_config().EMAIL_FROM_ADDRESS.clone(),
        web_config().EMAIL_TOKEN.clone(),
    );

    let result = match SmtpTransport::relay(&web_config().EMAIL_ADDRESS) {
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
            email: None,
            name: None,
            reset_token: Some(String::new()),
        };
        UserBmc::update(&root_ctx, &mm, user_id, u2u).await?;
    }
    // Create the success body.
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
        &web_config().EMAIL_FROM_NAME,
        &web_config().EMAIL_FROM_ADDRESS
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
        web_config().EMAIL_FROM_ADDRESS.clone(),
        web_config().EMAIL_TOKEN.clone(),
    );

    let result = match SmtpTransport::relay(&web_config().EMAIL_ADDRESS) {
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
