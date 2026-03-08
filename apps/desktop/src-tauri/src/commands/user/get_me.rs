use shared::structures::User;
use tauri::State;

use crate::{client_state::ClientState, structures::error::AppError, API_URL};

#[tauri::command]
pub async fn get_me(state: State<'_, ClientState>) -> Result<User, AppError> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or(AppError::NoSession)?
    };

    let res = state
        .http
        .get(format!("{}/users/@me", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(AppError::Internal(format!(
            "Failed to fetch user data. Status: {}",
            res.status()
        )));
    }

    let user: User = res
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(user)
}
