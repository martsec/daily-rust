use crate::web::common::Button;
use crate::web::common::ButtonLink;
use data_encoding::BASE64URL_NOPAD;
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;
use leptos_router::*;
use leptos_use::{
    use_websocket, use_websocket_with_options, UseWebSocketOptions, UseWebsocketReturn,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::event;
use tracing::instrument;
use tracing::trace;
use tracing::Level;
use tracing::{debug, info};
use uuid::Uuid;

use super::Ctx;
use super::Result as Res;

fn from_param_uuid(params: Memo<ParamsMap>, param_name: &str) -> (String, Uuid) {
    let raw = params
        .with(|ps| ps.get(param_name).map(std::borrow::ToOwned::to_owned).unwrap_or_default())
        ;

    let uuid = from_url_uuid(&raw);
    let url = to_url_uuid(uuid);
    (url, uuid)
}

#[must_use] pub fn from_url_uuid(url_id: &str) -> Uuid {
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

#[component]
#[must_use] pub fn Lobby() -> impl IntoView {
    let params = use_params_map();

    // FIXME Every time we use this, we call this function
    // and if the user_id is wrong, it returns each time a different
    // random UUID breaking several things...
    //
    // We can't have this as a variable and then do the move
    let id = move || from_param_uuid(params, "id");
    let player_id = move || from_param_uuid(params, "player_id");

    let (players, set_players) = create_signal(vec![]);

    // Websocket
    //
    // FIXME this causes issues if we change the lobby_id after init
    let ws_url = format!("/lobby/{}/ws", id().1);

    // Update signals when new data arrives from the webhook
    let update_signals = move |m: String| {
        info!("received message {}", m);
        if m.starts_with("PLAYERS") {
            let slice = &m[7..];
            let ps: Vec<Player> = serde_json::from_str(slice).unwrap_or_default();
            set_players.set(ps);
        }
    };
    let UseWebsocketReturn { message, send, .. } = use_websocket_with_options(
        &ws_url,
        UseWebSocketOptions::default().on_message(update_signals),
    );

    let (name, set_name) = create_signal("MetaTrust".to_string());

    let new_player = move |_| {
        let p = Player {
            id: player_id().1,
            name: name(),
        };
        let json = serde_json::to_string(&p).unwrap();
        let m = format!("REFRESH_PLAIERS{json}");
        send.clone()(&m);
    };
    view! {
        <div class="my-10 mx-auto flex justify-center">
          <img src="/img/portada.png" alt="portada" class="w-[20rem] max-w-none rounded-xl sm:w-[57rem] md:-ml-4 lg:-ml-0" width="2432" height="1442" />
        </div>
        <div class="my-0 mx-auto max-w-3xl text-center">

            //<h2 class="p-6 text-4xl">"Welcome to PLAI"</h2>
            //<p class="px-10 pb-10">
            //    "✨Become the artificial intelligence monopoly you deserve✨"
            //</p>
            <div class="flex grid grid-cols-2 items-center justify-around">
                <div class="my-2 px-6 lg:px-8">
                    <div class="flex py-2 flex-col justify-center">
                    <label for="name" class="block text-sm font-medium leading-6 text-gray-900">Your Startup Name</label>
                    <input type="text" id="name" name="name" class="rounded-md border-0 py-1.5 pl-7 pr-20 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" placeholder="Try something edgy like MetaTrust" on:input=move |ev| { set_name(event_target_value(&ev));} required/>
                    </div>
                    <div class="my-2">
                    <button type="submit" class="flex w-full justify-center rounded-md bg-emerald-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-emerald-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600" on:click=new_player>
                        "Create my startup"
                    </button>
                    </div>
                </div>

            <div class="my-2 px-6 lg:px-8">
                <PlayerList ps=players />
                <div class="my-2">
                    <Show
                        when=move || { 2 <= players().len() && players().len() <= 6 }
                        fallback=|| view! { "Choose between 1 to 5 players." }
                    >
                        <ButtonLink title="👩🏾‍💼 PLAI 👨🏾‍💼".to_string() href={format!("/plai/{}/{}/", id().0, player_id().0)}/>
                    </Show>
                </div>
            </div>
            </div>
        </div>
    }
}

#[component]
fn PlayerList(ps: ReadSignal<Vec<Player>>) -> impl IntoView {
    view! {
        <div class="py-2">
            <h4 class="p-4 text-xl">Current players</h4>
            <ul role="list" class="divide-y divide-gray-100">
                <For
                    each=move || ps.get().into_iter()
                    key=|p| p.name.clone()
                    let:p
                >
                    <li class="py-2"><p class="text-m leading-6 text-gray-900">{p.name}</p></li>
                </For>
        </ul>
        </div>
    }
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

#[cfg(feature = "ssr")]
pub mod ssr {
    use axum::extract::FromRef;
    use leptos::use_context;
    use leptos::ServerFnError;
    use std::sync::Arc;
    use std::sync::Mutex;
    use tokio::sync::broadcast;
    use uuid::Uuid;

    use super::Player;
    use super::Res;

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
        #[must_use] pub fn new() -> Self {
            Self::from_id(Uuid::new_v4())
        }

        #[must_use] pub fn from_id(id: Uuid) -> Self {
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
}

#[instrument]
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
    info!("Redirecting to {}", &redirect);
    leptos_axum::redirect(&redirect);

    // TODO How to update the other clients?

    Ok(())
}

#[server(GetPlayers, "/api/lobby")]
pub async fn get_players(lobby_id: String) -> Result<Vec<Player>, ServerFnError> {
    debug!("Get players of lobby {lobby_id}");
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
