//! Platform manager for multi-platform messenger support
//!
//! This module manages platform selection, navigation, and state persistence.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Url;

/// Represents the supported social media platforms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Platform {
    /// Instagram Direct Messenger
    Instagram,
    /// Facebook Messenger
    Messenger,
    /// Facebook Messages
    Facebook,
    /// X (Twitter) Messages
    X,
}

impl Platform {
    /// Returns the URL for the platform's inbox/direct page
    pub fn url(&self) -> &'static str {
        match self {
            Platform::Instagram => "https://www.instagram.com/direct/inbox/",
            Platform::Messenger => "https://www.messenger.com",
            Platform::Facebook => "https://www.facebook.com/messages/",
            Platform::X => "https://x.com/messages",
        }
    }

    /// Returns the display name of the platform
    pub fn name(&self) -> &'static str {
        match self {
            Platform::Instagram => "Instagram",
            Platform::Messenger => "Messenger",
            Platform::Facebook => "Facebook",
            Platform::X => "X",
        }
    }

    /// Parses a platform name string into a Platform enum
    pub fn from_str(s: &str) -> Option<Platform> {
        match s {
            "Instagram" => Some(Platform::Instagram),
            "Messenger" => Some(Platform::Messenger),
            "Facebook" => Some(Platform::Facebook),
            "X" => Some(Platform::X),
            _ => None,
        }
    }
}

/// Manages platform state and persistence
pub struct PlatformManager {
    current: std::sync::Mutex<Option<Platform>>,
    store_path: PathBuf,
}

impl PlatformManager {
    /// Creates a new PlatformManager with the given app data directory
    pub fn new(app_data_dir: &Path) -> Self {
        let store_path = app_data_dir.join("platform.json");
        let manager = Self {
            current: std::sync::Mutex::new(None),
            store_path,
        };
        manager.load_last();
        manager
    }

    /// Gets the currently selected platform
    pub fn get_current(&self) -> Option<Platform> {
        self.current.lock().unwrap().clone()
    }

    /// Sets the current platform and persists it to disk
    pub fn set_current(&self, platform: Platform) {
        *self.current.lock().unwrap() = Some(platform);
        self.persist();
    }

    /// Loads the last used platform from disk
    pub fn load_last(&self) -> Option<Platform> {
        if self.store_path.exists() {
            let content = fs::read_to_string(&self.store_path).ok()?;
            let platform = serde_json::from_str::<String>(&content).ok()?;
            Platform::from_str(&platform).map(|p| {
                *self.current.lock().unwrap() = Some(p.clone());
                p
            })
        } else {
            None
        }
    }

    /// Persists the current platform to disk
    fn persist(&self) {
        if let Some(platform) = self.current.lock().unwrap().as_ref() {
            let _ = fs::write(
                &self.store_path,
                serde_json::to_string(platform.name()).unwrap(),
            );
        }
    }
}

/// Tauri command to select a platform by name
#[tauri::command]
pub fn select_platform(
    platform_name: String,
    manager: tauri::State<'_, PlatformManager>,
    window: tauri::WebviewWindow,
) -> Result<String, String> {
    let platform = Platform::from_str(&platform_name)
        .ok_or_else(|| format!("Unknown platform: {}", platform_name))?;

    manager.set_current(platform.clone());
    let url = Url::parse(platform.url())
        .map_err(|e| format!("Invalid platform URL: {}", e))?;
    window
        .navigate(url)
        .map_err(|e| format!("Failed to navigate: {}", e))?;

    Ok(format!("Selected platform: {}", platform.name()))
}

/// Tauri command to get the currently selected platform
#[tauri::command]
pub fn get_current_platform(manager: tauri::State<'_, PlatformManager>) -> Option<String> {
    manager.get_current().map(|p| p.name().to_string())
}

/// Tauri command to get the last used platform from storage
#[tauri::command]
pub fn get_last_platform(manager: tauri::State<'_, PlatformManager>) -> Option<String> {
    manager.load_last().map(|p| p.name().to_string())
}

/// Tauri command to list all available platforms
#[tauri::command]
pub fn list_platforms() -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({"name": "Instagram", "url": Platform::Instagram.url()}),
        serde_json::json!({"name": "Messenger", "url": Platform::Messenger.url()}),
        serde_json::json!({"name": "Facebook", "url": Platform::Facebook.url()}),
        serde_json::json!({"name": "X", "url": Platform::X.url()}),
    ]
}
