use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::ui::game::GameView;
use crate::ui::startup::Setup;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let players: RwSignal<Vec<(i32, RwSignal<String>)>> = create_rw_signal(vec![
        (0, create_rw_signal("Player 1".to_string())),
        (1, create_rw_signal("Player 2".to_string())),
    ]);

    view! {
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Setup players=players/> }/>
                <Route path="plai" view=move || view! { <GameView players=players/> }/>
            </Routes>
        </Router>

    <script src="https://cdnjs.cloudflare.com/ajax/libs/flowbite/2.3.0/flowbite.min.js"></script>
    }
}

#[component]
pub fn Button(title: String) -> impl IntoView {
    view! {
        <button class="bg-amber-600 hover:bg-amber-800 mx-3 px-5 py-3 text-white rounded-lg">
            {title}
        </button>
    }
}

#[component]
pub fn ButtonLink(title: String, href: String) -> impl IntoView {
    view! {
        <a href={href} class="bg-amber-600 hover:bg-amber-800 mx-3 px-5 py-3 text-white rounded-lg">
            {title}
        </a>
    }
}
