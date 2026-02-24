#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_account_creation() {
        let account = Account {
            id: "123".to_string(),
            name: "Test Account".to_string(),
            data_dir: std::path::PathBuf::from("/tmp/test"),
        };
        assert_eq!(account.id, "123");
        assert_eq!(account.name, "Test Account");
        assert_eq!(account.data_dir, std::path::PathBuf::from("/tmp/test"));
    }
    
    #[test]
    fn test_add_account() {
        let app = tauri::AppHandle::default();
        let manager = AccountManager::new(app);
        let result = manager.add_account("Test Account");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_remove_account() {
        let app = tauri::AppHandle::default();
        let manager = AccountManager::new(app);
        let account = manager.add_account("Test Account").unwrap();
        let result = manager.remove_account(&account.id);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_switch_account() {
        let app = tauri::AppHandle::default();
        let manager = AccountManager::new(app);
        let account = manager.add_account("Test Account").unwrap();
        let result = manager.switch_account(&account.id);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_list_accounts() {
        let app = tauri::AppHandle::default();
        let manager = AccountManager::new(app);
        let accounts = manager.list_accounts();
        assert_eq!(accounts.len(), 0);
    }
    
    #[test]
    fn test_get_account() {
        let app = tauri::AppHandle::default();
        let manager = AccountManager::new(app);
        let account = manager.add_account("Test Account").unwrap();
        let result = manager.get_account(&account.id);
        assert!(result.is_some());
    }
}
