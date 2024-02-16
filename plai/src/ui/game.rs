//! Everything related to the game rendering engine

use crate::game::{Game, Player, PlayerState};
use crate::ui::app::Button;
use crate::ui::game_status::GameStatusBar;
use leptos::*;
use leptos_router::Redirect;

/// Main game component
#[component]
pub fn GameView(players: RwSignal<Vec<(i32, RwSignal<String>)>>) -> impl IntoView {
    let players: Vec<String> = players.get().iter().map(|(_, name)| name.get()).collect();
    let game = Game::new(&players);
    let game = create_rw_signal(game);

    if players.is_empty() {
        return view! { <Redirect path="/"/> };
    }

    view! {
        <div class="mb-8 items-center ">
            <GameStatusBar game=game/>
        </div>

        <div class="my-0 mx-auto max-w-3xl text-center">
            <PlayerDrawer/>
        </div>
    }
    .into()
}

#[component]
pub fn Card(title: String) -> impl IntoView {
    view! { <div class="card text-center shadow-lg shadow-gray-200 bg-white ">{title}</div> }
}

#[component]
pub fn PlayerHand(cards: ReadSignal<Vec<String>>) -> impl IntoView {
    let cards = move || {
        cards
            .get()
            .into_iter()
            .map(|d| {
                view! { <Card title=d/> }
            })
            .collect_view()
    };
    view! {
        <div class="grid grid-flow-row text-neutral-600 sm:grid-cols-1 md:grid-cols-9 lg:grid-cols-5 xl:grid-cols-9">
            {cards}
        </div>
    }
}

#[component]
pub fn PlayerDrawer() -> impl IntoView {
    let plain_cards: Vec<String> = (0..5).map(|n| format!("Card {n}")).collect();
    let (cards, set_cards) = create_signal(plain_cards);
    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`,
    // so it's super easy to move them into closures
    let increment = move |_| set_cards.update(|cs| cs.push("NewCard".to_string()));
    let decrement = move |_| {
        set_cards.update(|cs| {
            cs.pop();
        });
    };

    view! {
        <div class="text-center">
            <h4 class="inline-flex items-center text-base text-gray-500 dark:text-gray-400 font-medium">
                "Player's Hand"
            </h4>

            <Button title="Draw card".to_string() on:click=increment/>
            <Button title="Plai card".to_string() on:click=decrement/>

            <div class="mx-8 my-2">
                <PlayerHand cards=cards/>
            </div>
        </div>
    }
}
