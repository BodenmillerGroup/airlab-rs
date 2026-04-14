use crate::web::{AUTH_TOKEN, set_token_cookie};
use crate::web::{Error, Result};
use airlab_lib::ctx::Ctx;
use airlab_lib::model::ModelManager;
use airlab_lib::model::user::{UserBmc, UserForAuth};
use airlab_lib::token::{Token, validate_web_token};
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub async fn mw_ctx_resolve(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("MIDDLEWARE - mw_ctx_resolve");

    let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;

    let user: UserForAuth = UserBmc::first_by_username(&Ctx::root_ctx(), &mm, &token.ident)
        .await
        .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
        .ok_or(CtxExtError::UserNotFound)?;

    validate_web_token(&token, user.token_salt).map_err(|_| CtxExtError::FailValidate)?;

    set_token_cookie(cookies, &user.username, user.token_salt)
        .map_err(|_| CtxExtError::CannotSetTokenCookie)?;

    Ctx::new(user.id)
        .map(CtxW)
        .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("EXTRACTOR - Ctx");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenWrongFormat,
    UserNotFound,
    ModelAccessError(String),
    FailValidate,
    CannotSetTokenCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
}

impl core::fmt::Display for CtxExtError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CtxExtError {}

#[cfg(test)]
mod tests {
    use super::*;
    use airlab_lib::token::generate_web_token;
    use axum::Router;
    use tower::ServiceExt;
    use tower_cookies::CookieManagerLayer;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn ctx_resolve_returns_missing_cookie_error() {
        let mm = crate::web::test_support::init_test_db().await;
        let cookies = Cookies::default();

        let result = _ctx_resolve(State((*mm).clone()), &cookies).await;

        assert!(matches!(result, Err(CtxExtError::TokenNotInCookie)));
    }

    #[tokio::test]
    async fn ctx_resolve_accepts_valid_cookie() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let cookies = Cookies::default();
        let token = generate_web_token(
            "demo1@uzh.ch",
            uuid::Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")?,
        )?;
        cookies.add(Cookie::new(AUTH_TOKEN, token.to_string()));

        let result = _ctx_resolve(State((*mm).clone()), &cookies)
            .await
            .map_err(|err| std::io::Error::other(err.to_string()))?;

        assert_eq!(result.0.user_id(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn middleware_allows_request_with_invalid_cookie() -> TestResult {
        let mm = crate::web::test_support::init_test_db().await;
        let app = Router::new()
            .route("/", axum::routing::get(|| async { "ok" }))
            .layer(axum::middleware::from_fn_with_state(
                (*mm).clone(),
                mw_ctx_resolve,
            ))
            .layer(CookieManagerLayer::new());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header(axum::http::header::COOKIE, format!("{AUTH_TOKEN}=invalid"))
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        Ok(())
    }
}
