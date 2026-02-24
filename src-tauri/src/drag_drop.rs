//! Drag and drop handler for Tauri app.
//! Handles file drops, injects files into messenger.com's file input, and validates file types.

use tauri::{WebviewWindow, Manager, Emitter};
use std::path::{Path, PathBuf};
use serde::Serialize;
use mime_guess::from_path;
use log::{info, error};

/// File drop event payload.
#[derive(Serialize, Clone, Debug)]
pub struct FileDropPayload {
    pub files: Vec<FileDropInfo>,
    pub status: String,
    pub error: Option<String>,
}

/// File drop information.
#[derive(Serialize, Clone, Debug)]
pub struct FileDropInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub mime_type: String,
    pub is_image: bool,
    pub is_video: bool,
    pub is_audio: bool,
    pub is_document: bool,
}

/// Handle file drop event.
/// - `window`: WebviewWindow where the drop occurred.
/// - `paths`: List of dropped file paths.
pub fn handle_drop(window: &WebviewWindow, paths: Vec<PathBuf>) -> FileDropPayload {
    let mut files = Vec::new();
    let mut error = None;
    
    for path in paths {
        match process_file(&path) {
            Ok(file_info) => {
                files.push(file_info.clone());
                if let Err(e) = inject_file_to_messenger(window, &file_info) {
                    error = Some(format!("Failed to inject file {}: {}", file_info.name, e));
                    error!("Failed to inject file: {}", e);
                }
            }
            Err(e) => {
                error = Some(format!("Failed to process file {}: {}", path.display(), e));
                error!("Failed to process file: {}", e);
            }
        }
    }
    
    FileDropPayload {
        files,
        status: if error.is_none() { "success" } else { "error" }.to_string(),
        error,
    }
}

/// Process a file and extract metadata.
/// - `path`: Path to the file.
fn process_file(path: &Path) -> Result<FileDropInfo, String> {
    let metadata = std::fs::metadata(path).map_err(|e| e.to_string())?;
    let mime_type = from_path(path).first_or_octet_stream();
    let name = path.file_name()
        .ok_or("Invalid file name")?
        .to_string_lossy()
        .to_string();
    
    Ok(FileDropInfo {
        name,
        path: path.to_string_lossy().to_string(),
        size: metadata.len(),
        mime_type: mime_type.to_string(),
        is_image: mime_type.type_() == "image",
        is_video: mime_type.type_() == "video",
        is_audio: mime_type.type_() == "audio",
        is_document: mime_type.type_() == "application",
    })
}

/// Inject a file into messenger.com's file input.
/// Uses JavaScript DataTransfer API to simulate file input.
/// - `window`: WebviewWindow to inject into.
/// - `file_info`: File information.
fn inject_file_to_messenger(window: &WebviewWindow, file_info: &FileDropInfo) -> Result<(), String> {
    let js = format!(
        r#"
        (() => {{
            const fileInput = document.querySelector('input[type="file"]');
            if (!fileInput) {{
                console.error('File input not found');
                return;
            }}
            
            const dataTransfer = new DataTransfer();
            const file = new File([], '{}', {{ type: '{}' }});
            Object.defineProperty(file, 'path', {{ value: '{}' }});
            dataTransfer.items.add(file);
            
            fileInput.files = dataTransfer.files;
            fileInput.dispatchEvent(new Event('change', {{ bubbles: true }}));
        }})();
        "#,
        file_info.name, file_info.mime_type, file_info.path
    );
    
    window.eval(&js).map_err(|e| e.to_string())
}

/// Listen for drag-drop events in the window.
#[allow(dead_code)]
pub fn setup_drag_drop_handler(window: &WebviewWindow) {
    let window_clone = window.to_owned();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::DragDrop(event) = event {
            match event {
                tauri::DragDropEvent::Drop { paths, .. } => {
                    let payload = handle_drop(&window_clone, paths.to_vec());
                    if let Err(e) = window_clone.emit("file-drop", payload) {
                        error!("Failed to emit file-drop event: {}", e);
                    }
                }
                tauri::DragDropEvent::Enter { paths, .. } => {
                    info!("Drag entered with {} files", paths.to_vec().len());
                }
                tauri::DragDropEvent::Leave => {
                    info!("Drag left");
                }
                _ => {}
            }
        }
    });
}

/// Tauri command: Handle file drop.
#[tauri::command]
pub fn handle_file_drop(state: tauri::State<tauri::AppHandle>, paths: Vec<String>) -> FileDropPayload {
    if let Some(window) = state.get_webview_window("main") {
        let paths: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
        handle_drop(&window, paths)
    } else {
        FileDropPayload {
            files: Vec::new(),
            status: "error".to_string(),
            error: Some("Main window not found".to_string()),
        }
    }
}

/// Tauri command: Validate dropped files.
#[tauri::command]
#[allow(dead_code)]
pub fn validate_files(paths: Vec<String>) -> Vec<FileDropInfo> {
    paths
        .into_iter()
        .filter_map(|path| {
            let path = PathBuf::from(path);
            process_file(&path).ok()
        })
        .collect()
}