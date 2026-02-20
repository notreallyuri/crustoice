use crate::{client_state::ClientState, API_URL};
use shared::structures::{CategoryId, ChannelId, GuildId, MessageChannel};
use tauri::State;

#[tauri::command]
pub async fn create_channel(
    guild_id: String,
    name: String,
    category_id: Option<CategoryId>,
    state: State<'_, ClientState>,
) -> Result<MessageChannel, String> {
    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "name": name,
        "category_id": category_id
    });

    let res = client
        .post(format!("{}/guilds/{}/channels", API_URL, guild_id))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Failed to create channel: {}", res.status()));
    }

    let channel: MessageChannel = res.json().await.map_err(|e| e.to_string())?;

    {
        let mut store = state.store.lock().await;
        let g_id = GuildId(guild_id);

        if let Some(guild) = store.guilds.iter_mut().find(|g| g.id == g_id) {
            guild.channels.push(channel.clone());
        }
    }

    Ok(channel)
}

#[tauri::command]
pub async fn delete_channel(
    channel_id: String,
    state: State<'_, ClientState>,
) -> Result<(), String> {
    let client = reqwest::Client::new();

    let res = client
        .delete(format!("{}/channels/{}", API_URL, channel_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Failed to delete channel: {}", res.status()));
    }

    {
        let mut store = state.store.lock().await;
        let c_id = ChannelId(channel_id);

        for guild in store.guilds.iter_mut() {
            if guild.channels.iter().any(|c| c.id == c_id) {
                guild.channels.retain(|c| c.id != c_id);
                break;
            }
        }

        if store.active_channel_id == Some(c_id) {
            store.active_channel_id = None;
        }
    }

    Ok(())
}
