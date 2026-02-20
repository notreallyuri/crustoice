use crate::{client_state::ClientState, network, API_URL};
use shared::structures::UserProfile;
use tauri::State;

#[tauri::command]
pub async fn login(
    username: String,
    state: State<'_, ClientState>,
    app_handle: tauri::AppHandle,
) -> Result<UserProfile, String> {
    println!("[TAURI] Logging in as {}", username);

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/auth/login", API_URL))
        .json(&serde_json::json!({ "username": username }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("[TAURI ERROR] Login Failed: {}", res.status()));
    }

    let user: UserProfile = res.json().await.map_err(|e| e.to_string())?;

    {
        let mut store = state.store.lock().await;
        store.current_user = Some(user.clone());
        store.user_cache.insert(user.id.clone(), user.clone());
    }

    let store_handle = state.store.clone();
    let sender_handle = state.ws_sender.clone();
    let user_id = user.id.clone();

    tauri::async_runtime::spawn(async move {
        network::connect_to_server(app_handle, user_id, store_handle, sender_handle).await;
    });

    Ok(user)
}
