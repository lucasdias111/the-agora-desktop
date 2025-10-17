mod login_service;
mod message_service;
mod websocket;
mod users_service;


use login_service::{get_stored_token, login};
use message_service::{send_message_to_user, get_message_history_for_user};
use users_service::get_all_users_for_server;
use std::sync::Arc;
use tokio::sync::Mutex;
use websocket::{connect_websocket, WsSender};

use crate::login_service::AuthState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let ws_sender: WsSender = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .manage(ws_sender)
        .manage(AuthState {
            token: Mutex::new(None),
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_message_history_for_user,
            get_all_users_for_server,
            connect_websocket,
            send_message_to_user,
            login,
            get_stored_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
