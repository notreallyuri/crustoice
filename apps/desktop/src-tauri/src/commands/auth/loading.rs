use shared::structures::UserId;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_store::StoreExt;

use crate::client_state::ClientState;

#[tauri::command]
pub async fn close_splashscreen(app_handle: AppHandle) -> Result<(), String> {
    if let Some(splashscreen) = app_handle.get_webview_window("splashscreen") {
        splashscreen.close().map_err(|e| e.to_string())?;
    }

    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window.show().map_err(|e| e.to_string())?;
        main_window.set_focus().map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn check_auth(
    state: State<'_, ClientState>,
    app_handle: AppHandle,
) -> Result<UserId, String> {
    let store = app_handle.store("auth.json").map_err(|e| e.to_string())?;

    let token_val = store.get("jwt_token").ok_or("No token found in store")?;
    let user_id_val = store.get("user_id").ok_or("No user ID found in store")?;

    let token = token_val
        .as_str()
        .ok_or("Invalid token format")?
        .to_string();
    let user_id_str = user_id_val
        .as_str()
        .ok_or("Invalid user ID format")?
        .to_string();

    {
        let mut store_guard = state.store.lock().await;
        store_guard.jwt_token = Some(token);
    }

    Ok(UserId(user_id_str))
}
