//! Linux-specific features for Tauri app.
//! All functions are wrapped in `#[cfg(target_os = "linux")]`.

use tauri::AppHandle;
use std::process::Command;
use std::fs;
use dirs::home_dir;

/// Initialize Linux-specific features.
pub fn init(_app: &AppHandle) {
    log::info!("Initializing Linux platform features");
    // Placeholder for future initialization logic
}

/// Send a notification via DBus (notify-send).
/// - `title`: Notification title.
/// - `body`: Notification body text.
/// - `icon`: Icon name or path.
#[allow(dead_code)]
pub fn send_dbus_notification(title: &str, body: &str, icon: &str) {
    let result = Command::new("notify-send")
        .arg("--app-name=Messenger Desktop")
        .arg(format!("--icon={}", icon))
        .arg(title)
        .arg(body)
        .status();

    if let Err(e) = result {
        log::error!("Failed to send DBus notification: {}", e);
    }
}

/// Generate a desktop entry file for the application.
/// - `app_name`: Application name (e.g., "Messenger Desktop").
/// - `exec_path`: Path to the executable.
#[allow(dead_code)]
pub fn generate_desktop_file(app_name: &str, exec_path: &str) {
    let home = home_dir().expect("Failed to get home directory");
    let desktop_dir = home.join(".local/share/applications");
    let desktop_path = desktop_dir.join(format!(
        "{}.desktop",
        app_name.to_lowercase().replace(" ", "-")
    ));

    let desktop_content = format!(
        "[Desktop Entry]\n\
         Version=1.0\n\
         Type=Application\n\
         Name={}\n\
         Exec={}\n\
         Icon=messenger-desktop\n\
         Terminal=false\n\
         Categories=Network;InstantMessaging;\n",
        app_name, exec_path
    );

    if let Err(e) = fs::create_dir_all(&desktop_dir) {
        log::error!("Failed to create desktop directory: {}", e);
        return;
    }

    if let Err(e) = fs::write(&desktop_path, desktop_content) {
        log::error!("Failed to write desktop file: {}", e);
    } else {
        log::info!("Generated desktop file at: {}", desktop_path.display());
    }
}