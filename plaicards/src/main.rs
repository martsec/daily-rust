#![allow(unused_variables)]

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

use plaicards::fileserv::file_and_error_handler;
use plaicards::web::ssr::AppState;
use plaicards::{app::*, web::lobby::ssr::LobbyController};

async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    println!("{:?}", path);

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
async fn main() {
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
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
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
use uuid::Uuid;

async fn handler(
    ws: WebSocketUpgrade,
    Path(lobby_id): Path<Uuid>,
    State(lc): State<LobbyController>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, lobby_id, lc))
}

async fn handle_socket(mut socket: WebSocket, lobby_id: Uuid, lc: LobbyController) {
    // TODO get path params for lobby_id

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            println!("Message received: {:?}", msg);
            get_players(&lobby_id, &lc).await
        } else {
            println!("Client disconnected");
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

async fn get_players(lobby_id: &Uuid, lc: &LobbyController) -> Message {
    let lobby = lc.get_lobby(lobby_id.clone()).await.unwrap();
    dbg!(&lobby);
    let players = lobby.players.lock().unwrap().clone();
    let json = serde_json::to_string(&players).unwrap();
    dbg!(&json);
    Message::Text(json)
}
