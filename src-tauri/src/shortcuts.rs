use tauri::AppHandle;
use tauri::Manager;
use std::sync::Mutex;
use std::collections::HashMap;

pub struct ShortcutManager {
    registered: HashMap<String, String>, // action -> keys
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self {
            registered: HashMap::new(),
        }
    }

    pub fn register_all(app: &AppHandle) -> Result<(), String> {
        let manager = Self::new();
        app.manage(Mutex::new(manager));
        // Register default shortcuts via the plugin
        // Note: actual shortcut registration requires tauri-plugin-global-shortcut
        // which needs runtime setup in lib.rs via .plugin()
        Ok(())
    }

    #[allow(dead_code)]
    pub fn unregister_all(&self) {
        // Cleanup
    }
}

#[tauri::command]
pub fn register_shortcuts(app: AppHandle) -> Result<(), String> {
    ShortcutManager::register_all(&app)
}

#[tauri::command]
pub fn update_shortcut(
    app: AppHandle,
    action: String,
    keys: String,
) -> Result<(), String> {
    let state = app.state::<Mutex<ShortcutManager>>();
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.registered.insert(action, keys);
    Ok(())
}

#[tauri::command]
pub fn unregister_shortcut(
    app: AppHandle,
    action: String,
) -> Result<(), String> {
    let state = app.state::<Mutex<ShortcutManager>>();
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.registered.remove(&action);
    Ok(())
}

#[tauri::command]
pub fn init_shortcuts(app: AppHandle) -> Result<(), String> {
    ShortcutManager::register_all(&app)
}