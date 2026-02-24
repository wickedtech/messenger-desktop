//! Media handling for Tauri app.
//! Manages file uploads, downloads, media permissions, and previews.

use tauri::AppHandle;
use tauri::Manager;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use anyhow::{Context, Result};
use uuid::Uuid;

/// Media permissions state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaPermissions {
    pub camera: bool,
    pub microphone: bool,
}

/// Media file metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub mime_type: String,
    pub is_image: bool,
    pub is_video: bool,
    pub is_audio: bool,
}

/// Media manager state.
pub struct MediaManager {
    app: AppHandle,
    permissions: MediaPermissions,
    media_dir: PathBuf,
}

impl MediaManager {
    /// Create a new MediaManager.
    pub fn new(app: &AppHandle) -> Result<Self> {
        let media_dir = app.path().app_data_dir()
            .context("Failed to resolve app data directory")?
            .join("media");
        
        if !media_dir.exists() {
            fs::create_dir_all(&media_dir)
                .context("Failed to create media directory")?;
        }
        
        Ok(Self {
            app: app.clone(),
            permissions: MediaPermissions {
                camera: false,
                microphone: false,
            },
            media_dir,
        })
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
    
    /// Save a media file to the app's media directory.
    pub fn save_media_file(&self, name: &str, data: &[u8]) -> Result<MediaFile> {
        let ext = Path::new(name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("bin");
        
        let id = Uuid::new_v4().to_string();
        let file_name = format!("{}.{}", id, ext);
        let file_path = self.media_dir.join(&file_name);
        
        let mut file = fs::File::create(&file_path)
            .context("Failed to create media file")?;
        
        file.write_all(data)
            .context("Failed to write media file")?;
        
        let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream();
        let size = fs::metadata(&file_path)?.len();
        
        Ok(MediaFile {
            id,
            name: name.to_string(),
            path: file_path,
            size,
            mime_type: mime_type.to_string(),
            is_image: mime_type.type_() == "image",
            is_video: mime_type.type_() == "video",
            is_audio: mime_type.type_() == "audio",
        })
    }
    
    /// Get a media file by ID.
    pub fn get_media_file(&self, id: &str) -> Result<MediaFile> {
        let entries = fs::read_dir(&self.media_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            
            if file_name == id {
                let mime_type = mime_guess::from_path(&path).first_or_octet_stream();
                let size = fs::metadata(&path)?.len();
                let name = path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                
                return Ok(MediaFile {
                    id: file_name,
                    name,
                    path,
                    size,
                    mime_type: mime_type.to_string(),
                    is_image: mime_type.type_() == "image",
                    is_video: mime_type.type_() == "video",
                    is_audio: mime_type.type_() == "audio",
                });
            }
        }
        anyhow::bail!("Media file not found")
    }
    
    /// Generate a preview for a media file.
    pub fn generate_preview(&self, id: &str) -> Result<PathBuf> {
        let media_file = self.get_media_file(id)?;
        if !media_file.is_image && !media_file.is_video {
            anyhow::bail!("Preview not supported for this media type");
        }
        
        let preview_dir = self.media_dir.join("previews");
        if !preview_dir.exists() {
            fs::create_dir_all(&preview_dir)?;
        }
        
        let preview_path = preview_dir.join(format!("{}.jpg", id));
        if !preview_path.exists() {
            // Placeholder for actual preview generation logic
            // In a real implementation, this would use a library like `image` or `ffmpeg`
            fs::File::create(&preview_path)?;
        }
        
        Ok(preview_path)
    }
    
    /// Delete a media file by ID.
    pub fn delete_media_file(&self, id: &str) -> Result<()> {
        let media_file = self.get_media_file(id)?;
        fs::remove_file(media_file.path)?;
        
        // Delete preview if it exists
        let preview_path = self.media_dir.join("previews").join(format!("{}.jpg", id));
        if preview_path.exists() {
            fs::remove_file(preview_path)?;
        }
        
        Ok(())
    }
}

/// Tauri command: Get media permissions.
#[tauri::command]
#[allow(dead_code)]
pub fn get_media_permissions(state: tauri::State<MediaManager>) -> MediaPermissions {
    state.get_permissions()
}

/// Tauri command: Grant media permission.
#[tauri::command]
pub async fn grant_media_permission(state: tauri::State<'_, tokio::sync::Mutex<MediaManager>>, permission_type: String) -> Result<bool, String> {
    match permission_type.as_str() {
        "camera" => Ok(state.lock().await.request_camera()),
        "microphone" => Ok(state.lock().await.request_microphone()),
        _ => {
            log::error!("Unknown permission type: {}", permission_type);
            Ok(false)
        }
    }
}

/// Tauri command: Save a media file.
#[tauri::command]
#[allow(dead_code)]
pub async fn save_media_file(
    state: tauri::State<'_, MediaManager>,
    name: String,
    data: Vec<u8>,
) -> Result<MediaFile, String> {
    state.save_media_file(&name, &data)
        .map_err(|e| e.to_string())
}

/// Tauri command: Get a media file by ID.
#[tauri::command]
#[allow(dead_code)]
pub fn get_media_file_command(state: tauri::State<MediaManager>, id: String) -> Result<MediaFile, String> {
    state.get_media_file(&id)
        .map_err(|e| e.to_string())
}

/// Tauri command: Generate a preview for a media file.
#[tauri::command]
#[allow(dead_code)]
pub fn generate_preview_command(state: tauri::State<MediaManager>, id: String) -> Result<PathBuf, String> {
    state.generate_preview(&id)
        .map_err(|e| e.to_string())
}

/// Tauri command: Delete a media file by ID.
#[tauri::command]
#[allow(dead_code)]
pub fn delete_media_file_command(state: tauri::State<MediaManager>, id: String) -> Result<(), String> {
    state.delete_media_file(&id)
        .map_err(|e| e.to_string())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_permissions_default() {
        let permissions = MediaPermissions {
            camera: false,
            microphone: false,
        };
        assert!(!permissions.camera);
        assert!(!permissions.microphone);
    }

    #[test]
    fn test_media_permissions_clone() {
        let permissions = MediaPermissions {
            camera: true,
            microphone: true,
        };
        let cloned = permissions.clone();
        assert!(cloned.camera);
        assert!(cloned.microphone);
    }

    #[test]
    fn test_media_file_serialization() {
        let file = MediaFile {
            id: "test-id".to_string(),
            name: "test.jpg".to_string(),
            path: PathBuf::from("/test/path.jpg"),
            size: 1024,
            mime_type: "image/jpeg".to_string(),
            is_image: true,
            is_video: false,
            is_audio: false,
        };
        let json = serde_json::to_string(&file).unwrap();
        let deserialized: MediaFile = serde_json::from_str(&json).unwrap();
        assert_eq!(file.id, deserialized.id);
    }

    #[test]
    fn test_theme_manager_get_themes() {
        let themes = ThemeManager::get_themes();
        assert!(themes.contains(&"dark".to_string()));
        assert!(themes.contains(&"light".to_string()));
    }
}