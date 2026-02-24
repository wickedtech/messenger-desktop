use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct ShortcutManager {
    registered: Mutex<HashMap<String, Shortcut>>,
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self {
            registered: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_all(app: &AppHandle) -> tauri::Result<()> {
        let shortcut_plugin = app.global_shortcut();
        
        // CommandOrControl+Shift+M: Toggle main window
        Self::register_shortcut(
            app,
            &shortcut_plugin,
            "toggle_window",
            "CommandOrControl+Shift+M",
        )?;

        // CommandOrControl+N: New message
        Self::register_shortcut(
            app,
            &shortcut_plugin,
            "new_message",
            "CommandOrControl+N",
        )?;

        // CommandOrControl+Shift+D: Do Not Disturb
        Self::register_shortcut(
            app,
            &shortcut_plugin,
            "dnd",
            "CommandOrControl+Shift+D",
        )?;

        // F11: Toggle fullscreen
        Self::register_shortcut(
            app,
            &shortcut_plugin,
            "fullscreen",
            "F11",
        )?;

        Ok(())
    }

    fn register_shortcut(
        app: &AppHandle,
        plugin: &impl GlobalShortcutExt,
        action: &str,
        keys: &str,
    ) -> tauri::Result<()> {
        let shortcut: Shortcut = keys.parse().map_err(|e| {
            tauri::Error::InvalidPlugin(tauri::plugin::PluginError::InvalidHandle(format!(
                "Failed to parse shortcut '{}': {}",
                keys,
                e
            )))
        })?;

        let app_handle = app.clone();
        let action_string = action.to_string();

        plugin
            .register(shortcut, move |_app, shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    let _ = app_handle.emit("global-shortcut-pressed", (action_string.clone(), shortcut.shortcut().to_string()));
                }
            })
            .map_err(|e| {
                tauri::Error::InvalidPlugin(tauri::plugin::PluginError::InvalidHandle(format!(
                    "Failed to register shortcut '{}': {}",
                    keys,
                    e
                )))
            })?;

        let manager_state = app.state::<Mutex<Self>>();
        let mut registered = manager_state.lock().map_err(|e| {
            tauri::Error::InvalidPlugin(tauri::plugin::PluginError::InvalidHandle(format!(
                "Failed to lock shortcut manager: {}",
                e
            )))
        })?;

        registered.insert(action.to_string(), shortcut);

        Ok(())
    }

    pub fn update_shortcut(
        app: &AppHandle,
        action: String,
        keys: String,
    ) -> tauri::Result<()> {
        let shortcut_plugin = app.global_shortcut();
        
        // Unregister old shortcut
        let manager_state = app.state::<Mutex<Self>>();
        let mut registered = manager_state.lock().map_err(|e| {
            tauri::Error::InvalidPlugin(tauri::plugin::PluginError::InvalidHandle(format!(
                "Failed to lock shortcut manager: {}",
                e
            )))
        })?;

        if let Some(old_shortcut) = registered.remove(&action) {
            shortcut_plugin.unregister(old_shortcut).map_err(|e| {
                tauri::Error::InvalidPlugin(tauri::plugin::PluginError::InvalidHandle(format!(
                    "Failed to unregister old shortcut for '{}': {}",
                    action,
                    e
                )))
            })?;
        }

        // Register new shortcut
        drop(registered);
        
        Self::register_shortcut(app, &shortcut_plugin, &action, &keys)?;

        Ok(())
    }

    pub fn unregister_all(app: &AppHandle) {
        let shortcut_plugin = app.global_shortcut();
        
        if let Ok(manager_state) = app.state::<Mutex<Self>>() {
            if let Ok(mut registered) = manager_state.lock() {
                for (_, shortcut) in registered.drain() {
                    let _ = shortcut_plugin.unregister(shortcut);
                }
            }
        }
    }
}

// Tauri commands for frontend invocation
#[tauri::command]
pub fn register_shortcuts(app: AppHandle) -> Result<(), String> {
    ShortcutManager::register_all(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_shortcut(
    app: AppHandle,
    action: String,
    keys: String,
) -> Result<(), String> {
    ShortcutManager::update_shortcut(&app, action, keys).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn unregister_shortcut(
    app: AppHandle,
    action: String,
) -> Result<(), String> {
    let shortcut_plugin = app.global_shortcut();
    let manager_state = app.state::<Mutex<ShortcutManager>>();
    let mut registered = manager_state.lock().map_err(|e| e.to_string())?;

    if let Some(shortcut) = registered.remove(&action) {
        shortcut_plugin.unregister(shortcut).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn init_shortcuts(app: AppHandle) -> Result<(), String> {
    let manager = ShortcutManager::new();
    app.manage(Mutex::new(manager));
    ShortcutManager::register_all(&app).map_err(|e| e.to_string())
}
