//! Account manager for Tauri app.
//! Handles user accounts, authentication, and session management.

use tauri::{AppHandle, Manager, Emitter};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;
use uuid::Uuid;
use anyhow::{Context, Result};
use image::io::Reader as ImageReader;
use image::imageops::FilterType;

/// Account information.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub data_dir: String,
    pub is_active: bool,
    pub profile_picture: Option<String>,
    pub last_sync: Option<String>,
    pub session_token: Option<String>,
}

/// Account manager state.
pub struct AccountManager {
    accounts: Vec<Account>,
    app: AppHandle,
}

#[allow(dead_code)]
impl AccountManager {
    /// Create a new AccountManager.
    pub fn new(app: &AppHandle) -> Self {
        let accounts = app.state::<tauri_plugin_store::Store<tauri::Wry>>()
            .get("accounts")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        
        Self {
            accounts,
            app: app.clone(),
        }
    }
    
    /// Add a new account.
    pub fn add_account(&mut self, name: String) -> Result<Account> {
        let id = Uuid::new_v4().to_string();
        let app_data = self.app.path().app_data_dir()
            .context("Failed to resolve app data directory")?;
        let data_dir = app_data.join("accounts").join(&id);
        
        fs::create_dir_all(&data_dir)
            .context("Failed to create account directory")?;
        
        let account = Account {
            id: id.clone(),
            name,
            data_dir: data_dir.to_string_lossy().into_owned(),
            is_active: self.accounts.is_empty(),
            profile_picture: None,
            last_sync: None,
            session_token: None,
        };
        
        self.accounts.push(account.clone());
        self.save()?;
        Ok(account)
    }
    
    /// Remove an account.
    pub fn remove_account(&mut self, id: &str) -> Result<()> {
        if let Some(pos) = self.accounts.iter().position(|a| a.id == id) {
            let data_dir = Path::new(&self.accounts[pos].data_dir);
            if data_dir.exists() {
                fs::remove_dir_all(data_dir)
                    .context("Failed to remove account directory")?;
            }
            
            self.accounts.remove(pos);
            self.save()?;
        }
        Ok(())
    }
    
    /// Switch to an account.
    pub fn switch_account(&mut self, id: &str) -> Result<()> {
        for account in &mut self.accounts {
            account.is_active = account.id == id;
        }
        self.save()?;
        
        self.app.emit("switch-account", id)?;
        if let Some(window) = self.app.get_webview_window("main") {
            window.set_title(&format!("Messenger - {}", id))?;
        }
        Ok(())
    }
    
    /// List all accounts.
    pub fn list_accounts(&self) -> Vec<Account> {
        self.accounts.clone()
    }
    
    /// Set profile picture for an account.
    pub fn set_profile_picture(&mut self, id: &str, path: &str) -> Result<()> {
        if let Some(account) = self.accounts.iter_mut().find(|a| a.id == id) {
            let img = ImageReader::open(path)
                .context("Failed to open image")?
                .decode()
                .context("Failed to decode image")?;
            
            let output_path = Path::new(&account.data_dir).join("profile_picture.png");
            img.resize(128, 128, FilterType::Lanczos3)
                .save(&output_path)
                .context("Failed to save profile picture")?;
            
            account.profile_picture = Some(output_path.to_string_lossy().into_owned());
            self.save()?;
        }
        Ok(())
    }
    
    /// Set session token for an account.
    pub fn set_session_token(&mut self, id: &str, token: &str) -> Result<()> {
        if let Some(account) = self.accounts.iter_mut().find(|a| a.id == id) {
            account.session_token = Some(token.to_string());
            self.save()?;
        }
        Ok(())
    }
    
    /// Get session token for an account.
    pub fn get_session_token(&self, id: &str) -> Option<String> {
        self.accounts.iter().find(|a| a.id == id).and_then(|a| a.session_token.clone())
    }
    
    /// Update last sync time for an account.
    pub fn update_last_sync(&mut self, id: &str) -> Result<()> {
        if let Some(account) = self.accounts.iter_mut().find(|a| a.id == id) {
            account.last_sync = Some(chrono::Local::now().to_rfc3339());
            self.save()?;
        }
        Ok(())
    }
    
    /// Save accounts to store.
    fn save(&self) -> Result<()> {
        self.app.state::<tauri_plugin_store::Store<tauri::Wry>>()
            .set("accounts", serde_json::to_value(&self.accounts)?);
        Ok(())
    }
}

/// Tauri command: Add an account.
#[tauri::command]
pub fn add_account(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, name: String) -> Result<Account, String> {
    state.lock().unwrap().add_account(name).map_err(|e| e.to_string())
}

/// Tauri command: Remove an account.
#[tauri::command]
pub fn remove_account(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, id: String) -> Result<(), String> {
    state.lock().unwrap().remove_account(&id).map_err(|e| e.to_string())
}

/// Tauri command: Switch to an account.
#[tauri::command]
#[allow(dead_code)]
pub fn switch_account(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, id: String) -> Result<(), String> {
    state.lock().unwrap().switch_account(&id).map_err(|e| e.to_string())
}

/// Tauri command: List all accounts.
#[tauri::command]
pub fn list_accounts(state: tauri::State<'_, std::sync::Mutex<AccountManager>>) -> Vec<Account> {
    state.lock().unwrap().list_accounts()
}

/// Tauri command: Set profile picture for an account.
#[tauri::command]
#[allow(dead_code)]
pub fn set_profile_picture(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, id: String, path: String) -> Result<(), String> {
    state.lock().unwrap().set_profile_picture(&id, &path).map_err(|e| e.to_string())
}

/// Tauri command: Set session token for an account.
#[tauri::command]
#[allow(dead_code)]
pub fn set_session_token(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, id: String, token: String) -> Result<(), String> {
    state.lock().unwrap().set_session_token(&id, &token).map_err(|e| e.to_string())
}

/// Tauri command: Get session token for an account.
#[tauri::command]
#[allow(dead_code)]
pub fn get_session_token(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, id: String) -> Option<String> {
    state.lock().unwrap().get_session_token(&id)
}

/// Tauri command: Update last sync time for an account.
#[tauri::command]
#[allow(dead_code)]
pub fn update_last_sync(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, id: String) -> Result<(), String> {
    state.lock().unwrap().update_last_sync(&id).map_err(|e| e.to_string())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_serialization() {
        let account = Account {
            id: "test-123".to_string(),
            name: "Test Account".to_string(),
            data_dir: "/data".to_string(),
            is_active: true,
            profile_picture: None,
            last_sync: None,
            session_token: None,
        };
        let json = serde_json::to_string(&account).unwrap();
        let deserialized: Account = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "Test Account");
        assert_eq!(deserialized.id, "test-123");
        assert!(deserialized.is_active);
    }

    #[test]
    fn test_account_defaults() {
        let account = Account {
            id: "a".to_string(),
            name: "b".to_string(),
            data_dir: "/c".to_string(),
            is_active: false,
            profile_picture: None,
            last_sync: None,
            session_token: None,
        };
        assert!(account.profile_picture.is_none());
        assert!(account.session_token.is_none());
    }
}
