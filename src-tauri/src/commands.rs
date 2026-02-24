// Clipboard commands
#[tauri::command]
pub fn read_clipboard_text(clipboard: tauri::State<'_, tauri_plugin_clipboard_manager::Clipboard<R>>) -> Result<String, String> {
    clipboard.read_text().map_err(|e: arboard::Error| e.to_string())
}

#[tauri::command]
pub fn write_clipboard_text(clipboard: tauri::State<'_, tauri_plugin_clipboard_manager::Clipboard<R>>, text: String) -> Result<(), String> {
    clipboard.write_text(text).map_err(|e: arboard::Error| e.to_string())
}

// Print command
#[tauri::command]
pub async fn print_page(window: tauri::WebviewWindow) -> Result<(), String> {
    window.print().map_err(|e| e.to_string())
}