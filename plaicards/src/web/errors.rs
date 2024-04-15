use std::error::Error as StdError;
use std::{fmt::Display, str::FromStr};

use leptos::ServerFnError;

/// Utilities for returns and errors

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    NotFound,
    LoginFail,
    ServerError { e: ServerFnError },

    // Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequuestExt,

    // -- Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}

impl FromStr for Error {
    type Err = ServerFnError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Err(ServerFnError::ServerError(s.into()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal error")
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "Internal error".into()
    }
}
