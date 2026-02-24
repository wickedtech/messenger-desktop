//! Updater manager for Tauri app.
//! Handles update checks, downloads, and installations.

use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;
use serde::Serialize;
use tokio::sync::Mutex as TokioMutex;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

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
    last_check: std::sync::Mutex<Option<u64>>,
}

impl UpdaterManager {
    /// Create a new UpdaterManager.
    pub fn new(app: &AppHandle) -> Self {
        let channel = std::env::var("MESSENGER_RELEASE_CHANNEL").unwrap_or_else(|_| "stable".to_string());
        Self {
            app: app.clone(),
            channel,
            last_check: std::sync::Mutex::new(None),
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
                url: None,
            }))
        } else {
            Ok(None)
        }
    }

    /// Install an update.
    pub async fn install_update(&self) -> Result<()> {
        // Tauri 2 updater API has changed significantly
        // TODO: Implement proper updater for Tauri 2
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
pub async fn check_update(state: tauri::State<'_, TokioMutex<UpdaterManager>>) -> Result<Option<UpdateInfo>, String> {
    state.lock().await.check_update().await.map_err(|e| e.to_string())
}

/// Tauri command: Install an update.
#[tauri::command]
pub async fn install_update(state: tauri::State<'_, TokioMutex<UpdaterManager>>) -> Result<(), String> {
    state.lock().await.install_update().await.map_err(|e| e.to_string())
}

/// Tauri command: Get the current app version.
#[tauri::command]
pub fn get_current_version(state: tauri::State<'_, TokioMutex<UpdaterManager>>) -> String {
    state.blocking_lock().get_current_version()
}

/// Tauri command: Get the last update check time.
#[tauri::command]
pub fn get_last_check_time(state: tauri::State<'_, TokioMutex<UpdaterManager>>) -> Option<u64> {
    state.blocking_lock().get_last_check_time()
}

/// Tauri command: Set the release channel.
#[tauri::command]
pub fn set_channel(state: tauri::State<'_, TokioMutex<UpdaterManager>>, channel: String) {
    state.blocking_lock().set_channel(&channel);
}

/// Tauri command: Get the current release channel.
#[tauri::command]
pub fn get_channel(state: tauri::State<'_, TokioMutex<UpdaterManager>>) -> String {
    state.blocking_lock().get_channel()
}