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
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

use super::Ctx;
use super::Result as Res;

fn from_param_uuid(params: Memo<ParamsMap>, param_name: &str) -> (String, Uuid) {
    let raw = params
        .with(|ps| ps.get(param_name).map(|s| s.to_owned()).unwrap_or_default())
        .to_owned();

    let uuid = from_url_uuid(&raw);
    let url = to_url_uuid(uuid.clone());
    println!(">> {:<18}, {param_name:<9} {url:?} {uuid:?}", "In frontend");
    (url, uuid)
}

pub fn from_url_uuid(url_id: &str) -> Uuid {
    println!(">> {:<18}", "STARTING DECODING FROM URL");
    println!(">> {:<18}, {url_id}", "original");
    let res = Uuid::try_from(
        BASE64URL_NOPAD
            .decode(url_id.as_bytes())
            .unwrap_or_default(),
    );
    println!(">> {:<18}, {res:?}", "decoded");
    res.unwrap_or_else(|_| Uuid::new_v4())
}

fn to_url_uuid(id: Uuid) -> String {
    BASE64URL_NOPAD.encode(id.as_bytes())
}

#[component]
pub fn Lobby() -> impl IntoView {
    let params = use_params_map();

    // FIXME Every time we use this, we call this function
    // and if the user_id is wrong, it returns each time a different
    // random UUID breaking several things...
    //
    // We can't have this as a variable and then do the move
    let id = move || from_param_uuid(params, "id");
    let player_id = move || from_param_uuid(params, "player_id");

    let add_p = create_server_action::<AddPlayer>();

    // List of players loaded from the server
    let ps = create_resource(move || add_p.version().get(), move |_| get_players(id().0));

    // Websocket
    //
    // FIXME this causes issues if we change the lobby_id after init
    let ws_url = format!("/lobby/{}/ws", id().1);
    let UseWebsocketReturn { message, send, .. } = use_websocket(&ws_url);

    let send_refresh = move |_| {
        send.clone()("REFRESH_PLAIERS");
    };

    view! {
            <div class="my-0 mx-auto max-w-3xl text-center">

                <h2 class="p-6 text-4xl">"Welcome to PLAI"</h2>
                <p class="px-10 pb-10">
                    "✨Become the artificial intelligence monopoly you deserve✨"
                </p>

                <ActionForm action=add_p>
                <label for="price" class="block text-sm font-medium leading-6 text-gray-900">Name</label>
                    <input type="hidden" name="lobby_id" prop:value=move || id().0/>
                    <input type="hidden" name="player_id" prop:value=move || player_id().0/>
                    <input type="text" name="name" class=" rounded-md border-0 py-1.5 pl-7 pr-20 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" placeholder="Choose your name" required/>
                    <Button title="plai!".into() on:click=send_refresh/>
                </ActionForm>


                <p>You are in lobby {move || id().0} and are player id {move || player_id().0}</p>


                <div>
                    <Transition fallback=move || view! { <p>"Waiting for pl<b>ai</b>ers..."</p> }>
                    {move || {


    let p = move || {
        ps.get()
            .map(move |ps| match ps {
                Err(_) => None,
                Ok(ps) => Some(ps),
            })
            .flatten()
            .unwrap_or_default()
    };

    let player_list = move || {
        p().into_iter()
            .map(move |p| {
                view! {
                    <li class="py-2">
                     {p.name}
                    </li>
                }
            })
            .collect_view()
    };


    let game_uri = format!("/plai/{}/{}/", id().0, player_id().0);

    view!{<ul class="my-4 list-disc">{player_list}</ul>

                <div>

                    <div class="my-4">
                        <Show
                            when=move || { 2 <= p().len() && p().len() <= 6 }
                            fallback=|| view! { "Choose between 1 to 6 players." }
                        >
                            <ButtonLink title="👩🏾‍💼 Start Game 👨🏾‍💼".to_string() href=game_uri.clone()/>
                        </Show>
                    </div>
                </div>
    }

                    }
                    }

                    </Transition>
                </div>


                <div>
    <h4> Web socket updates </h4>

                    <div>

        {move ||
            match message.get() {
                None => view!{"It's just you"}.into_view(),
                Some(m) => into_player_view(&m),
            }
        }


    </div>


                </div>
            </div>
        }
}

/// Convert a JSON with the list of players to the view to render
fn into_player_view(s: &str) -> View {
    let ps: Vec<Player> = serde_json::from_str(s).unwrap_or_default();

    let player_list = ps
        .into_iter()
        .map(move |p| {
            view! {
                <li class="py-2">
                 {p.name}
                </li>
            }
        })
        .collect_view();

    view! {<ul class="my-4 list-disc">{player_list}</ul>}.into_view()
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

#[derive(Clone, Debug)]
pub struct Lobby {
    id: Uuid,
    pub players: Arc<Mutex<Vec<Player>>>,
}

impl Lobby {
    pub fn new() -> Self {
        Self::from_id(Uuid::new_v4())
    }

    pub fn from_id(id: Uuid) -> Self {
        Self {
            id,
            players: Arc::default(),
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

#[cfg(feature = "ssr")]
pub mod ssr {
    use axum::extract::FromRef;
    use leptos::use_context;
    use leptos::ServerFnError;
    use std::sync::Arc;
    use std::sync::Mutex;
    use uuid::Uuid;

    use super::Lobby;
    use super::Res;

    pub fn lobbys() -> Result<LobbyController, ServerFnError> {
        use_context::<LobbyController>()
            .ok_or_else(|| ServerFnError::ServerError("Database missing".into()))
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

#[server(AddPlayer, "/api/lobby")]
pub async fn add_player(
    lobby_id: String,
    player_id: String,
    name: String,
) -> Result<(), ServerFnError> {
    println!(">> Adding new player");
    use self::ssr::*;
    let lobbys = lobbys()?;
    let lobby_uuid = from_url_uuid(&lobby_id);
    let player_uuid = from_url_uuid(&player_id);

    let lobby = match lobbys.get_lobby(lobby_uuid).await {
        Ok(l) => l,
        Err(_) => lobbys.create().await?,
    };
    dbg!(&lobby);

    let p = lobby.update_player(Player {
        id: player_uuid,
        name,
    })?;

    //Redirect to correct URI
    dbg!(&lobby_id);
    dbg!(&lobby_uuid);
    dbg!(to_url_uuid(lobby.id));
    let redirect = format!("/lobby/{}/{}", to_url_uuid(lobby.id), to_url_uuid(p.id));
    leptos_axum::redirect(&redirect);

    // TODO How to update the other clients?

    Ok(())
}

#[server(GetPlayers, "/api/lobby")]
pub async fn get_players(lobby_id: String) -> Result<Vec<Player>, ServerFnError> {
    println!("Get players of lobby {lobby_id}");
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
