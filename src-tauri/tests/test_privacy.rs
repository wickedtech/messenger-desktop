#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_privacy_config() {
        let config = PrivacyConfig {
            block_typing: true,
            block_read_receipts: true,
            hide_active: true,
        };
        assert_eq!(config.block_typing, true);
        assert_eq!(config.block_read_receipts, true);
        assert_eq!(config.hide_active, true);
    }
    
    #[test]
    fn test_set_config() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        let mut manager = PrivacyManager::new(window);
        let config = PrivacyConfig {
            block_typing: true,
            block_read_receipts: true,
            hide_active: true,
        };
        manager.set_config(config);
    }
    
    #[test]
    fn test_get_config() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        let manager = PrivacyManager::new(window);
        let config = manager.get_config();
        assert_eq!(config.block_typing, false);
        assert_eq!(config.block_read_receipts, false);
        assert_eq!(config.hide_active, false);
    }
}
