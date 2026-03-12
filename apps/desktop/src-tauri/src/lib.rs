pub mod client_state;
pub mod commands;
pub mod structures;
pub mod util;

pub use commands::*;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

const API_URL: &str = "http://127.0.0.1:3000/api";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            if let Ok(store) = app_handle.store("auth.json") {
                let token = store
                    .get("jwt_token")
                    .and_then(|v| v.as_str().map(|s| s.to_string()));
                let user_id = store
                    .get("user_id")
                    .and_then(|v| v.as_str().map(|s| s.to_string()));

                let client_state = app.state::<crate::client_state::ClientState>();

                tauri::async_runtime::block_on(async move {
                    let mut guard = client_state.store.lock().await;
                    guard.jwt_token = token;
                    guard.user_id = user_id;

                    if guard.jwt_token.is_some() {
                        println!("Session successfully hydrated from disk on startup!");
                    }
                });
            }

            Ok(())
        })
        .manage(crate::client_state::ClientState::new())
        .invoke_handler(tauri::generate_handler![
            auth::prelude::close_splashscreen,
            auth::prelude::check_auth,
            auth::prelude::get_token,
            auth::prelude::login,
            auth::prelude::logout,
            auth::prelude::register,
            user::prelude::get_guilds,
            user::prelude::get_me,
            user::prelude::leave_guild,
            user::prelude::update_profile,
            user::prelude::update_username,
            user::prelude::update_email,
            user::prelude::change_password,
            guild::prelude::create_guild,
            channel::prelude::get_channel_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
