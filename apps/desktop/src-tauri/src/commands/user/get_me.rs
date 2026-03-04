use shared::structures::User;
use tauri::State;

use crate::{client_state::ClientState, API_URL};

#[tauri::command]
pub async fn get_me(state: State<'_, ClientState>) -> Result<User, String> {
    let token = {
        let store = state.store.lock().await;
        store
            .jwt_token
            .clone()
            .ok_or("No active session".to_string())?
    };

    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/user/@me", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(format!(
            "Failed to fetch user data. Status: {}",
            res.status()
        ));
    }

    let user: User = res
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(user)
}
