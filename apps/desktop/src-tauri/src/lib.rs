pub mod client_state;
pub mod commands;
pub mod network;

pub use commands::*;

use crate::client_state::ClientState;

const API_URL: &str = "http://127.0.0.1:3000/api";
const WS_URL: &str = "ws://127.0.0.1:3000/ws";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ClientState::default())
        .invoke_handler(tauri::generate_handler![
            chat::send_chat,
            chat::get_messages,
            connect::connect,
            auth::login,
            guilds::create_guild,
            guilds::delete_guild,
            channels::create_channel,
            channels::delete_channel,
            user::get_guilds
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
