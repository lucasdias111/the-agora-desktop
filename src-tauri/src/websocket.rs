use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub type WsSender = Arc<Mutex<Option<mpsc::UnboundedSender<String>>>>;
#[derive(Debug, Deserialize, Serialize, Clone)]
struct WsMessage {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "userJson")]
    user: String,
    timestamp: Option<i64>,
    payload: Option<serde_json::Value>,
}

#[tauri::command]
pub async fn connect_websocket(
    token: String,
    window: tauri::Window,
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
                            let _ = window.emit("ws_login", ws_msg.user);
                        }
                        "USER_LOGOUT" => {
                            let _ = window.emit("ws_logout", ws_msg.user);
                        }
                        "SEND_MESSAGE" => {
                            let _ = window.emit("ws_message_received", &ws_msg);
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
