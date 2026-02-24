#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_notification_creation() {
        let notification = Notification {
            title: "Test Title".to_string(),
            body: "Test Body".to_string(),
            icon: Some("test.png".to_string()),
            conversation_id: "123".to_string(),
        };
        assert_eq!(notification.title, "Test Title");
        assert_eq!(notification.body, "Test Body");
        assert_eq!(notification.icon, Some("test.png".to_string()));
        assert_eq!(notification.conversation_id, "123");
    }
    
    #[test]
    fn test_dnd_toggle() {
        let dnd = std::sync::Mutex::new(false);
        let manager = NotificationManager::new(
            tauri::AppHandle::default(),
            tauri::State::new(dnd),
        );
        assert_eq!(manager.is_dnd_enabled(), false);
        manager.toggle_dnd();
        assert_eq!(manager.is_dnd_enabled(), true);
    }
    
    #[test]
    fn test_set_dnd() {
        let dnd = std::sync::Mutex::new(false);
        let manager = NotificationManager::new(
            tauri::AppHandle::default(),
            tauri::State::new(dnd),
        );
        manager.set_dnd(true);
        assert_eq!(manager.is_dnd_enabled(), true);
        manager.set_dnd(false);
        assert_eq!(manager.is_dnd_enabled(), false);
    }
    
    #[test]
    fn test_notification_sound() {
        let dnd = std::sync::Mutex::new(false);
        let manager = NotificationManager::new(
            tauri::AppHandle::default(),
            tauri::State::new(dnd),
        );
        manager.set_notification_sound("test.wav");
    }
    
    #[test]
    fn test_show_notification() {
        let dnd = std::sync::Mutex::new(false);
        let manager = NotificationManager::new(
            tauri::AppHandle::default(),
            tauri::State::new(dnd),
        );
        let notification = Notification {
            title: "Test Title".to_string(),
            body: "Test Body".to_string(),
            icon: None,
            conversation_id: "123".to_string(),
        };
        let result = manager.show_notification(notification);
        assert!(result.is_ok());
    }
}
