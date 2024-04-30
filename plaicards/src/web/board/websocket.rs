use axum::{
    body::Body as AxumBody,
    extract::{ws::Message, Path, RawQuery, State},
    http::{header::HeaderMap, Request},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use leptos::*;

use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::instrument;
use tracing::{debug, info, trace};
use uuid::Uuid;

use crate::web::{board::msg, Result as Res};

use super::msg::{ClientMsg, ServerMsg, WsSerDe};
use super::ssr::GameController;

pub async fn handler(ws: WebSocketUpgrade, State(gc): State<GameController>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, gc))
}

async fn send(ws: &mut WebSocket, msg: ServerMsg) {
    let msg = Message::Text(msg.to_str());
    trace!("Sending: {:?}", &msg);
    ws.send(msg).await.expect("Cant send message");
}

async fn receive(ws: &mut WebSocket) -> Option<ClientMsg> {
    while let Some(Ok(message)) = ws.recv().await {
        trace!("Message received: {:?}", &message);
        if let Message::Text(msg) = message {
            let msg = ClientMsg::from_str(&msg);
            return Some(msg);
        }
    }
    None
}

/// Currently this handles all the websocket connections for the lobby
///
/// It allows to send and receive information
async fn handle_socket(mut socket: WebSocket, gc: GameController) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    //let _ = socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok();
    send(&mut socket, ServerMsg::Hello).await;

    let mut gid: Option<Uuid> = None;
    let mut pid: Option<Uuid> = None;

    // First connection must be the game_id and player_id
    match receive(&mut socket).await {
        Some(ClientMsg::Connect { game_id, player_id }) => {
            gid = Some(game_id);
            pid = Some(player_id);
        }
        None => send(&mut socket, ServerMsg::BadRequest).await,
    };

    debug!("Passing over the first connection parsing...");

    let game_id = gid.expect("Wrong Game id");
    let player_id = pid.expect("wrong player_id");

    let room = gc
        .get(&game_id)
        .await
        .expect("No game was found with this ID");

    debug!("New client in room {:?}", room.id);

    // Send current game setup. This way if it reconnects the client obtains
    // a complete view on the game.
    {
        let game = room.game.read().await;
        let ps = &game.players;

        send(
            &mut socket,
            ServerMsg::Players(ps.iter().map(msg::Player::from).collect()),
        )
        .await;

        for p in ps {
            // TODO compare with player_id to send different info
            let hand = ServerMsg::RivalHand {
                id: p.id,
                num_cards: p.hand.len(),
            };
            send(&mut socket, hand).await;
        }
    }

    //
    // Normal operation
    //

    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();
    // Send received room messages to client
    let mut rx = room.tx.subscribe();
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Send the messages
            // in any websocket error, break loop
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from client
    let tx = room.tx.clone();
    let r = room.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            info!("Message received: {:?}", msg);
        }
    });

    // Execute tasks and abor tasks if any of them failed
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    // Send "user left" message (similar to "joined" above).
    let msg = "Player left.".to_string();
    tracing::debug!("{msg}");
    let _ = room.tx.send(msg);
}
