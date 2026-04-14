#![allow(clippy::module_name_repetitions)]
use crate::web;
use airlab_lib::envs;
use airlab_lib::model;
use airlab_lib::{pwd, token};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use std::sync::Arc;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd {
        user_id: i64,
    },
    LoginFailPwdNotMatching {
        user_id: i64,
    },
    BadRequest(String),
    UnsupportedQueryValue(String),
    #[from]
    CtxExt(web::mw_auth::CtxExtError),
    #[from]
    Env(envs::Error),
    #[from]
    Model(model::Error),
    #[from]
    Pwd(pwd::Error),
    #[from]
    Token(token::Error),
    #[from]
    Io(#[serde_as(as = "DisplayFromStr")] std::io::Error),

    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("INFO_RES - model::Error {self:?}");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(Arc::new(self));
        response
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl Error {
    pub const fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::{
            BadRequest, CtxExt, LoginFailPwdNotMatching, LoginFailUserHasNoPwd,
            LoginFailUsernameNotFound, Model,
        };

        #[allow(unreachable_patterns)]
        match self {
            LoginFailUsernameNotFound
            | LoginFailUserHasNoPwd { .. }
            | LoginFailPwdNotMatching { .. } => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            Model(model::Error::EntityNotFound { entity, id }) => (
                StatusCode::BAD_REQUEST,
                ClientError::ENTITY_NOT_FOUND { entity, id: *id },
            ),

            BadRequest(_) => (StatusCode::BAD_REQUEST, ClientError::SERVICE_ERROR),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },

    SERVICE_ERROR,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_not_found_maps_to_bad_request() {
        let error = Error::from(model::Error::EntityNotFound {
            entity: "provider",
            id: 7,
        });

        let (status, client_error) = error.client_status_and_error();

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(matches!(
            client_error,
            ClientError::ENTITY_NOT_FOUND {
                entity: "provider",
                id: 7
            }
        ));
    }

    #[test]
    fn login_fail_maps_to_forbidden() {
        let (status, client_error) = Error::LoginFailUsernameNotFound.client_status_and_error();

        assert_eq!(status, StatusCode::FORBIDDEN);
        assert!(matches!(client_error, ClientError::LOGIN_FAIL));
    }

    #[test]
    fn into_response_stashes_error_extension() {
        let response = Error::LoginFailUsernameNotFound.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert!(response.extensions().get::<Arc<Error>>().is_some());
    }
}
