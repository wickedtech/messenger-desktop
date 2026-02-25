/// debug.rs — Developer Tools commands for Messenger Desktop
///
/// DevTools are always available because `tauri = { features = ["devtools"] }` is set.
/// In debug builds they auto-open on startup (see lib.rs setup).
///
/// Frontend usage (TypeScript):
///   import { invoke } from '@tauri-apps/api/core';
///   await invoke('open_devtools');
///   await invoke('close_devtools');
///   await invoke('toggle_devtools');
///   const open: boolean = await invoke('is_devtools_open');
///
/// Keyboard shortcut: press F12 or trigger `toggle_devtools` from the frontend.

use tauri::WebviewWindow;
use tracing::debug;

/// Open the webview DevTools inspector on the given window.
#[tauri::command]
pub fn open_devtools(window: WebviewWindow) {
    debug!("[devtools] open_devtools → window '{}'", window.label());
    window.open_devtools();
}

/// Close the webview DevTools inspector on the given window.
#[tauri::command]
pub fn close_devtools(window: WebviewWindow) {
    debug!("[devtools] close_devtools → window '{}'", window.label());
    window.close_devtools();
}

/// Toggle DevTools — opens if closed, closes if open.
#[tauri::command]
pub fn toggle_devtools(window: WebviewWindow) {
    if window.is_devtools_open() {
        debug!("[devtools] toggle → closing");
        window.close_devtools();
    } else {
        debug!("[devtools] toggle → opening");
        window.open_devtools();
    }
}

/// Returns `true` if DevTools are currently open on the given window.
#[tauri::command]
pub fn is_devtools_open(window: WebviewWindow) -> bool {
    let open = window.is_devtools_open();
    debug!("[devtools] is_devtools_open = {}", open);
    open
}
