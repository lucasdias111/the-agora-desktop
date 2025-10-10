use tauri::State;
use tokio::sync::Mutex;

pub struct AuthState {
    pub token: Mutex<Option<String>>,
}

#[tauri::command]
pub async fn login(username: String, password: String, auth_state: State<'_, AuthState>) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8080/auth/login")
        .json(&serde_json::json!({
            "username": username,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    print!("Loggin in");

    if response.status().is_success() {
        let auth_response: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
        let token = auth_response["token"].as_str().ok_or("No token in response")?;
        
        // Store token in app state
        *auth_state.token.lock().await = Some(token.to_string());
        
        Ok("Login successful".to_string())
    } else {
        Err("Login failed".to_string())
    }
}

#[tauri::command]
pub async fn get_stored_token(auth_state: State<'_, AuthState>) -> Result<Option<String>, String> {
    Ok(auth_state.token.lock().await.clone())
}