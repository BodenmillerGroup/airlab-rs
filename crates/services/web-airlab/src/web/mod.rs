mod error;
pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_clone;
pub mod routes_conjugate;
pub mod routes_fallback;
pub mod routes_group;
pub mod routes_login;
pub mod routes_lot;
pub mod routes_member;
pub mod routes_panel;
pub mod routes_panel_element;
pub mod routes_protein;
pub mod routes_provider;
pub mod routes_species;
pub mod routes_static;
pub mod routes_tag;
pub mod routes_user;
pub mod routes_validation;
pub mod routes_validation_file;
pub mod routes_ws;

pub use self::error::ClientError;
pub use self::error::{Error, Result};
use lib_auth::token::generate_web_token;
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
