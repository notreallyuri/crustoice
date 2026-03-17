use shared::{http::requests::prelude::CreateChannelRequest, structures::prelude::Channel};
use tauri::State;

use crate::{client_state::ClientState, structures::error::AppError};

#[tauri::command]
pub async fn create_channel(
    guild_id: String,
    payload: CreateChannelRequest,
    state: State<'_, ClientState>,
) -> Result<Channel, AppError> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or("Not authenticated")?
    };

    let res = state
        .http
        .post(format!("{}/guilds/{}/channels", crate::API_URL, guild_id))
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(AppError::from_res(res, "Create channel").await);
    }

    let channel: Channel = res
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(channel)
}
