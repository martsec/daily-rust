use axum::{
    body::Body as AxumBody,
    extract::{ws::Message, Path, RawQuery, State},
    http::{header::HeaderMap, Request},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use headers::Server;
use leptos::*;

use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use futures::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use tokio::sync::mpsc::{self, Sender};
use tracing::instrument;
use tracing::{debug, info, trace};
use uuid::Uuid;

use crate::web::board::GameController;

use super::ssr::{Lobby, LobbyController};
use super::Player;
use super::Res;

pub async fn handler(
    ws: WebSocketUpgrade,
    Path(lobby_id): Path<Uuid>,
    State(lc): State<LobbyController>,
    State(gc): State<GameController>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, lobby_id, lc, gc))
}

/// Currently this handles all the websocket connections for the lobby
///
/// It allows to send and receive information
async fn handle_socket(socket: WebSocket, lobby_id: Uuid, lc: LobbyController, gc: GameController) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();
    // TODO get path params for lobby_id
    //
    let lobby = lc.get_lobby(lobby_id).await.unwrap();

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
                let () = add_player(&lb, p).await.expect("Failed with p");
                // Refresh player list to all
                let updated_players = get_players(&lb).await;
                let _ = tx.send(updated_players);
            } else if msg.starts_with("START_GAME") {
                let players = lobby.players.lock().unwrap().clone();
                let players: Vec<(Uuid, String)> =
                    players.iter().map(|p| (p.id, p.name.clone())).collect();
                gc
                    .new_game(lobby_id, &players)
                    .await
                    .expect("ERROR adding new game");
                let _ = tx.send("TO_GAME".to_string());
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
    let msg = "Player left.".to_string();
    tracing::debug!("{msg}");
    let _ = lobby.tx.send(msg);
}

#[instrument]
async fn get_players(lobby: &Lobby) -> String {
    let players = lobby.players.lock().unwrap().clone();
    let json = serde_json::to_string(&players).unwrap();
    format!("PLAYERS{json}")
}

#[instrument]
async fn add_player(lobby: &Lobby, p: Player) -> Res<()> {
    let p = lobby.update_player(p)?;
    Ok(())
}
