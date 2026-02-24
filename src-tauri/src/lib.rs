use tauri::Manager;

mod notifications;

use notifications::*;

#[tauri::command]
async fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--flag".to_owned()])))
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .manage(std::sync::Mutex::new(0i32)) // unread_count
    .manage(std::sync::Mutex::new(false)) // dnd_enabled
    .invoke_handler(tauri::generate_handler![
      greet,
      show_notification,
      toggle_dnd,
      set_dnd,
      is_dnd_enabled,
      set_notification_sound,
    ])
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      
      // Configure WebView
      window.eval(&format!(
        "window.__TAURI__.invoke('plugin:notification|is_permission_granted')"
      ))?;
      
      // Set user agent to Chrome to avoid Messenger blocking WebView
      window.eval("navigator.userAgent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'")?;
      
      // Enable cookie persistence
      window.eval("window.__TAURI__.invoke('plugin:store|load')")?;
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
