use crate::{client_state::ClientState, structures::error::AppError, API_URL};
use shared::structures::Guild;
use tauri::State;

#[tauri::command]
pub async fn get_guilds(state: State<'_, ClientState>) -> Result<Vec<Guild>, AppError> {
    let token = {
        let store = state.store.lock().await;

        store.jwt_token.clone().ok_or(AppError::NoSession)?
    };

    let res = state
        .http
        .get(format!("{}/users/@me/guilds", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(AppError::Internal(format!(
            "Failed to fetch guilds. Status: {}",
            res.status()
        )));
    }

    let guilds: Vec<Guild> = res
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(guilds)
}
