use shared::structures::Guild;
use tauri::State;

use crate::client_state::ClientState;

pub async fn get_guilds(state: State<'_, ClientState>) -> Result<Vec<Guild>, String> {
    let guilds = Vec::new();

    let _token = {
        let store = state.store.lock().await;

        store.jwt_token.clone().ok_or("No active session")
    };

    Ok(guilds)
}
