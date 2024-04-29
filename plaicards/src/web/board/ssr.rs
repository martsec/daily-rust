use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, event, info, instrument, trace};
use uuid::Uuid;

use axum::extract::FromRef;
use leptos::use_context;
use leptos::ServerFnError;
use tokio::sync::broadcast;

use crate::game::Game;
use crate::web::{Error, Result as Res};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
}

/// Middleware to store the websocket room
#[derive(Clone, Debug)]
pub struct GameRoom {
    pub id: Uuid,
    pub game: Arc<RwLock<Game>>,
    pub tx: broadcast::Sender<String>,
}

impl GameRoom {
    pub fn new(id: Uuid, players: &[(Uuid, String)]) -> Self {
        let (tx, _rx) = broadcast::channel(5);

        let game = Game::new(players);
        Self {
            id,
            tx,
            game: Arc::new(RwLock::new(game)),
        }
    }
}

/// Database
#[derive(Clone, Debug)]
pub struct GameController {
    store: Arc<RwLock<HashMap<Uuid, GameRoom>>>,
}

impl GameController {
    pub async fn new() -> Self {
        let mut gc = Self {
            store: Arc::default(),
        };

        // FIXME just for dev purposes
        let gr = GameRoom::new(
            Uuid::from_str(&"9cb14765-bbfd-447a-b29e-bb203801acb6").unwrap(),
            &[
                (Uuid::new_v4(), "p1".to_string()),
                (Uuid::new_v4(), "p2".to_string()),
                (Uuid::new_v4(), "p3".to_string()),
                (Uuid::new_v4(), "p4".to_string()),
                (Uuid::new_v4(), "p5".to_string()),
            ],
        );
        gc.put(gr).await.unwrap();

        gc
    }
}

impl GameController {
    pub async fn put(&mut self, gr: GameRoom) -> Res<()> {
        let mut store = self.store.write().await;
        if store.contains_key(&gr.id) {
            Err(Error::Duplicated)
        } else {
            store.insert(gr.id.clone(), gr);
            Ok(())
        }
    }

    pub async fn get(&self, id: &Uuid) -> Res<GameRoom> {
        let store = self.store.read().await;

        let game = store.get(id).ok_or(Error::NotFound)?;
        Ok(game.clone())
    }
}

#[cfg(test)]
mod test {
    use rstest::{fixture, rstest};
    use uuid::Uuid;

    use super::{GameController, GameRoom};
    use crate::web::Error;

    #[fixture]
    fn game_room() -> GameRoom {
        let ps = vec![
            (Uuid::new_v4(), "p1".to_string()),
            (Uuid::new_v4(), "p2".to_string()),
            (Uuid::new_v4(), "p3".to_string()),
            (Uuid::new_v4(), "p4".to_string()),
            (Uuid::new_v4(), "p5".to_string()),
        ];
        GameRoom::new(Uuid::new_v4(), &ps)
    }

    #[rstest]
    async fn controller_multiple_reads(game_room: GameRoom) {
        let mut gc = GameController::new().await;
        let _ = gc.put(game_room.clone()).await;

        {
            let retrieved = gc.get(&game_room.id).await;
            let no_exist_id = Uuid::new_v4();
            let non_exist = gc.get(&no_exist_id).await;

            assert!(retrieved.is_ok());
            assert_eq!(non_exist.err(), Some(Error::NotFound));
        }
    }
}