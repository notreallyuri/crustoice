use crate::{client_state::ClientState, API_URL};
use shared::structures::Guild;
use tauri::State;

#[tauri::command]
pub async fn get_guilds(state: State<'_, ClientState>) -> Result<Vec<Guild>, String> {
    let token = {
        let store = state.store.lock().await;

        store.jwt_token.clone().ok_or("No active session")?
    };

    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/user/@me/guilds", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Failed to fetch guilds. Status: {}", res.status()));
    }

    let guilds: Vec<Guild> = res
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(guilds)
}
