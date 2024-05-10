use cfg_if::cfg_if;

use data_encoding::BASE64URL_NOPAD;
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

use super::Ctx;
use super::Result as Res;

mod view;
pub use self::view::Lobby;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        mod websocket;
        pub use self::websocket::handler as lobby_handler;
        pub mod ssr;
    }
}

#[must_use]
pub fn from_url_uuid(url_id: &str) -> Uuid {
    let res = Uuid::try_from(
        BASE64URL_NOPAD
            .decode(url_id.as_bytes())
            .unwrap_or_default(),
    );
    res.unwrap_or_else(|_| Uuid::new_v4())
}

fn to_url_uuid(id: Uuid) -> String {
    BASE64URL_NOPAD.encode(id.as_bytes())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
}

impl Player {
    fn new(name: String) -> Self {
        let id = Uuid::new_v4();
        Self { id, name }
    }
}

#[server(AddPlayer, "/api/lobby")]
pub async fn add_player(
    lobby_id: String,
    player_id: String,
    name: String,
) -> Result<(), ServerFnError> {
    use self::ssr::*;
    let lobbys = lobbys()?;
    let lobby_uuid = from_url_uuid(&lobby_id);
    let player_uuid = from_url_uuid(&player_id);

    let lobby = match lobbys.get_lobby(lobby_uuid).await {
        Ok(l) => l,
        Err(_) => lobbys.create().await?,
    };

    let p = lobby.update_player(Player {
        id: player_uuid,
        name,
    })?;

    //Redirect to correct URI
    let redirect = format!("/lobby/{}/{}", to_url_uuid(lobby.id), to_url_uuid(p.id));
    leptos_axum::redirect(&redirect);

    // TODO How to update the other clients?

    Ok(())
}

#[server(GetPlayers, "/api/lobby")]
pub async fn get_players(lobby_id: String) -> Result<Vec<Player>, ServerFnError> {
    use self::ssr::*;
    let lobbys = lobbys()?;
    let lobby_uuid = from_url_uuid(&lobby_id);

    let lobby = match lobbys.get_lobby(lobby_uuid).await {
        Ok(l) => l,
        Err(_) => lobbys.create().await?,
    };

    let players = lobby.players.lock().unwrap().clone();

    Ok(players)
}

#[cfg(test)]
mod tests {
    use super::{from_url_uuid, to_url_uuid};
    use uuid::Uuid;

    #[test]
    fn endode_uuid_matches_decode() {
        let original = Uuid::new_v4();

        let encoded = to_url_uuid(original);
        let decoded = from_url_uuid(&encoded);

        assert_eq!(
            original, decoded,
            "Something wrong with UUID to URL encode and decode"
        );
    }
}
