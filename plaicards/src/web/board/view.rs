use std::collections::hash_map::HashMap;
use std::rc::Rc;

use crate::game::Funding;
use crate::web::common::Button;
use crate::web::common::ButtonDisablable;
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
use uuid::Uuid;

use super::msg;
use super::msg::{ClientMsg, ServerMsg, WsSerDe};

fn from_param_uuid(params: Memo<ParamsMap>, param_name: &str) -> Option<Uuid> {
    let raw = params.with_untracked(|ps| {
        ps.get(param_name)
            .map(std::borrow::ToOwned::to_owned)
            .unwrap_or_default()
    });

    from_url_uuid(&raw)
}

fn from_url_uuid(url_id: &str) -> Option<Uuid> {
    let res = Uuid::try_from(
        BASE64URL_NOPAD
            .decode(url_id.as_bytes())
            .unwrap_or_default(),
    );
    res.ok()
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
///
///   In order to pass it to Children (sub components), use the `provide_context`
///   in the parent and `expect_context` functions from leptos.
///
///   ```rust,ignore
///   use leptos::{expect_context, provide_context};
///   use crate::web::board::view::Ws;
///
///   let ws = Ws::new(..);
///   provide_context(ws);
///
///   // And inside any child component
///   let ws = expect_context::<Ws>();
///   ```
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
        let ctx = WsContext::new(message, Rc::new(send.clone()));
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

    pub fn message(&self) -> Memo<Option<ServerMsg>> {
        let ctx = self.ctx.clone();
        create_memo(move |_| {
            ctx.message
                .with(|m| m.clone().map(|m| ServerMsg::from_str(&m)))
        })
    }
}

/// Main board View
#[component]
pub fn Board() -> impl IntoView {
    let params = use_params_map();
    let id = move || from_param_uuid(params, "id");
    let player_id = move || from_param_uuid(params, "player_id");

    if id().is_none() || player_id().is_none() {
        let lobby = format!("/lobby/{}/{}", Uuid::new_v4(), Uuid::new_v4());
        return view! { <Redirect path=lobby/> }.into_view();
    }
    let id = move || id().expect("Internal error with params");
    let player_id = move || player_id().expect("Internal error with params");

    let ws = Ws::new("/game/ws");
    provide_context(ws.clone());

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
        <Nav/>

        <PlayersHands current_player=player_id()/>

        <MiddleBoard/>
        <div class="flex justify-around mt-0.5">
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
        </div>
      </div>
    }
    .into_view()
}

#[component]
fn Nav() -> impl IntoView {
    let players: RwSignal<Vec<msg::Player>> = create_rw_signal(vec![]);
    let ws = expect_context::<Ws>();

    let updated_players = move || {
        if let Some(ServerMsg::Players(ps)) = ws.message()() {
            players.set(ps);
        }
        players.get()
    };

    create_effect(move |_| {
        logging::log!("Value of players updated in NAV {:?}", players());
    });

    view! {
      <nav class="flex justify-center">
        <div class="fixed top-2 z-50 content-center py-2 w-11/12 rounded-2xl bg-white/30 backdrop-blur-md">
          <div class="container grid grid-cols-3 justify-items-center items-center px-4 mx-auto text-white">
            <div class="justify-self-start">
              <h1>Rounds: 12</h1>
            </div>

            // <!-- Game Title -->
            <div class="justify-self-center">
              <h1 class="text-2xl">PLAI</h1>
            </div>

            // <!-- Players' Icons -->
            <div class="flex justify-self-end">
              <For each=move || updated_players().into_iter().enumerate() key=|(_, p)| p.id let:ip>
                <div class="flex relative group">
                  <span
                    class="mx-1 w-6 h-6 rounded-full"
                    class=("bg-blue", move || ip.0 == 0)
                    class=("bg-green", move || ip.0 == 1)
                    class=("bg-orange", move || ip.0 == 2)
                    class=("bg-yellow", move || ip.0 == 3)
                    class=("bg-gray-illustration", move || ip.0 == 4)
                  ></span>
                  <span class="absolute left-1/2 p-1 px-1 m-4 mx-auto text-sm text-gray-100 bg-gray-800 rounded-md opacity-0 transition-opacity -translate-x-1/2 translate-y-full group-hover:opacity-100">
                    {ip.1.name}
                  </span>
                </div>
              </For>
            </div>
          </div>
        </div>
      </nav>
    }
}

