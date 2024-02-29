use crate::game::Game;
use leptos::*;
use crate::player::{Player, PlayerState};

#[component]
fn StartupIcon() -> impl IntoView {
    view! {
        <div class="z-10 flex items-center justify-center w-8 h-8 bg-green-600 rounded-full ring-0 ring-white sm:ring-8 shrink-0">
            <svg
                class="w-6 h-6 text-blue-100"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M13.6 16.7c.2.3.5.5.9.6a1.4 1.4 0 0 0 1.7-.8c.2-.6-.4-1.3-1.2-1.5-.8-.2-1.4-.8-1.2-1.5a1.4 1.4 0 0 1 1.7-.7c.4 0 .7.2.9.5m-1.4 4v.6m0-5.9v.7M4 15v4m3-6v6M6 8.5 10.5 5 14 7.5 18 4m0 0h-3.5M18 4v3m2 8a5 5 0 1 1-10 0 5 5 0 0 1 10 0Z"
                ></path>
            </svg>
        </div>
    }
}

#[component]
fn OpenSourceIcon() -> impl IntoView {
    view! {
        <div class="z-10 flex items-center justify-center w-8 h-8 bg-red-600 rounded-full ring-0 ring-white sm:ring-8 shrink-0">
            <svg
                class="w-6 h-6 text-green-100 "
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="currentColor"
                viewBox="0 0 24 24"
            >
                <path
                    fill-rule="evenodd"
                    d="M12 6a3.5 3.5 0 1 0 0 7 3.5 3.5 0 0 0 0-7Zm-1.5 8a4 4 0 0 0-4 4c0 1.1.9 2 2 2h7a2 2 0 0 0 2-2 4 4 0 0 0-4-4h-3Zm6.8-3.1a5.5 5.5 0 0 0-2.8-6.3c.6-.4 1.3-.6 2-.6a3.5 3.5 0 0 1 .8 6.9Zm2.2 7.1h.5a2 2 0 0 0 2-2 4 4 0 0 0-4-4h-1.1l-.5.8c1.9 1 3.1 3 3.1 5.2ZM4 7.5a3.5 3.5 0 0 1 5.5-2.9A5.5 5.5 0 0 0 6.7 11 3.5 3.5 0 0 1 4 7.5ZM7.1 12H6a4 4 0 0 0-4 4c0 1.1.9 2 2 2h.5a6 6 0 0 1 3-5.2l-.4-.8Z"
                    clip-rule="evenodd"
                ></path>
            </svg>

        </div>
    }
}

#[component]
fn EliminatedIcon() -> impl IntoView {
    view! {
        <div class="z-10 flex items-center justify-center w-8 h-8 bg-grey-600 rounded-full ring-0 ring-white sm:ring-8 shrink-0">
            <svg
                class="w-6 h-6 text-red-200"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M5 12h14"
                ></path>
            </svg>
        </div>
    }
}

#[component]
fn PlayerStatus<'a>(player: &'a Player) -> impl IntoView {
    use rand::distributions::{Alphanumeric, DistString};
    let state = player.state().clone();
    let state = move || match state {
        PlayerState::Startup => view! { <StartupIcon/> },
        PlayerState::OpenSource => view! { <OpenSourceIcon/> },
        PlayerState::Eliminated => view! { <EliminatedIcon/> },
    };
    let name = player.name().clone();
    let tooltip_id = format!(
        "tooltip-player-{}",
        Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
    );

    view! {
        <li class="flex items-center mx-4">
            <span
                class="me-2"
                data-tooltip-trigger="hover"
                data-tooltip-target=&tooltip_id
                data-tooltip-placement="bottom"
            >
                {state}
            </span>

            {name}

            <div
                id=tooltip_id
                role="tooltip"
                class="absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white bg-gray-900 rounded-lg shadow-sm opacity-0 tooltip dark:bg-gray-700"
            >
                "Player is "
                {player.state().to_string()}
                <div class="tooltip-arrow" data-popper-arrow></div>
            </div>

        </li>
    }
}

#[component]
pub fn GameStatusBar(game: RwSignal<Game>) -> impl IntoView {
    let players = game.get().players;
    let player_list = players
        .iter()
        .map(|p| view! { <PlayerStatus player=&p/> })
        .collect_view();

    view! {
        <div class="top-0 left-0 z-50 grid w-full h-16 grid-cols-1 px-8 bg-white border-t-4 border-red-300 md:grid-cols-3">

            // Rounds
            <div class="items-center justify-center hidden text-gray-500 dark:text-gray-400 me-auto md:flex">
                <svg
                    class="w-6 h-6 text-gray-800 dark:text-white"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M17.7 7.7A7.1 7.1 0 0 0 5 10.8M18 4v4h-4m-7.7 8.3A7.1 7.1 0 0 0 19 13.2M6 20v-4h4"
                    ></path>
                </svg>
                <span class="text-sm px-2">{game.get().round.number} " rounds"</span>

            </div>
            // Player status
            <div class="flex items-center justify-center mx-auto">
                <ol class="flex items-center w-full text-sm font-medium text-center text-gray-500 dark:text-gray-400 sm:text-base">
                    {player_list}
                </ol>
            </div>

        </div>
    }
}
