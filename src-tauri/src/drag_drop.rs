//! Drag and drop handler for Tauri app.
//! Injects dropped files into messenger.com's file input using DataTransfer API.

use tauri::WebviewWindow;
use std::path::PathBuf;
use tauri::Manager;

/// Handle file drop event.
/// - `window`: WebviewWindow where the drop occurred.
/// - `paths`: List of dropped file paths.
pub fn handle_drop(window: &WebviewWindow, paths: Vec<PathBuf>) {
    for path in paths {
        if let Err(e) = inject_file_to_messenger(window, &path) {
            log::error!("Failed to inject file: {}", e);
        }
    }
}

/// Inject a file into messenger.com's file input.
/// Uses JavaScript DataTransfer API to simulate file input.
/// - `window`: WebviewWindow to inject into.
/// - `path`: Path to the file.
fn inject_file_to_messenger(window: &WebviewWindow, path: &PathBuf) -> Result<(), String> {
    let path_str = path.to_string_lossy().to_string();
    let file_name = path.file_name()
        .ok_or("Invalid file name")?
        .to_string_lossy()
        .to_string();
    
    // JavaScript snippet to inject the file into messenger.com's file input
    let js = format!(
        r#"
        (() => {{
            const fileInput = document.querySelector('input[type="file"]');
            if (!fileInput) {{
                console.error('File input not found');
                return;
            }}
            
            const dataTransfer = new DataTransfer();
            const file = new File([], '{}', {{ type: 'application/octet-stream' }});
            Object.defineProperty(file, 'path', {{ value: '{}' }});
            dataTransfer.items.add(file);
            
            fileInput.files = dataTransfer.files;
            fileInput.dispatchEvent(new Event('change', {{ bubbles: true }}));
        }})();
        "#,
        file_name, path_str
    );
    
    window.eval(&js).map_err(|e| e.to_string())
}

/// Listen for drag-drop events in the window.
pub fn setup_drag_drop_handler(window: &WebviewWindow) {
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::DragDrop(event) = event {
            if let tauri::DragDropEvent::Dropped { paths, .. } = *event {
                handle_drop(window, paths);
            }
        }
    });
}

/// Tauri command: Handle file drop.
#[tauri::command]
pub fn handle_file_drop(state: tauri::State<AppHandle>, paths: Vec<String>) {
    if let Some(window) = state.get_webview_window("main") {
        let paths: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
        handle_drop(&window, paths);
    }
}