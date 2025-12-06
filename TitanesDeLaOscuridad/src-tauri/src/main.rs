use tauri::Manager;
#[tauri::command]
fn warperia_login(user: String, pass: String) -> bool { user.len()>0 && pass.len()>0 }
#[tauri::command]
fn list_servers() -> serde_json::Value { serde_json::json!({"servers":[{"id":"warperia","name":"Warperia","status":"online"}]}) }
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![warperia_login, list_servers])
    .run(tauri::generate_context!())
    .expect("run error");
}
