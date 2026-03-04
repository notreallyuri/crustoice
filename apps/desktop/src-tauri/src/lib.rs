pub mod client_state;
pub mod commands;

pub use commands::*;

const API_URL: &str = "http://127.0.0.1:3000/api";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|_app| Ok(()))
        .manage(crate::client_state::ClientState::default())
        .invoke_handler(tauri::generate_handler![
            auth::loading::close_splashscreen,
            auth::loading::check_auth,
            auth::login::login,
            auth::register::register,
            user::get_me::get_me,
            guild::create::create_guild,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
