use axum::{
    http::{response, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use strum_macros;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequuestExt,

    // -- Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:<12} {self:?}", "INTO_RES");

        // Placeholder axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        // Insert the error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    #[allow(unreachable_patterns)] // SInce we might code everything and the fallback is not used
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequuestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Model
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
