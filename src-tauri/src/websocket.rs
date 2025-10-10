use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub type WsSender = Arc<Mutex<Option<mpsc::UnboundedSender<String>>>>;

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
                let _ = window.emit("ws-message", &text);
            }
        }
    });

    Ok("Connected to the websocket server".into())
}
