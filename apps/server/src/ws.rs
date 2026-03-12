use crate::{
    services::{auth::jwt::verify_token, ws::prelude::*},
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
use shared::{
    protocol::{ClientMessage, ServerMessage},
    structures::ids::UserId,
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
    println!("New connection: {} (waiting for identity)", socket_id);

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let current_user_id = loop {
        match ws_receiver.next().await {
            Some(Ok(Message::Text(text))) => match serde_json::from_str::<ClientMessage>(&text) {
                Ok(ClientMessage::Identify { token }) => match verify_token(&token) {
                    Ok(raw_id) => {
                        let user_id = UserId(raw_id);
                        println!("Socket {} authenticated as {}", socket_id, user_id.0);

                        match handle_identify(&state, user_id.clone(), tx.clone()).await {
                            Ok(_) => break user_id,
                            Err(e) => {
                                eprintln!("Identity failed for socket {}: {}", socket_id, e);
                                let _ =
                                    tx.send(Message::Close(Some(axum::extract::ws::CloseFrame {
                                        code: 4001,
                                        reason: "User not found".into(),
                                    })));
                                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                send_task.abort();
                                return;
                            }
                        }
                    }
                    Err(_) => {
                        let _ = tx.send(Message::Close(Some(axum::extract::ws::CloseFrame {
                            code: 4001,
                            reason: "User not found".into(),
                        })));
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        send_task.abort();
                        return;
                    }
                },
                _ => {
                    send_error(&tx, "Must identify first");
                }
            },
            _ => {
                println!("Socket {} disconnected before identifying", socket_id);
                send_task.abort();
                return;
            }
        }
    };

    let presence_state = state.clone();
    let presence_user_id = current_user_id.clone();
    let heartbeat = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            let _ = refresh_presence(&presence_state, &presence_user_id).await;
        }
    });

    while let Some(Ok(msg)) = ws_receiver.next().await {
        if let Message::Text(text) = msg {
            match serde_json::from_str::<ClientMessage>(&text) {
                Ok(client_msg) => {
                    handle_authenticated_message(client_msg, &current_user_id, &state, &tx).await;
                }
                Err(e) => {
                    eprintln!("Failed to parse message from {}: {}", current_user_id.0, e);
                    send_error(&tx, "Invalid message format");
                }
            }
        }
    }

    println!("User {} disconnected.", current_user_id.0);
    heartbeat.abort();
    send_task.abort();
    state.sessions.write().unwrap().remove(&current_user_id);
    let _ = handle_disconnect(&state, current_user_id).await;
}

async fn handle_authenticated_message(
    msg: ClientMessage,
    user_id: &UserId,
    state: &SharedState,
    tx: &tokio::sync::mpsc::UnboundedSender<Message>,
) {
    match msg {
        ClientMessage::Chat {
            channel_id,
            content,
        } => {
            if content.is_empty() || content.len() > 2000 {
                send_error(tx, "Message must be between 1 and 2000 characters");
                return;
            }
            if let Err(e) = handle_chat(channel_id, content, state, user_id.clone()).await {
                eprintln!("Chat error for {}: {}", user_id.0, e);
                send_error(tx, "Failed to send message");
            }
        }
        ClientMessage::SetPresence { presence } => {
            if let Err(e) = set_presence(state, user_id, &presence).await {
                eprintln!("Presence error for {}: {}", user_id.0, e);
            }
        }
        ClientMessage::Identify { .. } => {}
        _ => {}
    }
}

fn send_error(tx: &tokio::sync::mpsc::UnboundedSender<Message>, msg: &str) {
    let payload = ServerMessage::Error {
        message: msg.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&payload) {
        let _ = tx.send(Message::Text(json.into()));
    }
}