#[component]
fn PlayersHands(current_player: Uuid) -> impl IntoView {
    let ws = expect_context::<Ws>();
    let players: RwSignal<Vec<msg::Player>> = create_rw_signal(vec![]);

    let updated_players = move || {
        if let Some(ServerMsg::Players(ps)) = ws.message()() {
            // Order list starting by current player
            let idx = ps
                .iter()
                .position(|p| p.id == current_player)
                .unwrap_or_else(|| {
                    panic!("Current player ID  {current_player} not available in the player list")
                });

            let mut sorted_ps = ps[idx..].to_vec();
            sorted_ps.extend(ps[..idx].to_vec());

            players.set(sorted_ps);
        }
        players.get()
    };

    view! {
      <Show when=move || !updated_players().is_empty() fallback=|| view! {}>

        <PlayerDrawer player=players.get()[0].clone()/>
        <Show
          when=move || { players().len() > 2 }
          fallback=move || {
              view! {
                <div class="flex justify-around mt-0.5">
                  <HandHorizontal player=players.get()[1].clone()/>
                </div>
              }
          }
        >

          <HandVertical player=players.get()[1].clone() left=true/>
          <div class="flex justify-around mt-0.5">
            <HandHorizontal player=players.get()[2].clone()/>
            <Show when=move || players().len() == 5 fallback=|| view! {}>
              <HandHorizontal player=players.get()[3].clone()/>
            </Show>
          </div>
          <Show
            when=move || players().len() == 5
            fallback=move || view! { <HandVertical player=players.get()[3].clone() left=false/> }
          >
            <HandVertical player=players.get()[4].clone() left=false/>
          </Show>

        </Show>
      </Show>
    }
}

#[component]
fn MiddleBoard() -> impl IntoView {
    view! {
      <div class="flex justify-center my-8 space-x-4">
        <div class="w-24 h-32 bg-gray-700"></div>
        <div class="w-24 h-32 bg-gray-500"></div>
      </div>
    }
}

#[component]
fn HandVertical(player: msg::Player, left: bool) -> impl IntoView {
    let ws = expect_context::<Ws>();
    let cards_memory = create_rw_signal(0);
    let cards = move || {
        if let Some(ServerMsg::RivalHand { id, num_cards }) = ws.message()() {
            if id == player.id {
                cards_memory.set(num_cards);
            }
        }
        cards_memory()
    };

    view! {
      <div class="absolute top-1/4" class=("left-5", move || left) class=("right-5", move || !left)>
        <div class="p-3 bg-white rounded">
          <h2>{player.name}</h2>
          <div class="drawer-container">
            {move || {
                vec![0; cards()]
                    .into_iter()
                    .map(|_| {
                        view! {
                          <div class="bg-cover animate-slideIn card-vertical bg-card-back will-change-transform"></div>
                        }
                    })
                    .collect_view()
            }}

          </div>
        </div>
      </div>
    }
}

#[component]
fn HandHorizontal(player: msg::Player) -> impl IntoView {
    let ws = expect_context::<Ws>();

    let cards_memory = create_rw_signal(0);
    let cards = move || {
        if let Some(ServerMsg::RivalHand { id, num_cards }) = ws.message()() {
            if id == player.id {
                cards_memory.set(num_cards);
            }
        }
        cards_memory()
    };

    view! {
      <div class="p-2 bg-white rounded">
        <div class="p-2 card-container">
          {move || {
              vec![0; cards()]
                  .into_iter()
                  .map(|_| {
                      view! {
                        <div class="bg-cover animate-slideIn card bg-card-back will-change-transform"></div>
                      }
                  })
                  .collect_view()
          }}

        </div>
        <h2>{player.name}</h2>
      </div>
    }
}

