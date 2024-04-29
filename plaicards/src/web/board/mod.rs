use cfg_if::cfg_if;

mod view;

pub use self::view::Board;

cfg_if! {
    if #[cfg(feature = "ssr")] {

        mod ssr;
        mod websocket;

        pub use self::websocket::handler as board_handler;
        pub use self::ssr::GameController;
    }
}

/// Contract for the websocket messages between front and back
pub mod msg {

    #[cfg(feature = "ssr")]
    use crate::game::Player as GPlayer;

    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    use uuid::Uuid;

    pub trait WsSerDe<'a>: Serialize + Deserialize<'a> {
        fn to_str(&self) -> String {
            serde_json::to_string(&self).unwrap()
        }

        #[must_use] fn from_str(msg: &'a str) -> Self {
            serde_json::from_str(msg).expect("Malformed Message")
        }
    }
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct Player {
        pub id: Uuid,
        pub name: String,
    }

    #[cfg(feature = "ssr")]
    impl From<&GPlayer> for Player {
        fn from(p: &GPlayer) -> Self {
            Self {
                id: Uuid::new_v4(),
                name: p.name().clone(),
            }
        }
    }

    /// Encapsulates all messages the server will send
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    #[serde(tag = "t", content = "c")]
    pub enum ServerMsg {
        Hello,
        BadRequest,
        Players(Vec<Player>),
        RivalHand { id: Uuid, num_cards: usize },
        AddCard(Card),
    }
    impl<'a> WsSerDe<'a> for ServerMsg {}

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct Card {
        #[serde(rename = "t")]
        pub title: String,
        #[serde(rename = "e")]
        pub effect: String,
        #[serde(rename = "d")]
        pub description: String,
    }

    /// Encapsulates all messages the client will send
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    #[serde(tag = "t", content = "c")]
    pub enum ClientMsg {
        Connect { game_id: Uuid, player_id: Uuid },
    }

    impl<'a> WsSerDe<'a> for ClientMsg {}
}
