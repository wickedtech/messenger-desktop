#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shortcut_creation() {
        let shortcut = Shortcut {
            action: "toggle_window".to_string(),
            keys: "Ctrl+Shift+M".to_string(),
        };
        assert_eq!(shortcut.action, "toggle_window");
        assert_eq!(shortcut.keys, "Ctrl+Shift+M");
    }
    
    #[test]
    fn test_register_shortcuts() {
        let manager = ShortcutManager::new(tauri::AppHandle::default());
        let result = manager.register_shortcuts();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_update_shortcut() {
        let manager = ShortcutManager::new(tauri::AppHandle::default());
        let result = manager.update_shortcut("toggle_window".to_string(), "Ctrl+Shift+M".to_string());
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_unregister_all() {
        let manager = ShortcutManager::new(tauri::AppHandle::default());
        let result = manager.unregister_all();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_shortcut_handler() {
        let manager = ShortcutManager::new(tauri::AppHandle::default());
        let result = manager.register_shortcuts();
        assert!(result.is_ok());
        // Simulate shortcut press
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        window.eval("document.querySelector('[aria-label=\"New message\"]').click()")
            .unwrap();
    }
}
