use shared::{
    protocol::ClientMessage,
    structures::{ChannelId, ChatMessage},
};
use tauri::State;

use crate::{client_state::ClientState, API_URL};

#[tauri::command]
pub async fn send_chat(
    content: String,
    channel_id: ChannelId,
    state: State<'_, ClientState>,
) -> Result<(), String> {
    println!("Send message! Send chat! message: {}", content);

    let packet = serde_json::to_string(&ClientMessage::Chat {
        channel_id,
        content,
    })
    .map_err(|e| e.to_string())?;

    let guard = state.ws_sender.lock().await;
    if let Some(tx) = &*guard {
        tx.send(packet).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_messages(
    channel_id: String,
    state: State<'_, ClientState>,
) -> Result<Vec<ChatMessage>, String> {
    println!("[TAURI] Fetching history for room {}", channel_id);

    let client = reqwest::Client::new();
    let url = format!("{}/channels/{}/history", API_URL, channel_id);

    let res = client.get(url).send().await.map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!(
            "[TAURI ERROR] Failed to fetch messages: {}",
            res.status()
        ));
    }

    let history: Vec<ChatMessage> = res.json().await.map_err(|e| e.to_string())?;

    {
        let mut store = state.store.lock().await;
        let c_id = ChannelId(channel_id);

        store.active_channel_id = Some(c_id.clone());
        store.messages.insert(c_id, history.clone());
    }

    Ok(history)
}
