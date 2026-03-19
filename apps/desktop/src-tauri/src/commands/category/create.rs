use crate::{client_state::ClientState, structures::error::AppError, API_URL};
use shared::{http::prelude::CreateCategoryRequest, structures::prelude::ChannelCategory};
use tauri::State;

#[tauri::command]
pub async fn create_category(
    guild_id: String,
    payload: CreateCategoryRequest,
    state: State<'_, ClientState>,
) -> Result<ChannelCategory, AppError> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or(AppError::NoSession)?
    };

    let res = state
        .http
        .post(format!("{}/guilds/{}/categories", API_URL, guild_id))
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(AppError::from_res(res, "Create category").await);
    }

    let category: ChannelCategory = res
        .json()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(category)
}
