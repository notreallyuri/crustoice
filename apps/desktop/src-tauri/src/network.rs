use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use shared::{
    protocol::{ClientMessage, ServerMessage},
    structures::UserId,
};
use tauri::Emitter;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::{client_state::ClientStore, WS_URL};

pub async fn connect_to_server(
    app_handle: tauri::AppHandle,
    user_id: UserId,
    store: Arc<Mutex<ClientStore>>,
    ws_sender: Arc<Mutex<Option<mpsc::UnboundedSender<String>>>>,
) {
    match connect_async(WS_URL).await {
        Ok((ws_stream, _)) => {
            let (mut write, mut read) = ws_stream.split();
            let (tx, mut rx) = mpsc::unbounded_channel::<String>();

            {
                let mut sender_guard = ws_sender.lock().await;
                *sender_guard = Some(tx.clone());
            }

            let identify_packet =
                serde_json::to_string(&ClientMessage::Identify { user_id }).unwrap();

            let _ = tx.send(identify_packet);

            let write_task = tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if let Err(_) = write.send(Message::Text(msg.into())).await {
                        break;
                    }
                }
            });

            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&text) {
                        let mut store = store.lock().await;
                        update_client_store(&mut store, &server_msg);
                    }

                    let _ = app_handle.emit("ws-event", text.to_string());
                }
            }

            write_task.abort();
        }
        Err(e) => eprintln!("Connection Error: {}", e),
    }
}

fn update_client_store(store: &mut ClientStore, msg: &ServerMessage) {
    match msg {
        ServerMessage::InitialState {
            connected_users,
            guilds,
        } => {
            store.guilds = guilds.clone();
            for user in connected_users {
                store.user_cache.insert(user.id.clone(), user.clone());
            }
        }
        ServerMessage::PresenceUpdate { user, status: _ } => {
            store.user_cache.insert(user.id.clone(), user.clone());
        }
        ServerMessage::IdentityValidated { user } => {
            store.current_user = Some(user.clone());
        }
        _ => {}
    }
}
