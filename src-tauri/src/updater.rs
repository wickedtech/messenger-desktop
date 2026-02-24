//! Updater manager for Tauri app.
//! Handles update checks, downloads, and installations.

use tauri::{AppHandle, Emitter, Manager};
use serde::Serialize;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Context, Result};
use log::{info, error};

/// Update information.
#[derive(Serialize, Clone, Debug)]
pub struct UpdateInfo {
    pub version: String,
    pub body: String,
    pub date: Option<String>,
    pub url: Option<String>,
}

/// Update progress.
#[derive(Serialize, Clone, Debug)]
pub struct UpdateProgress {
    pub downloaded: u64,
    pub total: Option<u64>,
    pub progress: f64,
    pub status: String,
}

/// Updater manager state.
pub struct UpdaterManager {
    app: AppHandle,
    pub channel: String,
    last_check: Mutex<Option<u64>>,
}

impl UpdaterManager {
    /// Create a new UpdaterManager.
    pub fn new(app: &AppHandle) -> Self {
        let channel = std::env::var("MESSENGER_RELEASE_CHANNEL").unwrap_or_else(|_| "stable".to_string());
        Self {
            app: app.clone(),
            channel,
            last_check: Mutex::new(None),
        }
    }
    
    /// Check for updates.
    pub async fn check_update(&self) -> Result<Option<UpdateInfo>> {
        let updater = self.app.updater()?;
        let update = updater.check().await?;
        
        if let Some(update) = update {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)?.as_secs();
            *self.last_check.lock().unwrap() = Some(current_time);
            
            Ok(Some(UpdateInfo {
                version: update.version.to_string(),
                body: update.body.unwrap_or_default(),
                date: update.date.map(|d| d.to_string()),
                url: update.url.map(|u| u.to_string()),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Install an update.
    pub async fn install_update(&self) -> Result<()> {
        let updater = self.app.updater()?;
        let handle = updater.download_and_install(|downloaded, total| {
            let progress = if let Some(total) = total {
                (downloaded as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            
            let status = if total.is_some() && downloaded >= total.unwrap() {
                "completed".to_string()
            } else {
                "downloading".to_string()
            };
            
            self.app.emit("update-progress", UpdateProgress {
                downloaded,
                total,
                progress,
                status,
            }).unwrap();
        }).await?;
        
        handle.await?;
        Ok(())
    }
    
    /// Get the current app version.
    pub fn get_current_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    
    /// Get the last update check time.
    pub fn get_last_check_time(&self) -> Option<u64> {
        *self.last_check.lock().unwrap()
    }
    
    /// Set the release channel.
    pub fn set_channel(&mut self, channel: &str) {
        self.channel = channel.to_string();
    }
    
    /// Get the current release channel.
    pub fn get_channel(&self) -> String {
        self.channel.clone()
    }
    
    /// Check if an update is available (cached).
    pub fn is_update_available(&self) -> bool {
        // Placeholder for cached update check logic
        false
    }
}

/// Tauri command: Check for updates.
#[tauri::command]
pub async fn check_update(state: tauri::State<'_, Mutex<UpdaterManager>>) -> Result<Option<UpdateInfo>, String> {
    state.lock().unwrap().check_update().await.map_err(|e| e.to_string())
}

/// Tauri command: Install an update.
#[tauri::command]
pub async fn install_update(state: tauri::State<'_, Mutex<UpdaterManager>>) -> Result<(), String> {
    state.lock().unwrap().install_update().await.map_err(|e| e.to_string())
}

/// Tauri command: Get the current app version.
#[tauri::command]
pub fn get_current_version(state: tauri::State<'_, Mutex<UpdaterManager>>) -> String {
    state.lock().unwrap().get_current_version()
}

/// Tauri command: Get the last update check time.
#[tauri::command]
pub fn get_last_check_time(state: tauri::State<'_, Mutex<UpdaterManager>>) -> Option<u64> {
    state.lock().unwrap().get_last_check_time()
}

/// Tauri command: Set the release channel.
#[tauri::command]
pub fn set_channel(state: tauri::State<'_, Mutex<UpdaterManager>>, channel: String) {
    state.lock().unwrap().set_channel(&channel);
}

/// Tauri command: Get the current release channel.
#[tauri::command]
pub fn get_channel(state: tauri::State<'_, Mutex<UpdaterManager>>) -> String {
    state.lock().unwrap().get_channel()
}