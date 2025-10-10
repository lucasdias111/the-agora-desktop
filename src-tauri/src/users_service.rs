#[tauri::command]
pub async fn get_all_users_for_server() -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/users/get_all_users")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let users: Vec<String> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(users)
}