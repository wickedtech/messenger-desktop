/// Read text from the clipboard.
#[tauri::command]
pub fn read_clipboard_text(clipboard: tauri::State<'_, tauri_plugin_clipboard_manager::Clipboard<tauri::Wry>>) -> Result<String, String> {
    clipboard.read_text().map_err(|e: arboard::Error| e.to_string())
}

/// Write text to the clipboard.
#[tauri::command]
pub fn write_clipboard_text(clipboard: tauri::State<'_, tauri_plugin_clipboard_manager::Clipboard<tauri::Wry>>, text: String) -> Result<(), String> {
    clipboard.write_text(text).map_err(|e: arboard::Error| e.to_string())
}

/// Print the current page.
#[tauri::command]
pub async fn print_page(window: tauri::WebviewWindow) -> Result<(), String> {
    window.print().map_err(|e| e.to_string())
}