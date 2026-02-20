use crate::{client_state::ClientState, API_URL};
use shared::structures::Guild;
use tauri::State;

#[tauri::command]
pub async fn get_guilds(
    user_id: String,
    state: State<'_, ClientState>,
) -> Result<Vec<Guild>, String> {
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/user/{}/guilds", API_URL, user_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Failed to fetch guilds: {}", res.status()));
    }

    let guilds: Vec<Guild> = res.json().await.map_err(|e| e.to_string())?;

    {
        let mut store = state.store.lock().await;
        store.guilds = guilds.clone();
    }

    Ok(guilds)
}
