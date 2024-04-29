use crate::error_template::{AppError, ErrorTemplate};
use crate::web::board::Board;
use crate::web::lobby::Lobby;
use crate::web::HomePage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
#[must_use] pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        // Stylesheet name is the CRATE NAME by default!!!
        <Stylesheet id="leptos" href="/pkg/plaicards.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }
            trailing_slash={TrailingSlash::Redirect}
        >
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/lobby/:id/:player_id" view=move || view!{ <Lobby/>} />
                    <Route path="/plai/:id/:player_id" view=move || view!{ <Board/>} />
                </Routes>
            </main>
        </Router>
    }
}
