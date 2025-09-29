use crate::WsSender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct WebSocketMessage {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "toUserId")]
    to_user_id: String,
    message: String,
}

#[tauri::command]
pub async fn send_message_to_user(
    to_user_id: String,
    message_text: String,
    sender: tauri::State<'_, WsSender>,
) -> Result<String, String> {
    let message = WebSocketMessage {
        message_type: "SEND_MESSAGE".to_string(),
        to_user_id,
        message: message_text,
    };

    println!("Sending message: {:?}", message);

    let json_message = serde_json::to_string(&message)
        .map_err(|e| format!("Failed to serialize message: {}", e))?;

    if let Some(tx) = sender.lock().await.as_ref() {
        tx.send(json_message).map_err(|e| e.to_string())?;
        Ok("Message sent".to_string())
    } else {
        Err("WebSocket is not connected".to_string())
    }
}
