use serde::{Deserialize, Serialize};
use tauri::State;
use crate::login_service::{get_stored_token, AuthState};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[tauri::command]
pub async fn get_all_users_for_server() -> Result<Vec<User>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/users/get_all_users")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let users: Vec<User> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(users)
}

#[tauri::command]
pub async fn get_current_user(auth_state: State<'_, AuthState>) -> Result<User, String> {
    let token = auth_state.token.lock().await.clone()
        .ok_or("No token found. Please login first.")?;

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/users/me")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let user: User = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(user)
}