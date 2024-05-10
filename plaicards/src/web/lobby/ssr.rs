use axum::extract::FromRef;
use leptos::use_context;
use leptos::ServerFnError;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use uuid::Uuid;

use super::Ctx;
use super::Player;
use crate::web::Result as Res;

pub fn lobbys() -> Result<LobbyController, ServerFnError> {
    use_context::<LobbyController>()
        .ok_or_else(|| ServerFnError::ServerError("Database missing".into()))
}

#[derive(Clone, Debug)]
pub struct Lobby {
    pub id: Uuid,
    pub players: Arc<Mutex<Vec<Player>>>,
    // Channel to send messages to all connected clients
    pub tx: broadcast::Sender<String>,
}

impl Default for Lobby {
    fn default() -> Self {
        Self::new()
    }
}

impl Lobby {
    #[must_use]
    pub fn new() -> Self {
        Self::from_id(Uuid::new_v4())
    }

    #[must_use]
    pub fn from_id(id: Uuid) -> Self {
        let (tx, _rx) = broadcast::channel(10);
        Self {
            id,
            players: Arc::default(),
            tx,
        }
    }
}

impl Lobby {
    pub fn update_player(&self, player: Player) -> Res<Player> {
        let mut store = self.players.lock().unwrap();
        match store.iter_mut().find(|p| player.id == p.id) {
            Some(p) => p.name = player.name.clone(),
            None => store.push(player.clone()),
        };

        Ok(player)
    }
}

#[derive(FromRef, Clone, Debug)]
pub struct LobbyController {
    lobby_store: Arc<Mutex<Vec<Lobby>>>,
}

impl LobbyController {
    pub async fn new() -> Self {
        Self {
            lobby_store: Arc::default(),
        }
    }
}

impl LobbyController {
    pub async fn create(&self) -> Res<Lobby> {
        let lobby = Lobby::new();

        let mut store = self.lobby_store.lock().unwrap();

        store.push(lobby.clone());

        Ok(lobby)
    }

    /// Returns a lobby given an ID
    ///
    /// If the lobby does not exists, it creates a new one.
    pub async fn get_lobby(&self, lobby_id: Uuid) -> Res<Lobby> {
        let mut store = self.lobby_store.lock().unwrap();

        match store.iter().find(|l| l.id == lobby_id) {
            Some(l) => Ok(l.clone()),
            None => {
                let lobby = Lobby::from_id(lobby_id);
                store.push(lobby.clone());
                Ok(lobby)
            }
        }
    }
}
