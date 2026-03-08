use crate::{client_state::ClientState, structures::error::AppError, API_URL};
use tauri::State;

#[tauri::command]
pub async fn leave_guild(guild_id: String, state: State<'_, ClientState>) -> Result<(), AppError> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or(AppError::NoSession)?
    };

    let res = state
        .http
        .delete(format!("{}/users/@me/guilds/leave/{}", API_URL, guild_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(AppError::Internal("Failed to leave guild".to_string()));
    }

    Ok(())
}
