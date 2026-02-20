use crate::services::identity::handle_identify;
use crate::services::{broadcast, identity, presence};
use crate::{
    requests_ws::{chat, identify, set_status},
    state::SharedState,
};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use shared::structures::user;
use shared::{
    protocol::{ClientMessage, ServerMessage},
    structures::{PresenceStatus, UserId},
};
use uuid::Uuid;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: SharedState) {
    let socket_id = Uuid::new_v4().to_string();
    println!("New connection: {} (Waiting for Identity)", socket_id);

    let mut current_user_id: Option<UserId> = None;

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(msg)) = ws_receiver.next().await {
        if let Message::Text(text) = msg
            && let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text)
        {
            if let ClientMessage::Identify { ref user_id } = client_msg {
                current_user_id = Some(user_id.clone());
            }

            handle_client_message(tx.clone(), client_msg, &state, &current_user_id).await;
        }
    }

    if let Some(user_id) = current_user_id {
        println!("User {} disconnected.", user_id.0);

        {
            let mut guard = state.lock().await;
            guard.sessions.remove(&user_id);
        }

        identity::handle_disconnect(&state, &user_id).await;
    } else {
        println!("Socket {} disconnected (Was never identified).", socket_id);
    }

    send_task.abort();
}

async fn handle_client_message(
    tx: tokio::sync::mpsc::UnboundedSender<Message>,
    msg: ClientMessage,
    state: &SharedState,
    current_user_id: &Option<UserId>,
) {
    match msg {
        ClientMessage::Identify { user_id } => {
            if let Err(e) = handle_identify(state, user_id, tx).await {
                eprintln!("Failed to identify user: {}", e);
            }
        }
        ClientMessage::Chat {
            channel_id,
            content,
        } => {
            if let Some(uid) = current_user_id {
                chat(channel_id.0, content, state, uid.0.clone()).await;
            }
        }
        ClientMessage::SetStatus { status } => {
            if let Some(uid) = current_user_id {
                let _ = presence::set_presence(&state, uid, status).await;
            }
        }
        _ => {}
    }
}
