use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

use crate::{client_state::ClientState, structures::error::AppError};

#[tauri::command]
pub async fn logout(state: State<'_, ClientState>, app_handle: AppHandle) -> Result<(), AppError> {
    {
        let store = app_handle.store("auth.json").map_err(|e| e.to_string())?;

        store.delete("jwt_token");
        store.delete("user_id");

        store.save().map_err(|e| e.to_string())?;
    }

    {
        let mut store_guard = state.store.lock().await;
        store_guard.jwt_token = None;
    }

    Ok(())
}
