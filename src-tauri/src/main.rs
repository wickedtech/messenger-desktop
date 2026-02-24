#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri_plugin_log::LogTarget;
use log::LevelFilter;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // Initialize logging
      let log_plugin = tauri_plugin_log::Builder::default()
        .targets([
          LogTarget::LogDir,
          LogTarget::Stdout,
          LogTarget::Webview,
        ])
        .level(LevelFilter::Info)
        .build();
      
      app.handle().plugin(log_plugin)?;
      
      // Initialize plugins
      app.handle().plugin(tauri_plugin_store::Builder::default().build())?;
      app.handle().plugin(tauri_plugin_autostart::init(
        tauri_plugin_autostart::MacosLauncher::LaunchAgent,
        Some(vec!["--hidden"]),
      ))?;
      app.handle().plugin(tauri_plugin_notification::init())?;
      app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;
      
      // Create main window
      let window = tauri::WindowBuilder::new(
        app,
        "main",
        tauri::WindowUrl::App("index.html".into()),
      )
      .title("Messenger Desktop")
      .inner_size(800.0, 600.0)
      .min_inner_size(400.0, 300.0)
      .build()?;
      
      // Set window visibility
      #[cfg(debug_assertions)]
      window.open_devtools();
      
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      // Add your invoke handlers here
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}