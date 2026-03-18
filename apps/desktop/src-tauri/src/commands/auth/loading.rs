use shared::structures::ids::UserId;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_store::StoreExt;

use crate::{client_state::ClientState, structures::error::AppError};

#[tauri::command]
pub async fn close_splashscreen(app_handle: AppHandle) -> Result<(), AppError> {
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
pub async fn get_token(state: State<'_, ClientState>) -> Result<String, AppError> {
    let store = state.store.lock().await;
    store.jwt_token.clone().ok_or(AppError::NoSession)
}

#[tauri::command]
pub async fn check_auth(
    state: State<'_, ClientState>,
    app_handle: AppHandle,
) -> Result<UserId, AppError> {
    let store = app_handle.store("auth.json").map_err(|e| {
        println!("Failed to open store: {}", e);
        e.to_string()
    })?;

    let token_val = store.get("jwt_token").ok_or_else(|| {
        println!("No jwt_token found in store. User needs to log in.");
        "No token found in store".to_string()
    })?;

    let user_id_val = store.get("user_id").ok_or_else(|| {
        println!("No user_id found in store.");
        "No user ID found in store".to_string()
    })?;

    let token = token_val
        .as_str()
        .ok_or_else(|| {
            println!("Token is not a valid string");
            "Invalid token format".to_string()
        })?
        .to_string();

    let user_id_str = user_id_val
        .as_str()
        .ok_or_else(|| {
            println!("User ID is not a valid string");
            "Invalid user ID format".to_string()
        })?
        .to_string();

    {
        let mut store_guard = state.store.lock().await;
        store_guard.jwt_token = Some(token.clone());
    }

    state.connect_ws(app_handle.clone(), token).await;

    Ok(UserId(user_id_str))
}
