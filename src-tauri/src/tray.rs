use tauri::{AppHandle, Manager, Emitter, WindowEvent};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, TrayIconId};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use std::sync::Mutex;

const TRAY_ID: &str = "messenger-tray";

pub struct TrayManager {
    app: AppHandle,
}

impl TrayManager {
    pub fn new(app: &AppHandle) -> tauri::Result<Self> {
        let menu = Self::build_menu(app)?;
        
        let _tray = TrayIconBuilder::new()
            .icon(app.default_icon()?.clone())
            .menu(&menu)
            .menu_on_left_click(true)
            .on_menu_event(move |app, event| {
                Self::handle_menu_event(app, event.id.as_ref());
            })
            .on_tray_icon_event(|app, event| {
                Self::handle_event(app, event);
            })
            .build(app)?;

        Ok(Self {
            app: app.clone(),
        })
    }

    fn build_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
        let open_item = MenuItem::with_id(app, "open", "Open Messenger", true, None::<&str>)?;
        let separator1 = PredefinedMenuItem::separator(app)?;
        let new_message_item = MenuItem::with_id(app, "new_message", "New Message", true, None::<&str>)?;
        let mute_item = MenuItem::with_id(app, "mute", "Mute", true, None::<&str>)?;
        let dnd_item = MenuItem::with_id(app, "dnd", "Do Not Disturb", true, None::<&str>)?;
        let separator2 = PredefinedMenuItem::separator(app)?;
        let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
        let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

        Menu::with_items(
            app,
            &[
                &open_item,
                &separator1,
                &new_message_item,
                &mute_item,
                &dnd_item,
                &separator2,
                &settings_item,
                &quit_item,
            ],
        )
    }

    pub fn update_unread_count(&self, count: u32) {
        let tooltip = if count > 0 {
            format!("Messenger ({})", count)
        } else {
            "Messenger".to_string()
        };

        if let Ok(tray) = self.app.tray_by_id(TrayIconId::new(TRAY_ID)) {
            let _ = tray.set_tooltip(Some(&tooltip));
        }

        // Emit event for frontend to react
        let _ = self.app.emit("tray-badge-update", count);
    }

    pub fn handle_event(app: &AppHandle, event: TrayIconEvent) {
        if let TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
            if let Some(window) = app.get_webview_window("main") {
                let is_visible = window.is_visible().unwrap_or(true);
                if is_visible {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
    }

    fn handle_menu_event(app: &AppHandle, menu_id: &str) {
        match menu_id {
            "open" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "new_message" => {
                let _ = app.emit("global-shortcut-trigger", "new_message");
            }
            "mute" => {
                let _ = app.emit("global-shortcut-trigger", "mute");
            }
            "dnd" => {
                let _ = app.emit("global-shortcut-trigger", "dnd");
            }
            "settings" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("navigate", "settings");
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        }
    }

    pub fn set_tooltip(&self, text: &str) {
        if let Ok(tray) = self.app.tray_by_id(TrayIconId::new(TRAY_ID)) {
            let _ = tray.set_tooltip(Some(text));
        }
    }
}

// Tauri commands for frontend invocation
#[tauri::command]
pub fn update_unread_count(
    state: tauri::State<'_, std::sync::Mutex<TrayManager>>,
    count: u32,
) -> Result<(), String> {
    let manager = state.lock().map_err(|e| e.to_string())?;
    manager.update_unread_count(count);
    Ok(())
}

#[tauri::command]
pub fn set_tray_tooltip(
    state: tauri::State<'_, std::sync::Mutex<TrayManager>>,
    text: String,
) -> Result<(), String> {
    let manager = state.lock().map_err(|e| e.to_string())?;
    manager.set_tooltip(&text);
    Ok(())
}

#[tauri::command]
pub fn init_tray(app: AppHandle) -> Result<(), String> {
    let manager = TrayManager::new(&app).map_err(|e| e.to_string())?;
    app.manage(std::sync::Mutex::new(manager));
    Ok(())
}
