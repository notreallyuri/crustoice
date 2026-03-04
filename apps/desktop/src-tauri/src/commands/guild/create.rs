use crate::{client_state::ClientState, API_URL};
use shared::{requests::CreateGuildRequest, structures::Guild};
use tauri::State;

#[tauri::command]
pub async fn create_guild(
    payload: CreateGuildRequest,
    state: State<'_, ClientState>,
) -> Result<Guild, String> {
    let token = {
        let store = state.store.lock().await;
        store
            .jwt_token
            .clone()
            .ok_or("No active session".to_string())?
    };

    let client = reqwest::Client::new();

    print!("Attempting to create guild with name: {}", payload.name);

    let res = client
        .post(format!("{}/guilds", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!(
            "Failed to create guild with status: {}",
            res.status()
        ));
    }

    let data: Guild = res
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(data)
}
