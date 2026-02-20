use crate::{client_state::ClientState, API_URL};
use shared::structures::{Guild, GuildId};
use tauri::State;

#[tauri::command]
pub async fn create_guild(
    name: String,
    owner_id: String,
    state: State<'_, ClientState>,
) -> Result<Guild, String> {
    let client = reqwest::Client::new();

    let res = client
        .post(format!("{}/guilds", API_URL))
        .json(&serde_json::json!({ "name": name, "owner_id": owner_id }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Failed to create guild: {}", res.status()));
    }

    let guild: Guild = res.json().await.map_err(|e| e.to_string())?;

    {
        let mut store = state.store.lock().await;
        store.guilds.push(guild.clone());

        store.active_guild_id = Some(guild.id.clone());
        if let Some(first_channel) = guild.channels.first() {
            store.active_channel_id = Some(first_channel.id.clone());
        }
    }

    Ok(guild)
}

#[tauri::command]
pub async fn delete_guild(guild_id: String, state: State<'_, ClientState>) -> Result<(), String> {
    let client = reqwest::Client::new();

    let res = client
        .delete(format!("{}/guilds/{}", API_URL, guild_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Failed to delete guild: {}", res.status()));
    }

    {
        let mut store = state.store.lock().await;
        let g_id = GuildId(guild_id);

        store.guilds.retain(|g| g.id != g_id);

        if store.active_guild_id == Some(g_id) {
            store.active_guild_id = None;
            store.active_channel_id = None;
        }
    }

    Ok(())
}
