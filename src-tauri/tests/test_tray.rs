#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tray_creation() {
        let tray = TrayManager::new(
            tauri::AppHandle::default(),
            tauri::State::new(std::sync::Mutex::new(0i32)),
        );
        let system_tray = tray.build_tray();
        assert!(system_tray.menu().is_some());
    }
    
    #[test]
    fn test_update_unread_count() {
        let unread_count = std::sync::Mutex::new(0i32);
        let tray = TrayManager::new(
            tauri::AppHandle::default(),
            tauri::State::new(unread_count),
        );
        tray.update_unread_count(5);
        let count = tray.unread_count.lock().unwrap();
        assert_eq!(*count, 5);
    }
    
    #[test]
    fn test_set_tray_tooltip() {
        let unread_count = std::sync::Mutex::new(0i32);
        let tray = TrayManager::new(
            tauri::AppHandle::default(),
            tauri::State::new(unread_count),
        );
        tray.set_tray_tooltip("Test Tooltip".to_string());
    }
    
    #[test]
    fn test_handle_tray_event() {
        let unread_count = std::sync::Mutex::new(0i32);
        let tray = TrayManager::new(
            tauri::AppHandle::default(),
            tauri::State::new(unread_count),
        );
        let event = tauri::SystemTrayEvent::LeftClick {
            position: (0, 0),
            size: (0, 0),
        };
        tray.handle_tray_event(event);
    }
    
    #[test]
    fn test_dock_badge() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        platform::macos::set_dock_badge(&window, "5").unwrap();
    }
    
    #[test]
    fn test_bounce_dock() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        platform::macos::bounce_dock(&window).unwrap();
    }
    
    #[test]
    fn test_taskbar_overlay() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        platform::windows::set_taskbar_overlay(&window, "5").unwrap();
    }
    
    #[test]
    fn test_jump_list() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        platform::windows::set_jump_list(&window, vec!["Item 1".to_string(), "Item 2".to_string()]).unwrap();
    }
    
    #[test]
    fn test_toast_notification() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        platform::windows::show_toast_notification(&window, "Test Title", "Test Body").unwrap();
    }
    
    #[test]
    fn test_appindicator_badge() {
        platform::linux::set_appindicator_badge("5").unwrap();
    }
    
    #[test]
    fn test_generate_desktop_file() {
        platform::linux::generate_desktop_file("Messenger Desktop", "messenger-desktop").unwrap();
    }
    
    #[test]
    fn test_dbuss_notification() {
        platform::linux::show_dbuss_notification("Test Title", "Test Body").unwrap();
    }
}
