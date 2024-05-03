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

use crate::{
    game::TurnAction,
    web::{
        board::{msg, ssr::GameRoom},
        Error, Result as Res,
    },
};

use super::msg::{ClientMsg, ServerMsg, WsSerDe};
use super::ssr::GameController;
use crate::game::{Error as GError, Game};

pub async fn handler(ws: WebSocketUpgrade, State(gc): State<GameController>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, gc))
}

async fn send(ws: &Sender<ServerMsg>, msg: ServerMsg) {
    //let msg = Message::Text(msg.to_str());
    trace!("Sending: {:?}", &msg);
    ws.send(msg).await.expect("Cant send message");
}

/// Wrapper to send messages back to the client or to the room
#[derive(Clone)]
struct WsSender {
    room: tokio::sync::broadcast::Sender<ServerMsg>,
    client: tokio::sync::mpsc::Sender<ServerMsg>,
}

impl WsSender {
    pub fn new(
        room: tokio::sync::broadcast::Sender<ServerMsg>,
        client: tokio::sync::mpsc::Sender<ServerMsg>,
    ) -> Self {
        Self { room, client }
    }

    pub async fn to_room(&self, msg: ServerMsg) -> Res<()> {
        trace!("[WS->Room]: {:?}", &msg);
        match self.room.send(msg) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::WebsocketError),
        }
    }

    pub async fn to_client(&self, msg: ServerMsg) -> Res<()> {
        trace!("[WS->Client]: {:?}", &msg);
        self.client
            .send(msg)
            .await
            .map_err(|_| Error::WebsocketError)
    }
}

async fn receive(ws: &mut SplitStream<WebSocket>) -> Option<ClientMsg> {
    while let Some(Ok(message)) = ws.next().await {
        trace!("[Client->WS]: {:?}", &message);
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
async fn handle_socket(socket: WebSocket, gc: GameController) {
    // By splitting, we can send and receive at the same time.
    let (mut sink, mut receiver) = socket.split();

    // create an mpsc so we can send messages to the sink from multiple threads
    let (sender, mut rx) = mpsc::channel::<ServerMsg>(2);

    // spawn a task that forwards messages from the mpsc to the sink
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let msg = Message::Text(message.to_str());
            if sink.send(msg).await.is_err() {
                break;
            }
        }
    });

    ////////////////////////////////
    // Initialization with client
    ////////////////////////////////
    send(&sender, ServerMsg::Hello).await;
    let mut gid: Option<Uuid> = None;
    let mut pid: Option<Uuid> = None;

    // First connection must be the game_id and player_id
    match receive(&mut receiver).await {
        Some(ClientMsg::Connect { game_id, player_id }) => {
            gid = Some(game_id);
            pid = Some(player_id);
        }
        _ => send(&sender, ServerMsg::BadRequest).await,
    };

    let game_id = gid.expect("Wrong Game id");
    let player_id = pid.expect("wrong player_id");

    let room = gc
        .get(&game_id)
        .await
        .expect("No game was found with this ID");

    debug!("New client in room {:?}", room.id);
    let sender = WsSender::new(room.tx.clone(), sender);

    // Send current game setup. This way if it reconnects the client obtains
    // a complete view on the game.
    {
        let game = room.game.read().await;
        let ps = &game.players;

        sender
            .to_client(ServerMsg::Players(
                ps.iter().map(msg::Player::from).collect(),
            ))
            .await
            .expect("WSERR");

        for p in ps {
            if p.id == player_id {
                for card in p.hand.card_iter() {
                    let c = msg::Card::from(card);
                    sender
                        .to_client(ServerMsg::AddCard(c))
                        .await
                        .expect("WSERR");
                }
            } else {
                let hand = ServerMsg::RivalHand {
                    id: p.id,
                    num_cards: p.hand.len(),
                };
                sender.to_client(hand).await.expect("WSERR");
            }
        }
        // Forward next player to room
        sender
            .to_client(ServerMsg::NextPlayer(game.active_player().id))
            .await
            .expect("WSERR");
    }

    //
    // Normal operation
    //

    // Forward received room messages to client
    let s = sender.clone();
    let mut rx = room.tx.subscribe();
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Send the messages
            // in any websocket error, break loop
            if s.to_client(msg).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from client
    let s = sender.clone();
    let r = room.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receive(&mut receiver).await {
            info!("Message received: {:?}", msg);
            match msg {
                ClientMsg::Connect { .. } => {}
                ClientMsg::DoFunding(funding) => {
                    let mut game = r.game.write().await;
                    match game.turn_action(player_id, TurnAction::Funding(funding)) {
                        Ok(_) => {}
                        Err(e) => {
                            s.to_client(e.into()).await.expect("WSERR");
                            continue;
                        }
                    }
                    update_player_state(&s, &game, &player_id)
                        .await
                        .expect("WSERR");
                    // Forward next player to room
                    s.to_room(ServerMsg::NextPlayer(game.active_player().id))
                        .await
                        .expect("WSERR");
                }
            }
        }
    });

    // Execute tasks and abor tasks if any of them failed
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    // Send "user left" message (similar to "joined" above).
    tracing::debug!("Player left");
    let _ = sender.to_room(ServerMsg::PlayerLeft).await;
}

/// Sends an update for the current player to all players
async fn update_player_state(s: &WsSender, game: &Game, player_id: &Uuid) -> Res<()> {
    // Updating user hand
    let p = game.get_player(*player_id);
    if let Some(card) = p.hand.card_iter().last() {
        let c = msg::Card::from(card);
        s.to_client(ServerMsg::AddCard(c)).await?;
    }
    // Forwarding new state to clients
    let hand = ServerMsg::RivalHand {
        id: p.id,
        num_cards: p.hand.len(),
    };
    s.to_room(hand).await.expect("ws error");

    Ok(())
}
