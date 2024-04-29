use std::rc::Rc;

use crate::web::common::Button;
use crate::web::common::ButtonLink;
use data_encoding::BASE64URL_NOPAD;
use leptos::ev::Event;
use leptos::logging::log;
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;
use leptos_router::*;
use leptos_use::{
    use_websocket, use_websocket_with_options, UseWebSocketOptions, UseWebsocketReturn,
};
use tracing::info;
use uuid::Uuid;

use super::msg;
use super::msg::{ClientMsg, ServerMsg, WsSerDe};

fn from_param_uuid(params: Memo<ParamsMap>, param_name: &str) -> Uuid {
    let raw = params
        .with(|ps| ps.get(param_name).map(std::borrow::ToOwned::to_owned).unwrap_or_default())
        ;

    let uuid = from_url_uuid(&raw);
    let url = to_url_uuid(uuid);
    uuid
}

fn from_url_uuid(url_id: &str) -> Uuid {
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

#[derive(Clone)]
struct WsContext {
    pub message: Signal<Option<String>>,
    send: Rc<dyn Fn(&str)>,
}

impl WsContext {
    pub fn new(message: Signal<Option<String>>, send: Rc<dyn Fn(&str)>) -> Self {
        Self { message, send }
    }

    // create a method to avoid having to use parantheses around the field
    #[inline(always)]
    pub fn send(&self, message: &str) {
        (self.send)(message);
    }
}

type History = RwSignal<Vec<String>>;

/// Websocket encapsulation with the following features:
///
/// * History of messages
/// * Contract enforcement
///   * Serializer from [`ClientMsg`]
///   * Deserializer from [`ServerMsg`]
#[derive(Clone)]
struct Ws {
    history: History,
    ctx: WsContext,
}

impl Ws {
    pub fn new(url: &str) -> Self {
        let history: History = create_rw_signal(vec![format!("[init] Starting Ws to {}", &url)]);

        let UseWebsocketReturn { message, send, .. } = use_websocket_with_options(
            "/game/ws",
            UseWebSocketOptions::default()
                .on_open(Self::callback_open(history))
                .on_message(Self::callback_message(history)),
        );

        provide_context(WsContext::new(message, Rc::new(send.clone())));
        let ctx = expect_context::<WsContext>();

        Self { history, ctx }
    }

    fn callback_open(history: RwSignal<Vec<String>>) -> impl Fn(Event) {
        move |e: Event| {
            history.update(|h| h.push(format!("[onopen]: event {:?}", e.type_())));
        }
    }
    fn callback_message(history: RwSignal<Vec<String>>) -> impl Fn(String) {
        move |m: String| {
            history.update(|h| h.push(format!("[onmessage]: event {m}")));
        }
    }
}

impl Ws {
    pub fn send(&self, msg: ClientMsg) {
        let msg = msg.to_str();
        self.ctx.send(&msg);
        let () = self
            .history
            .update(|history: &mut Vec<_>| history.push(format!("[send] {msg}")));
    }

    #[inline(always)]
    pub fn message(&self) -> Memo<Option<ServerMsg>> {
        let s = self.ctx.message.get().map(|m| ServerMsg::from_str(&m));

        let msg = self.ctx.message;
        create_memo(move |_| msg.with(|m| m.clone().map(|m| ServerMsg::from_str(&m))))
    }
}

/// Main board View
#[component]
pub fn Board() -> impl IntoView {
    let params = use_params_map();
    let id = move || from_param_uuid(params, "id");
    let player_id = move || from_param_uuid(params, "player_id");

    let ws = Ws::new("/game/ws");

    // Respond to events
    let websocket = ws.clone();
    create_effect(move |_| {
        if websocket.message()() == Some(ServerMsg::Hello) {
            logging::log!("I'm sending the connect message");
            let conn_msg = msg::ClientMsg::Connect {
                game_id: id(),
                player_id: player_id(),
            };
            //websocket.send(&conn_msg.to_str());
            websocket.send(conn_msg);
        }
    });

    view! {
    <div class="h-screen bg-gray-200">
    <Nav ws=ws.clone()/>


    //<!-- Player Drawers -->
    <div class="mt-0.5 flex justify-around">
              <div class="py-20">
                                        <ul>
                        <For
                            each=move || ws.history.get().into_iter().enumerate()
                            key=|(index, _)| *index
                            let:item
                        >
                            <li>{item.1}</li>
                        </For>
                    </ul>
            </div>

        <HandHorizontal />
        <HandHorizontal />
    </div>
    <HandVertical left=false />
    <HandVertical left=true />

    <MiddleBoard />

    <PlayerDrawer ws=ws/>
    </div>
    }
}

#[component]
fn Nav(ws: Ws) -> impl IntoView {
    let players: RwSignal<Vec<msg::Player>> = create_rw_signal(vec![]);

    let updated_players = move || {
        if let Some(ServerMsg::Players(ps)) = ws.message()() {
            players.set(ps);
        }
        players.get()
    };

    view! {
     <nav class="flex justify-center">
      <div class="fixed top-2 content-center w-11/12 bg-white/30 backdrop-blur-md py-2 z-50 rounded-2xl">
         <div class="container mx-auto px-4 grid grid-cols-3 justify-items-center items-center text-white">
           <div class="justify-self-start">
             <h1>Rounds: 12</h1>
           </div>

           //<!-- Game Title -->
           <div class="justify-self-center">
             <h1 class="text-2xl">PLAI</h1>
           </div>

           //<!-- Players' Icons -->
           <div class="justify-self-end flex">
              <For
                each=move || updated_players().into_iter().enumerate()
                key=|(_, p) | p.id
                let:ip
              >
              <div class="group flex relative">
                  <span class="mx-1 h-6 w-6 rounded-full"
                    class:bg-blue=move || ip.0 == 0
                    class:bg-green=move || ip.0 == 1
                    class:bg-orange=move || ip.0 == 2
                    class:bg-yellow=move || ip.0 == 3
                    class:bg-gray-illustration=move || ip.0 == 4

                  ></span>
    <span class="group-hover:opacity-100 transition-opacity bg-gray-800 px-1 text-sm text-gray-100 rounded-md absolute left-1/2
    -translate-x-1/2 translate-y-full opacity-0 m-4 p-1 mx-auto">{ip.1.name}</span>
              </div>
              </For>
             //<div class="mx-1 h-6 w-6 rounded-full bg-blue"></div>
             //<div class="mx-1 h-6 w-6 rounded-full bg-green"></div>
             //<div class="mx-1 h-6 w-6 rounded-full bg-orange"></div>
             //<div class="mx-1 h-6 w-6 rounded-full bg-yellow"></div>
             //<div class="mx-1 h-6 w-6 rounded-full bg-gray-illustration"></div>
           </div>
         </div>
       </div>
    </nav>
       }
}

#[component]
fn MiddleBoard() -> impl IntoView {
    view! {
    //<!-- Middle Area for Decks -->
    <div class="my-8 flex justify-center space-x-4">
      <div class="h-32 w-24 bg-gray-700"></div>
      <div class="h-32 w-24 bg-gray-500"></div>
    </div>
      }
}

#[component]
fn HandVertical(left: bool) -> impl IntoView {
    view! {
    <div class="absolute left-5 top-1/4">
      <div class="rounded bg-white p-3">
        <h2>Player 1</h2>
        <div class="drawer-container">
          <div class="card-vertical will-change-transform"></div>
          <div class="card-vertical will-change-transform"></div>
          <div class="card-vertical will-change-transform"></div>
          <div class="card-vertical will-change-transform"></div>
          <div class="card-vertical will-change-transform"></div>
        </div>
      </div>
    </div>
    }
}

#[component]
fn HandHorizontal() -> impl IntoView {
    view! {
    <div class="rounded bg-white p-2">
      <div class="card-container p-2">
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
      </div>
      <h2>Player 3</h2>
    </div>
    }
}

#[component]
fn PlayerDrawer(ws: Ws) -> impl IntoView {
    view! {
    //<!-- Bottom Drawer for Player's Cards -->
    <div id="playersDrawer" class="fixed bottom-0 left-0 right-0 rounded-t-lg p-4 text-white  bg-green-700/20 backdrop-blur-md">

      <div class="grid grid-cols-3 justify-items-center">
        <div class="justify-self-start">
        <button onclick="toggleDrawer()" class="focus:shadow-outline rounded bg-red-500 px-4 py-2 font-bold text-white hover:bg-red-700 focus:outline-none">Hide</button>
        </div>
        <div>
          <h2>Your Cards</h2>
        </div>
        <div>
        </div>
      </div>

      <div class="grid justify-center">
      <div class="card-container pt-4">
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
      </div>
    </div>
    </div>
    }
}
