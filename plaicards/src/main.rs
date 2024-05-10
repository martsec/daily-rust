#![allow(unused_variables)]

use std::collections::HashMap;

use axum::{
    body::Body as AxumBody,
    extract::{ws::Message, Path, RawQuery, State},
    http::{header::HeaderMap, Request},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use leptos::{get_configuration, provide_context, tracing, Props};
use leptos_axum::{
    generate_route_list, handle_server_fns_with_context, render_app_to_stream_with_context,
    LeptosRoutes,
};

use tower_http::compression::CompressionLayer;
// Tracing
use tracing::Instrument;
use tracing::{event, info, instrument, span, Level};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::FmtSubscriber;

use plaicards::web::board::{board_handler, GameController};
use plaicards::web::lobby::lobby_handler;
use plaicards::web::{lobby::Player, ssr::AppState, Result as Res};
use plaicards::{app::App, web::lobby::ssr::LobbyController};
use plaicards::{fileserv::file_and_error_handler, web::lobby::ssr::Lobby};

async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.lobby.clone());
        },
        request,
    )
    .await
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = render_app_to_stream_with_context(
        app_state.leptos_options.clone(),
        move || {
            provide_context(app_state.lobby.clone());
        },
        App,
    );
    handler(req).await.into_response()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tracing
    // See https://github.com/tokio-rs/tracing?tab=readme-ov-file
    let log_level = if cfg!(debug_assertions) {
        tracing::Level::TRACE
    } else {
        tracing::Level::INFO
    };
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .compact()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // TODO add axum insight
    // https://crates.io/crates/axum-insights

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let lobby_controller = LobbyController::new().await;

    let game_controller = GameController::new().await;

    let app_state = AppState {
        leptos_options,
        gc: game_controller,
        lobby: lobby_controller,
        routes: routes.clone(),
    };

    let compression_layer = CompressionLayer::new().deflate(true).gzip(true);

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .route("/lobby/:lobby_id/ws", get(lobby_handler))
        .route("/game/ws", get(board_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state)
        .layer(compression_layer);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Starting server. Listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}

// TODO move me out just for testing
//
//
