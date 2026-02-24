//! Platform-specific detection and dispatch for Tauri app.
//! Uses conditional compilation to load OS-specific modules.

use tauri::AppHandle;

/// Initialize platform-specific features.
pub fn init(app: &AppHandle) {
    log::info!("Initializing platform-specific features");
    
    #[cfg(target_os = "macos")]
    macos::init(app);

    #[cfg(target_os = "windows")]
    windows::init(app);

    #[cfg(target_os = "linux")]
    linux::init(app);
}

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;

/// Stub for unsupported platforms.
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn set_dock_badge(_count: u32) {
    log::warn!("Dock badge not supported on this platform");
}

/// Stub for unsupported platforms.
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn bounce_dock(_critical: bool) {
    log::warn!("Dock bounce not supported on this platform");
}

/// Stub for unsupported platforms.
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn set_taskbar_badge(_count: u32) {
    log::warn!("Taskbar badge not supported on this platform");
}

/// Stub for unsupported platforms.
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn show_toast_notification(_title: &str, _body: &str) {
    log::warn!("Toast notifications not supported on this platform");
}

/// Stub for unsupported platforms.
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn send_dbus_notification(_title: &str, _body: &str, _icon: &str) {
    log::warn!("DBus notifications not supported on this platform");
}

/// Stub for unsupported platforms.
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn generate_desktop_file(_app_name: &str, _exec_path: &str) {
    log::warn!("Desktop file generation not supported on this platform");
}