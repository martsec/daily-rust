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
use uuid::Uuid;

use super::from_url_uuid;
use super::to_url_uuid;

use super::Player;

fn from_param_uuid(params: Memo<ParamsMap>, param_name: &str) -> (String, Uuid, bool) {
    let raw = params.with(|ps| {
        ps.get(param_name)
            .map(std::borrow::ToOwned::to_owned)
            .unwrap_or_default()
    });

    let uuid = from_url_uuid(&raw);
    let url = to_url_uuid(uuid);
    let need_reload = url != raw;
    (url, uuid, need_reload)
}
#[component]
#[must_use]
pub fn Lobby() -> impl IntoView {
    let params = use_params_map();

    // FIXME Every time we use this, we call this function
    // and if the user_id is wrong, it returns each time a different
    // random UUID breaking several things...
    //
    // We can't have this as a variable and then do the move
    let id = move || from_param_uuid(params, "id");
    let player_id = move || from_param_uuid(params, "player_id");

    if id().2 || player_id().2 {
        let url = move || format!("/lobby/{}/{}", id().0, player_id().0);
        return view! { <Redirect path=url()/> }.into_view();
    }

    let (players, set_players) = create_signal(vec![]);
    let (to_game, set_to_game) = create_signal(false);
    let game_url = move || format!("/plai/{}/{}", id().0, player_id().0);

    // Websocket
    let ws_url = format!("/lobby/{}/ws", id().1);
    // Update signals when new data arrives from the webhook
    let update_signals = move |m: String| {
        if let Some(slice) = m.strip_prefix("PLAYERS") {
            let ps: Vec<Player> = serde_json::from_str(slice).unwrap_or_default();
            set_players.set(ps);
        } else if m.starts_with("TO_GAME") {
            set_to_game.set(true);
        }
    };
    let UseWebsocketReturn { message, send, .. } = use_websocket_with_options(
        &ws_url,
        UseWebSocketOptions::default().on_message(update_signals),
    );

    let (name, set_name) = create_signal("MetaTrust".to_string());

    let send1 = send.clone();
    let new_player = move |_| {
        let p = Player {
            id: player_id().1,
            name: name(),
        };
        let json = serde_json::to_string(&p).unwrap();
        let m = format!("REFRESH_PLAIERS{json}");
        send1(&m);
    };

    let send1 = send.clone();
    let start_game = move |_| send.clone()("START_GAME");

    view! {
      <Show when=to_game fallback=|| view! {}>
        <Redirect path=game_url()/>
      </Show>

      <div class="my-10 mx-auto flex justify-center">
        <img
          src="/img/portada.png"
          alt="portada"
          class="w-[20rem] max-w-none rounded-xl sm:w-[57rem] md:-ml-4 lg:-ml-0"
          width="2432"
          height="1442"
        />
      </div>
      <div class="my-0 mx-auto max-w-3xl text-center">

        // <h2 class="p-6 text-4xl">"Welcome to PLAI"</h2>
        // <p class="px-10 pb-10">
        // "✨Become the artificial intelligence monopoly you deserve✨"
        // </p>
        <div class="flex grid grid-cols-2 items-center justify-around">
          <div class="my-2 px-6 lg:px-8">
            <div class="flex py-2 flex-col justify-center">
              <label for="name" class="block text-sm font-medium leading-6 text-gray-900">
                Your Startup Name
              </label>
              <input
                type="text"
                id="name"
                name="name"
                class="rounded-md border-0 py-1.5 pl-7 pr-20 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                placeholder="Try something edgy like MetaTrust"
                on:input=move |ev| {
                    set_name(event_target_value(&ev));
                }

                required
              />
            </div>
            <div class="my-2">
              <button
                type="submit"
                class="flex w-full justify-center rounded-md bg-emerald-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-emerald-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                on:click=new_player
              >
                "Create my startup"
              </button>
            </div>
          </div>

          <div class="my-2 px-6 lg:px-8">
            <PlayerList ps=players/>
            <div class="my-2">
              // <Show
              // when=move || { 2 <= players().len() && players().len() <= 6 }
              // fallback=|| view! { "Choose between 1 to 5 players." }
              // >
              <Button title="👩🏾‍💼 PLAI 👨🏾‍💼".to_string() on:click=start_game/>
            // </Show>
            </div>
          </div>
        </div>
      </div>
    }.into_view()
}

#[component]
fn PlayerList(ps: ReadSignal<Vec<Player>>) -> impl IntoView {
    view! {
      <div class="py-2">
        <h4 class="p-4 text-xl">Current players</h4>
        <ul role="list" class="divide-y divide-gray-100">
          <For each=move || ps.get().into_iter() key=|p| p.name.clone() let:p>
            <li class="py-2">
              <p class="text-m leading-6 text-gray-900">{p.name}</p>
            </li>
          </For>
        </ul>
      </div>
    }
}
