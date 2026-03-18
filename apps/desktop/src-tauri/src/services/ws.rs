use futures_util::{SinkExt, StreamExt};
use shared::protocol::{ClientMessage, ServerMessage};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::WS_URL;

pub async fn start_ws_client(
    app: AppHandle,
    token: String,
    mut cmd_rx: mpsc::UnboundedReceiver<ClientMessage>,
) {
    loop {
        match connect_async(WS_URL).await {
            Ok((ws_stream, _)) => {
                println!("WS connected");

                let (mut ws_sink, mut ws_source) = ws_stream.split();

                let identify = ClientMessage::Identify {
                    token: token.clone(),
                };
                let identify_json = match serde_json::to_string(&identify) {
                    Ok(j) => j,
                    Err(e) => {
                        eprintln!("Failed to serialize identify: {}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                        continue;
                    }
                };

                if ws_sink
                    .send(Message::Text(identify_json.into()))
                    .await
                    .is_err()
                {
                    eprintln!("Failed to send identify");
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    continue;
                }

                let (conn_tx, mut conn_rx) = mpsc::unbounded_channel::<String>();

                let send_task = tokio::spawn(async move {
                    while let Some(raw) = conn_rx.recv().await {
                        if ws_sink.send(Message::Text(raw.into())).await.is_err() {
                            break;
                        }
                    }
                });

                loop {
                    tokio::select! {
                        msg = ws_source.next() => {
                            match msg {
                                Some(Ok(Message::Text(text))) => {
                                    match serde_json::from_str::<ServerMessage>(&text) {
                                        Ok(server_msg) => {
                                            let _ = app.emit("ws://message", &server_msg);
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to parse WS message: {}", e);
                                        }
                                    }
                                }
                                Some(Ok(Message::Close(_))) => {
                                    println!("WS closed by server");
                                    break;
                                }
                                Some(Err(e)) => {
                                    eprintln!("WS error: {}", e);
                                    break;
                                }
                                None => break,
                                _ => {}
                            }
                        }

                        cmd = cmd_rx.recv() => {
                            match cmd {
                                Some(msg) => {
                                    match serde_json::to_string(&msg) {
                                        Ok(json) => { let _ = conn_tx.send(json); }
                                        Err(e) => eprintln!("Failed to serialize command: {}", e),
                                    }
                                }
                                None => {
                                    send_task.abort();
                                    return; // Logout — exit entirely
                                }
                            }
                        }
                    }
                }

                send_task.abort();
                println!("WS disconnected, reconnecting in 3s...");

                if cmd_rx.is_closed() {
                    return;
                }
            }
            Err(e) => {
                eprintln!("WS connection failed: {}", e);
                if cmd_rx.is_closed() {
                    return;
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}
