use crate::{client_state::ClientState, structures::error::AppError, API_URL};
use shared::structures::channel::Message;
use tauri::State;

#[tauri::command]
pub async fn get_channel_history(
    state: State<'_, ClientState>,
    channel_id: String,
    before: Option<i64>,
) -> Result<Vec<Message>, AppError> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or(AppError::NoSession)?
    };

    let mut url = format!("{}/channels/{}/history?limit=50", API_URL, channel_id);
    if let Some(before_ms) = before {
        url.push_str(&format!("&before={}", before_ms));
    }

    let res = state
        .http
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(AppError::Internal(format!(
            "Failed to fetch history. Status: {}",
            res.status()
        )));
    }

    let messages: Vec<Message> = res
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(messages)
}
