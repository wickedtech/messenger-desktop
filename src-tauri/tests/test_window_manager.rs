#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_window_state() {
        let state = WindowState {
            x: 100.0,
            y: 100.0,
            width: 1200.0,
            height: 800.0,
            is_maximized: false,
        };
        assert_eq!(state.x, 100.0);
        assert_eq!(state.y, 100.0);
        assert_eq!(state.width, 1200.0);
        assert_eq!(state.height, 800.0);
        assert_eq!(state.is_maximized, false);
    }
    
    #[test]
    fn test_minimize_to_tray() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let result = manager.minimize_to_tray();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_restore_from_tray() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let result = manager.restore_from_tray();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_save_window_state() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let result = manager.save_window_state();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_restore_window_state() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let state = WindowState {
            x: 100.0,
            y: 100.0,
            width: 1200.0,
            height: 800.0,
            is_maximized: false,
        };
        let result = manager.restore_window_state(state);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_toggle_always_on_top() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let result = manager.toggle_always_on_top();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_toggle_focus_mode() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let result = manager.toggle_focus_mode();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_set_zoom() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let result = manager.set_zoom(1.5);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_toggle_fullscreen() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let result = manager.toggle_fullscreen();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_open_settings_window() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let result = manager.open_settings_window();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_close_all_windows() {
        let manager = WindowManager::new(tauri::AppHandle::default());
        let result = manager.close_all_windows();
        assert!(result.is_ok());
    }
}
