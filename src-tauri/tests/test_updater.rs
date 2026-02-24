#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_update_info() {
        let info = UpdateInfo {
            version: "1.0.0".to_string(),
            body: "Test update".to_string(),
            date: "2023-01-01".to_string(),
        };
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.body, "Test update");
        assert_eq!(info.date, "2023-01-01");
    }
    
    #[test]
    fn test_get_current_version() {
        let manager = UpdaterManager::new(tauri::AppHandle::default());
        let version = manager.get_current_version();
        assert!(!version.is_empty());
    }
    
    #[test]
    fn test_check_update() {
        let manager = UpdaterManager::new(tauri::AppHandle::default());
        let result = manager.check_update();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_install_update() {
        let manager = UpdaterManager::new(tauri::AppHandle::default());
        let result = manager.install_update();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_download_update() {
        let manager = UpdaterManager::new(tauri::AppHandle::default());
        let result = manager.download_update();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_install_downloaded_update() {
        let manager = UpdaterManager::new(tauri::AppHandle::default());
        let result = manager.install_downloaded_update();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_set_release_channel() {
        let manager = UpdaterManager::new(tauri::AppHandle::default());
        let result = manager.set_release_channel("stable");
        assert!(result.is_ok());
    }
}
