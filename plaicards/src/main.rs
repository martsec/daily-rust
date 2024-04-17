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
use leptos::*;
use leptos_axum::{
    generate_route_list, handle_server_fns_with_context, render_app_to_stream_with_context,
    LeptosRoutes,
};

// Tracing
use tracing::Instrument;
use tracing::{event, info, instrument, span, Level};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::FmtSubscriber;

use plaicards::web::{lobby::Player, ssr::AppState, Result as Res};
use plaicards::{app::*, web::lobby::ssr::LobbyController};
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
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
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

    let app_state = AppState {
        leptos_options,
        lobby: lobby_controller,
        routes: routes.clone(),
    };

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .route("/lobby/:id/ws", get(handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state);

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

use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use futures::{sink::SinkExt, stream::StreamExt};
use uuid::Uuid;

async fn handler(
    ws: WebSocketUpgrade,
    Path(lobby_id): Path<Uuid>,
    State(lc): State<LobbyController>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, lobby_id, lc))
}

/// Currently this handles all the websocket connections for the lobby
///
/// It allows to send and receive information
async fn handle_socket(socket: WebSocket, lobby_id: Uuid, lc: LobbyController) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();
    // TODO get path params for lobby_id
    //
    let lobby = lc.get_lobby(lobby_id.clone()).await.unwrap();

    let mut rx = lobby.tx.subscribe();

    let _ = lobby.tx.send("A new player joined".to_string());

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = lobby.tx.clone();
    let lb = lobby.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            info!("Message received: {:?}", msg);
            if msg.starts_with("REFRESH_PLAIERS") {
                let slice = &msg[15..];
                let p: Player = serde_json::from_str(slice).expect("malformed player");
                let _ = add_player(&lb, p).await.expect("Failed with p");
                // Refresh player list to all
                let updated_players = get_players(&lb).await;
                let _ = tx.send(updated_players);
            } else {
                continue;
            }
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    let msg = format!("Player left.");
    tracing::debug!("{msg}");
    let _ = lobby.tx.send(msg);
}

#[instrument]
async fn get_players(lobby: &Lobby) -> String {
    let players = lobby.players.lock().unwrap().clone();
    let json = serde_json::to_string(&players).unwrap();
    info!("{}", &json);
    format!("PLAYERS{}", json)
}

#[instrument]
async fn add_player(lobby: &Lobby, p: Player) -> Res<()> {
    let p = lobby.update_player(p)?;
    Ok(())
}
