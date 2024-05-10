//! Game errors

use std::error::Error as StdError;
use std::fmt::Display;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    EmptyDeck,
    GameEnded,
    NotYourTurn,
    RuleBreak,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game error {self:?}")
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "Internal error"
    }
}
