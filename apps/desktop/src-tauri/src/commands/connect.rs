use shared::structures::UserId;

use crate::{client_state::ClientState, network};

#[tauri::command]
pub async fn connect(
    user_id: UserId,
    state: tauri::State<'_, ClientState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let store_handle = state.store.clone();
    let ws_sender_handle = state.ws_sender.clone();

    tokio::spawn(async move {
        network::connect_to_server(app_handle, user_id, store_handle, ws_sender_handle).await;
    });

    Ok(())
}
