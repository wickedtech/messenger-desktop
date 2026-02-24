//! Media permissions manager for Tauri app.
//! Handles camera and microphone permissions.

use tauri::AppHandle;
use tauri::Manager;
use serde::{Serialize, Deserialize};

/// Media permissions state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaPermissions {
    pub camera: bool,
    pub microphone: bool,
}

/// Media manager state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaManager {
    app: AppHandle,
    permissions: MediaPermissions,
}

impl MediaManager {
    /// Create a new MediaManager.
    pub fn new(app: &AppHandle) -> Self {
        Self {
            app: app.clone(),
            permissions: MediaPermissions {
                camera: false,
                microphone: false,
            },
        }
    }
    
    /// Setup WebView permissions for messenger.com domain.
    pub fn setup_permissions(&self) {
        log::info!("Configuring WebView media permissions for messenger.com");
        // Placeholder for WebView permission configuration
        // In a real implementation, this would configure the WebView to auto-grant
        // camera/microphone permissions for the messenger.com domain.
    }
    
    /// Request camera permission.
    pub fn request_camera(&mut self) -> bool {
        log::info!("Requesting camera permission");
        self.permissions.camera = true; // Stub for actual permission request
        self.permissions.camera
    }
    
    /// Request microphone permission.
    pub fn request_microphone(&mut self) -> bool {
        log::info!("Requesting microphone permission");
        self.permissions.microphone = true; // Stub for actual permission request
        self.permissions.microphone
    }
    
    /// Get current media permissions.
    pub fn get_permissions(&self) -> MediaPermissions {
        self.permissions.clone()
    }
}

/// Tauri command: Get media permissions.
#[tauri::command]
pub fn get_media_permissions(state: tauri::State<MediaManager>) -> MediaPermissions {
    state.get_permissions()
}

/// Tauri command: Grant media permission.
#[tauri::command]
pub fn grant_media_permission(state: tauri::State<MediaManager>, permission_type: String) -> bool {
    match permission_type.as_str() {
        "camera" => state.request_camera(),
        "microphone" => state.request_microphone(),
        _ => {
            log::error!("Unknown permission type: {}", permission_type);
            false
        }
    }
}