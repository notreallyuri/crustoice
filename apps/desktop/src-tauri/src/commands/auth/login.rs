use crate::API_URL;
use serde_json::json;
use shared::{
    http::{requests::LoginRequest, responses::AuthResponse},
    structures::UserId,
};
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

use crate::client_state::ClientState;

#[tauri::command]
pub async fn login(
    payload: LoginRequest,
    state: State<'_, ClientState>,
    app_handle: AppHandle,
) -> Result<UserId, String> {
    let client = reqwest::Client::new();

    print!("Attempting to log in with email: {}", payload.email);

    let res = client
        .post(format!("{}/auth/login", API_URL))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Login failed with status: {}", res.status()));
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

    Ok(auth_data.user_id)
}
