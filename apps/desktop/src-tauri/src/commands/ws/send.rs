use crate::{client_state::ClientState, structures::error::AppError};
use shared::protocol::ClientMessage;
use tauri::State;

#[tauri::command]
pub async fn ws_send(
    state: State<'_, ClientState>,
    message: ClientMessage,
) -> Result<(), AppError> {
    let tx = state.ws_tx.lock().await;
    if let Some(tx) = tx.as_ref() {
        tx.send(message)
            .map_err(|e| AppError::Internal(format!("WS send failed: {}", e)))?;
        Ok(())
    } else {
        Err(AppError::Internal("WebSocket not connected".to_string()))
    }
}
