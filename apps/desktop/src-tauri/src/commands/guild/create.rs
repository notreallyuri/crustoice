use crate::{
    client_state::ClientState, general::upload::upload_internal, structures::error::AppError,
    API_URL,
};
use shared::{http::requests::CreateGuildRequest, structures::Guild};
use tauri::State;

#[tauri::command]
pub async fn create_guild(
    payload: CreateGuildRequest,
    state: State<'_, ClientState>,
    icon_path: Option<String>,
) -> Result<Guild, AppError> {
    let token = {
        let store = state.store.lock().await;
        store
            .jwt_token
            .clone()
            .ok_or("No active session".to_string())?
    };

    let res = state
        .http
        .post(format!("{}/guilds", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(AppError::from_res(res, "Guild Create").await);
    }

    let data: Guild = res
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    if let Some(path) = icon_path {
        let ext = std::path::Path::new(&path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("png")
            .to_string();

        let id_str = data.id.0.to_string();

        upload_internal(&state, "guild", &id_str, &ext, &path).await?;
    }

    Ok(data)
}