#[component]
fn PlayerDrawer(player: msg::Player) -> impl IntoView {
    let ws = expect_context::<Ws>();
    let ws_message = ws.message();

    let cards: RwSignal<Vec<msg::Card>> = create_rw_signal(vec![]);
    let updated_hand = move || {
        if let Some(ServerMsg::AddCard(c)) = ws_message() {
            cards.update(|cs| cs.push(c));
        }
        cards()
    };

    let is_players_turn = create_rw_signal(false);
    let check_player_turn = move || {
        if let Some(ServerMsg::NextPlayer(pid)) = ws_message() {
            is_players_turn.set(pid == player.id);
        }
        is_players_turn.get()
    };
    provide_context(is_players_turn);
    create_effect(move |_| {
        if is_players_turn() {
            logging::log!("It's now your turn");
        }
    });

    let type_to_color = HashMap::from([
        ("Adversary".to_string(), "text-orange"),
        ("UseCase".to_string(), "text-green"),
        ("Buzzword".to_string(), "text-yellow"),
        ("MarketEvent".to_string(), "text-blue"),
        ("Special".to_string(), "text-gray"),
    ]);

    view! {
      <div
        id="playersDrawer"
        class="fixed right-0 bottom-0 left-0 p-4 text-white rounded-t-lg hover:z-20 bg-green-700/20 backdrop-blur-md"
      >

        <div class="grid grid-cols-3 justify-items-center">
          <div class="justify-self-start">
            <PlayerActions player=player.clone()/>
          </div>
          <div>
            <h2>Your Cards {player.name}</h2>
          </div>
          <div class="justify-self-end">
            <p>{move || if check_player_turn() { "Your turn" } else { "" }}</p>
          </div>
        </div>

        <div class="grid justify-center">
          <div class="pt-4 card-container">
            <For
              each=move || updated_hand().into_iter().enumerate()
              key=|(_, c)| c.title.clone()
              children=move |(i, c)| {
                  view! {
                    <div
                      class=&format!(
                          "animate-slideIn card card-faceup will-change-transform text-center py-6 bg-cover bg-card-{}",
                          c.ctype.to_lowercase(),
                      )

                      class=("bg-card-adversary", || false)
                      class=("bg-card-usecase", || false)
                      class=("bg-card-buzzword", || false)
                      class=("bg-card-special", || false)
                      class=("bg-card-marketevent", || false)
                    >
                      <p class=format!(
                          "select-none uppercase text-gray-illustration font-extrabold mt-10 {}",
                          type_to_color.get(&c.ctype).unwrap_or(&"text-gray-illustration"),
                      )>{c.title}</p>
                      <p class="mt-2 font-bold text-black uppercase select-none">{c.effect}</p>
                      <p class="mt-2 italic select-none text-dove-gray">{c.description}</p>
                    </div>
                  }
              }
            />

          </div>
        </div>
      </div>
    }
}

#[component]
fn PlayerActions(player: msg::Player) -> impl IntoView {
    let ws = expect_context::<Ws>();
    let is_players_turn =
        use_context::<RwSignal<bool>>().expect("to have found the players turn signal");

    let is_not_turn = Signal::derive(move || !is_players_turn());
    let funding_button = create_rw_signal(None);
    create_effect(move |_| match funding_button() {
        None => {}
        Some(f) => {
            ws.send(ClientMsg::DoFunding(f));
        }
    });

    view! {
      <ButtonDisablable
        title="Family Funding".into()
        disabled=is_not_turn
        on:click=move |_| {
            funding_button.set(Some(Funding::Family));
        }
      />

      <ButtonDisablable
        title="Regional Funding".into()
        disabled=is_not_turn
        on:click=move |_| {
            funding_button.set(Some(Funding::Regional));
        }
      />

      <ButtonDisablable
        title="VC Funding".into()
        disabled=is_not_turn
        on:click=move |_| {
            funding_button.set(Some(Funding::VC));
        }
      />
    }
}
