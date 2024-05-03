//! Returns and errors of the website part

use std::error::Error as StdError;
use std::{fmt::Display, str::FromStr};

use leptos::ServerFnError;

use crate::game::Error as GameError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NotFound,
    Duplicated,
    LoginFail,
    ServerError { e: ServerFnError },
    WebsocketError,
    GameError(GameError),

    // Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequuestExt,
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
        "Internal error"
    }
}

impl From<GameError> for Error {
    fn from(value: GameError) -> Self {
        Self::GameError(value)
    }
}
