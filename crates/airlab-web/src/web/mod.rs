mod error;
pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_fallback;
pub mod routes_group;
pub mod routes_json;
pub mod routes_login;
pub mod routes_search;
pub mod routes_static;
pub mod routes_telemetry;
pub mod routes_user;
pub mod routes_validation_file;
pub mod routes_ws;

pub use self::error::ClientError;
pub use self::error::{Error, Result};
use airlab_lib::token::generate_web_token;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<()> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);
}

#[cfg(test)]
pub(crate) mod test_support {
    use crate::web::mw_auth::{CtxExtError, CtxW};
    use airlab_lib::_dev_utils;
    use airlab_lib::ctx::Ctx;
    use axum::Router;
    use axum::body::{Body, to_bytes};
    use axum::http::Request;
    use axum::middleware::{self, Next};
    use axum::response::Response;
    use std::fs;
    use std::sync::Once;

    pub fn init_web_test_env() {
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            _dev_utils::init_test_env();
        });

        let root = std::env::temp_dir().join("airlab-web-test-assets");
        let web_dir = root.join("web");
        let data_dir = root.join("data");
        let _ = fs::create_dir_all(&web_dir);
        let _ = fs::create_dir_all(&data_dir);
        let _ = fs::write(web_dir.join("index.html"), "<html>airlab-test</html>");

        for (key, value) in [
            ("SERVICE_HOST_PORT", "3000"),
            ("SERVICE_HOST_ADDR", "127.0.0.1"),
            ("SERVICE_WEB_FOLDER", web_dir.to_string_lossy().as_ref()),
            ("SERVICE_EMAIL_FROM_ADDRESS", "noreply@example.test"),
            ("SERVICE_EMAIL_FROM_NAME", "Airlab"),
            ("SERVICE_EMAIL_TOKEN", "token"),
            ("SERVICE_EMAIL_ADDRESS", "smtp.example.test"),
            ("SERVICE_LOG_AGGR_URL", "https://logs.example.test"),
            ("SERVICE_RESET_PWD_URL", "https://example.test/reset"),
            ("SERVICE_DATA_PATH", data_dir.to_string_lossy().as_ref()),
            ("SUPER_USER", "admin@example.test"),
            ("SUPER_USER_PWD", "secret"),
            ("SETUP_DEMO_GROUP", "false"),
        ] {
            // SAFETY: test setup controls these process env vars.
            unsafe { std::env::set_var(key, value) };
        }
    }

    pub async fn init_test_db() -> _dev_utils::TestDb {
        init_web_test_env();
        _dev_utils::init_test().await
    }

    pub fn authed_router(router: Router) -> Router {
        router.layer(middleware::from_fn(inject_test_ctx))
    }

    async fn inject_test_ctx(mut req: Request<Body>, next: Next) -> Response {
        if let Ok(ctx) = Ctx::new(1) {
            req.extensions_mut()
                .insert::<core::result::Result<CtxW, CtxExtError>>(Ok(CtxW(ctx)));
        }
        next.run(req).await
    }

    pub async fn response_body_string(
        response: Response,
    ) -> std::result::Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let bytes = to_bytes(response.into_body(), usize::MAX).await?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use serial_test::serial;
    use tower::ServiceExt;
    use tower_cookies::CookieManagerLayer;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    async fn set_cookie_handler(cookies: Cookies) -> Result<&'static str> {
        set_token_cookie(&cookies, "demo1@uzh.ch", Uuid::nil())?;
        Ok("ok")
    }

    async fn remove_cookie_handler(cookies: Cookies) -> &'static str {
        remove_token_cookie(&cookies);
        "ok"
    }

    #[tokio::test]
    #[serial]
    async fn set_token_cookie_sets_auth_cookie() -> TestResult {
        crate::web::test_support::init_web_test_env();

        let app = Router::new()
            .route("/", axum::routing::get(set_cookie_handler))
            .layer(CookieManagerLayer::new());

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;

        let set_cookie = response
            .headers()
            .get(axum::http::header::SET_COOKIE)
            .ok_or_else(|| std::io::Error::other("set-cookie header should be present"))?
            .to_str()?;

        assert!(set_cookie.contains(AUTH_TOKEN));
        assert!(set_cookie.contains("HttpOnly"));
        assert!(set_cookie.contains("Path=/"));
        Ok(())
    }

    #[tokio::test]
    async fn remove_token_cookie_clears_auth_cookie() -> TestResult {
        let app = Router::new()
            .route("/", axum::routing::get(remove_cookie_handler))
            .layer(CookieManagerLayer::new());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header(axum::http::header::COOKIE, format!("{AUTH_TOKEN}=abc"))
                    .body(Body::empty())?,
            )
            .await?;

        let set_cookie = response
            .headers()
            .get(axum::http::header::SET_COOKIE)
            .ok_or_else(|| std::io::Error::other("set-cookie header should be present"))?
            .to_str()?;

        assert!(set_cookie.contains(AUTH_TOKEN));
        assert!(set_cookie.contains("Max-Age=0") || set_cookie.contains("Expires="));
        Ok(())
    }
}
