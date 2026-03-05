use crate::{client_state::ClientState, general::upload::upload_internal, API_URL};
use serde_json::json;
use shared::{
    requests::{AuthResponse, RegisterRequest},
    structures::UserId,
};
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub async fn register(
    payload: RegisterRequest,
    state: State<'_, ClientState>,
    avatar_path: Option<String>,
    app_handle: AppHandle,
) -> Result<UserId, String> {
    let client = reqwest::Client::new();

    let res = client
        .post(format!("{}/auth/register", API_URL))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Registration failed with status: {}", res.status()));
    }

    let auth_data: AuthResponse = res
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    {
        let store = app_handle.store("auth.json").map_err(|e| e.to_string())?;
        store.set("jwt_token", json!(auth_data.token));
        store.set("user_id", json!(auth_data.user_id.0));
        store.save().map_err(|e| e.to_string())?;
    }

    {
        let mut store_guard = state.store.lock().await;
        store_guard.jwt_token = Some(auth_data.token);
    }

    if let Some(path) = avatar_path {
        let ext = std::path::Path::new(&path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("png")
            .to_string();

        let id_str = auth_data.user_id.0.to_string();

        upload_internal(&state, "avatar", &id_str, &ext, &path).await?;
    }

    Ok(auth_data.user_id)
}
