use crate::{
    services::{chat, identity, jwt::verify_token, presence},
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
use shared::{protocol::ClientMessage, structures::UserId};
use uuid::Uuid;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: SharedState) {
    let socket_id = Uuid::new_v4().to_string();
    println!("New connection: {} (Waiting for identity)", socket_id);

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
            match client_msg {
                ClientMessage::Identify { token } => match verify_token(&token) {
                    Ok(raw_id) => {
                        let user_id = UserId(raw_id);
                        current_user_id = Some(user_id.clone());

                        println!("Socket {} authenticated as {}", socket_id, user_id.0);

                        if let Err(e) = identity::handle_identify(&state, user_id, tx.clone()).await
                        {
                            eprintln!("Failed to identify user: {}", e);
                        }
                    }

                    Err(e) => {
                        eprintln!("Socket {} provided invalid token: {}", socket_id, e);
                    }
                },
                ClientMessage::Chat {
                    channel_id,
                    content,
                } => {
                    if let Some(uid) = &current_user_id {
                        let _ = chat::handle_chat(channel_id, content, &state, uid.clone()).await;
                    } else {
                        eprintln!("Unauthenticated socket tried to chat!");
                    }
                }
                ClientMessage::SetPresence { presence } => {
                    if let Some(uid) = &current_user_id {
                        let _ = presence::set_presence(&state, uid, &presence).await;
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(user_id) = current_user_id {
        println!("User {} disconnected.", user_id.0);

        state.sessions.write().unwrap().remove(&user_id);

        let _ = identity::handle_disconnect(&state, user_id).await;
    } else {
        println!("Socket {} disconnected (Was never identified).", socket_id);
    }

    send_task.abort();
}
