pub mod board;
mod common;
mod errors;
pub mod landing;
pub mod lobby;

/// Context to pass variables around
#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

// Constructor
impl Ctx {
    #[must_use]
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Accessor
impl Ctx {
    #[must_use]
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

pub use errors::{Error, Result};

#[cfg(feature = "ssr")]
pub mod ssr {

    use crate::web::lobby::ssr::LobbyController;
    use axum::extract::FromRef;
    use leptos::LeptosOptions;
    use leptos_router::RouteListing;

    use super::board::GameController;

    /// This takes advantage of Axum's `SubStates` feature by deriving `FromRef`. This is the only way to have more than one
    /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub lobby: LobbyController,
        pub gc: GameController,
        pub routes: Vec<RouteListing>,
    }
}
