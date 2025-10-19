use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{Emitter, Window};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use crate::users_service::User;

pub type WsSender = Arc<Mutex<Option<mpsc::UnboundedSender<String>>>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct WsMessage {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "userJson")]
    user_json: String,
    timestamp: Option<i64>,
    payload: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: Option<i64>,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub message: String,
    pub is_read: bool,
    pub is_edited: bool,
    pub created_at: Option<String>,
}

#[tauri::command]
pub async fn connect_websocket(
    token: String,
    window: Window,
    sender: tauri::State<'_, WsSender>,
) -> Result<String, String> {
    let ws_url = format!("ws://localhost:8081/ws?token={}", token);

    let (ws_stream, _) = connect_async(ws_url).await.map_err(|e| e.to_string())?;
    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    *sender.lock().await = Some(tx);

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let _ = write.send(Message::Text(msg)).await;
        }
    });

    // Receive messages
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            if let Ok(Message::Text(text)) = msg {
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    match ws_msg.msg_type.as_str() {
                        "USER_LOGIN" => {
                            handle_login(&window, &ws_msg);
                        }
                        "USER_LOGOUT" => {
                            handle_logout(&window, &ws_msg);
                        }
                        "SEND_MESSAGE" => {
                            handle_message(&window, &ws_msg);
                        }
                        _ => {
                            println!("Unknown message type: {}", ws_msg.msg_type);
                        }
                    }
                } else {
                    eprintln!("Failed to parse WebSocket message: {}", text);
                }
            }
        }
    });

    Ok("Connected to the websocket server".into())
}

fn handle_message(window: &Window, ws_msg: &WsMessage) {
    if let Some(payload_json) = &ws_msg.payload {
        if let Ok(chat_message) = serde_json::from_str::<ChatMessage>(payload_json) {
            let _ = window.emit("ws_message_received", chat_message);
        } else {
            eprintln!("Failed to parse chat message: {}", payload_json);
        }
    }
}

fn handle_logout(window: &Window, ws_msg: &WsMessage) {
    if let Ok(user) = serde_json::from_str::<User>(&ws_msg.user_json) {
        let _ = window.emit("ws_logout", user);
    } else {
        eprintln!("Failed to parse user JSON: {}", ws_msg.user_json);
    }
}

fn handle_login(window: &Window, ws_msg: &WsMessage) {
    if let Ok(user) = serde_json::from_str::<User>(&ws_msg.user_json) {
        let _ = window.emit("ws_login", user);
    } else {
        eprintln!("Failed to parse user JSON: {}", ws_msg.user_json);
    }
}