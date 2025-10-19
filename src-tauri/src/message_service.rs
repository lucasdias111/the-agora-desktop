use crate::{login_service::AuthState, WsSender};
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::websocket::ChatMessage;

#[derive(Debug, Serialize, Deserialize)]
struct WebSocketMessage {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "toUserId")]
    to_user_id: i64,
    message: String,
}

#[tauri::command]
pub async fn send_message_to_user(
    to_user_id: i64,
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

#[tauri::command]
pub async fn get_message_history_for_user(
    from_user_id: i64,
    to_user_id: i64,
    auth_state: State<'_, AuthState>
) -> Result<Vec<ChatMessage>, String> {
    let token = auth_state.token.lock().await.clone()
        .ok_or("No token found")?;

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/chat_messages/get_chat_history")
        .header("Authorization", format!("Bearer {}", token))
        .query(&[
            ("userId", from_user_id.to_string()),
            ("toUserId", to_user_id.to_string())
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let messages: Vec<ChatMessage> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(messages)
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}