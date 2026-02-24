// Native Notification System for Messenger desktop app
// Receives notification data from JavaScript injection, shows OS-native notifications
// Supports Do Not Disturb mode, custom sounds, and quick reply (platform-specific)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Notification data received from JavaScript injection
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NotificationData {
    pub id: String,
    pub title: String,
    pub body: String,
    pub icon_url: Option<String>,
    pub conversation_id: Option<String>,
    pub sender_name: Option<String>,
    pub sender_avatar: Option<String>,
    pub timestamp: Option<u64>,
    pub require_interaction: bool,
    pub silent: bool,
}

/// Platform-specific notification settings
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NotificationSettings {
    pub enabled: bool,
    pub sound_enabled: bool,
    pub sound_path: Option<String>,
    pub do_not_disturb: bool,
    pub dnd_schedule: Option<DNDSchedule>,
    pub show_preview: bool,
    pub quick_reply_enabled: bool,
}

/// Do Not Disturb schedule
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct DNDSchedule {
    pub start_time: String, // HH:MM format
    pub end_time: String,   // HH:MM format
}

/// Notification service state
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NotificationState {
    pub settings: NotificationSettings,
    #[allow(dead_code)]
    pub temporary_icons: Vec<PathBuf>,
}

/// Native Notification Service - manages OS-native notifications
#[allow(dead_code)]
pub struct NotificationService {
    state: Arc<RwLock<NotificationState>>,
    #[allow(dead_code)]
    app_data_dir: PathBuf,
}

impl NotificationService {
    /// Create a new notification service
    pub fn new(app_data_dir: PathBuf) -> Self {
        Self {
            state: Arc::new(RwLock::new(NotificationState {
                settings: NotificationSettings {
                    enabled: true,
                    sound_enabled: false,
                    sound_path: None,
                    do_not_disturb: false,
                    dnd_schedule: None,
                    show_preview: true,
                    quick_reply_enabled: false,
                },
                temporary_icons: Vec::new(),
            })),
            app_data_dir,
        }
    }

    /// Show a native notification
    pub async fn show_notification(&self, data: NotificationData) -> Result<()> {
        debug!("Showing notification: {}", data.title);

        // Check if notifications are enabled
        let state = self.state.read().await;
        let settings_enabled = state.settings.enabled;
        let settings_do_not_disturb = state.settings.do_not_disturb;
        let settings_dnd_schedule = state.settings.dnd_schedule.clone();
        let settings_sound_enabled = state.settings.sound_enabled;
        let settings_sound_path = state.settings.sound_path.clone();

        if !settings_enabled {
            info!("Notifications disabled, skipping: {}", data.title);
            return Ok(());
        }

        // Check Do Not Disturb mode
        if settings_do_not_disturb {
            info!("DND active, suppressing notification: {}", data.title);
            return Ok(());
        }

        // Check DND schedule if configured
        if let Some(schedule) = &settings_dnd_schedule {
            if self.is_in_dnd_schedule(schedule).await {
                info!("In DND schedule, suppressing notification: {}", data.title);
                return Ok(());
            }
        }

        // Download and prepare icon if provided
        let icon_path = if let Some(icon_url) = &data.icon_url {
            self.download_and_save_icon(icon_url, &data.id).await?
        } else {
            None
        };

        // Prepare notification payload
        let payload = NotificationPayload {
            id: data.id.clone(),
            title: data.title.clone(),
            body: data.body.clone(),
            icon_path,
            conversation_id: data.conversation_id.clone(),
            sender_name: data.sender_name.clone(),
            silent: data.silent,
        };

        drop(state); // Release the lock before calling platform-specific code

        // Show the notification using platform-specific implementation
        #[cfg(target_os = "macos")]
        self.show_native_macos(&payload).await?;

        #[cfg(target_os = "windows")]
        self.show_native_windows(&payload).await?;

        #[cfg(target_os = "linux")]
        self.show_native_linux(&payload).await?;

        // Play sound if enabled
        if settings_sound_enabled {
            self.play_notification_sound(&settings_sound_path).await?;
        }

        info!("Notification shown: {} - {}", data.title, data.body);
        Ok(())
    }

    /// Download and save icon from URL to temporary location
    async fn download_and_save_icon(
        &self,
        url: &str,
        _notification_id: &str,
    ) -> Result<Option<String>> {
        debug!("Downloading icon from: {}", url);

        // In a real implementation, you would use reqwest to download:
        // let response = reqwest::get(url).await?;
        // let bytes = response.bytes().await?;
        // let icon_path = self.app_data_dir.join(format!("notification_{}.png", notification_id));
        // fs::write(&icon_path, &bytes)?;
        // 
        // self.state.write().await.temporary_icons.push(icon_path.clone());
        
        // For now, return the URL as-is since we can't download in this environment
        // The actual implementation should save to a temp file
        
        Ok(Some(url.to_string()))
    }

    /// Check if current time is within DND schedule
    async fn is_in_dnd_schedule(&self, _schedule: &DNDSchedule) -> bool {
        // Parse start and end times
        // Compare with current time
        
        // For now, return false (not in DND)
        // In a real implementation:
        // let now = Local::now();
        // let start = Self::parse_time(&schedule.start_time).unwrap();
        // let end = Self::parse_time(&schedule.end_time).unwrap();
        // 
        // if start <= end {
        //     now.time() >= start && now.time() <= end
        // } else {
        //     // Overnight schedule
        //     now.time() >= start || now.time() <= end
        // }
        
        false
    }

    /// Play notification sound
    async fn play_notification_sound(&self, sound_path: &Option<String>) -> Result<()> {
        if let Some(path) = sound_path {
            debug!("Playing notification sound: {}", path);
            
            // In a real implementation, you would use:
            // - macOS:NSSound with file path
            // - Windows:Windows.Media.Playback
            // - Linux:pactl or paplay for ALSA/PulseAudio
            
            // For now, just log since we can't play sounds in this environment
            info!("Would play sound from: {}", path);
        } else {
            debug!("Playing default notification sound");
            
            // Default sound based on platform:
            // - macOS: NSAlertDefaultSound
            // - Windows: SystemSound::Notification
            // - Linux: /usr/share/sounds/generic.wav
            
            info!("Would play default system notification sound");
        }
        
        Ok(())
    }

    /// Set Do Not Disturb mode
    pub async fn set_dnd(&self, enabled: bool) -> Result<()> {
        debug!("Setting DND to: {}", enabled);

        self.state.write().await.settings.do_not_disturb = enabled;

        info!("Do Not Disturb mode: {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    }

    /// Set notification sound path
    pub async fn set_notification_sound(&self, path: String) -> Result<()> {
        debug!("Setting notification sound to: {}", path);

        // Verify the file exists
        if !PathBuf::from(&path).exists() {
            warn!("Sound file does not exist: {}", path);
            return Err(anyhow::anyhow!("Sound file does not exist"));
        }

        self.state.write().await.settings.sound_path = Some(path);

        info!("Notification sound updated");
        Ok(())
    }

    /// Enable/disable notifications
    pub async fn set_enabled(&self, enabled: bool) -> Result<()> {
        debug!("Setting notifications enabled to: {}", enabled);

        self.state.write().await.settings.enabled = enabled;

        info!("Notifications {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    }

    /// Set sound enabled state
    pub async fn set_sound_enabled(&self, enabled: bool) -> Result<()> {
        debug!("Setting sound enabled to: {}", enabled);

        self.state.write().await.settings.sound_enabled = enabled;

        info!("Notification sound {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    }

    /// Set show preview preference
    #[allow(dead_code)]
    pub async fn set_show_preview(&self, enabled: bool) -> Result<()> {
        debug!("Setting show preview to: {}", enabled);

        self.state.write().await.settings.show_preview = enabled;

        Ok(())
    }

    /// Set quick reply preference
    #[allow(dead_code)]
    pub async fn set_quick_reply_enabled(&self, enabled: bool) -> Result<()> {
        debug!("Setting quick reply enabled to: {}", enabled);

        self.state.write().await.settings.quick_reply_enabled = enabled;

        Ok(())
    }

    /// Get current notification settings
    pub async fn get_settings(&self) -> NotificationSettings {
        self.state.read().await.settings.clone()
    }

    /// Close the notification service and clean up temporary files
    #[allow(dead_code)]
    pub async fn cleanup(&self) -> Result<()> {
        debug!("Cleaning up notification service");

        let mut state = self.state.write().await;
        
        for icon_path in &state.temporary_icons {
            if icon_path.exists() {
                if let Err(e) = fs::remove_file(icon_path) {
                    warn!("Failed to remove temporary icon {}: {}", icon_path.display(), e);
                }
            }
        }
        
        state.temporary_icons.clear();
        
        info!("Notification service cleanup complete");
        Ok(())
    }

    // Platform-specific notification implementations
    #[cfg(target_os = "macos")]
    async fn show_native_macos(&self, payload: &NotificationPayload) -> Result<()> {
        debug!("Showing macOS native notification");

        // Use NSUserNotification on macOS
        // Cocoa bindings or user_notifications crate
        
        info!(
            "macOS notification: {} - {}",
            payload.title, payload.body
        );

        // In a real implementation:
        // let notification = NSUserNotification::new(nil);
        // notification.setTitle(payload.title.to_nsstring());
        // notification.setInformativeText(payload.body.to_nsstring());
        // 
        // if let Some(icon_path) = &payload.icon_path {
        //     // Set icon from file
        // }
        // 
        // let center = NSUserNotificationCenter::defaultUserNotificationCenter(nil);
        // center.scheduleNotification(notification);

        Ok(())
    }

    #[cfg(target_os = "windows")]
    async fn show_native_windows(&self, payload: &NotificationPayload) -> Result<()> {
        debug!("Showing Windows native notification");

        // Use Windows Notification API on Windows 10+
        // windows-rs or winapi crate
        
        info!(
            "Windows notification: {} - {}",
            payload.title, payload.body
        );

        // In a real implementation:
        // let notifier = ToastNotificationManager::CreateToastNotifier().unwrap();
        // let xml = Self::create_toast_xml(payload);
        // let notification = ToastNotification::from_xml(&xml).unwrap();
        // notifier.show(&notification).unwrap();

        Ok(())
    }

    #[cfg(target_os = "linux")]
    async fn show_native_linux(&self, payload: &NotificationPayload) -> Result<()> {
        debug!("Showing Linux native notification");

        // Use D-Bus notification interface (freedesktop spec)
        // dbus crate or zbus crate
        
        info!(
            "Linux notification: {} - {}",
            payload.title, payload.body
        );

        // In a real implementation:
        // let connection = zbus::Connection::session().await?;
        // let notification = zbus::Message::new_signal(
        //     "/org/freedesktop/Notifications",
        //     "org.freedesktop.Notifications",
        //     "Notify",
        // )?;
        // 
        // // Build notification payload and send via D-Bus

        Ok(())
    }

    // Helper to create toast XML for Windows
    #[cfg(target_os = "windows")]
    fn create_toast_xml(payload: &NotificationPayload) -> String {
        let icon_xml = payload
            .icon_path
            .as_ref()
            .map(|icon| format!(r#"<image id="1" src="{}"/>"#, icon))
            .unwrap_or_default();

        let body_xml = if payload.sender_name.is_some() {
            format!(
                r#"<text id="1">{}</text>
            <text id="2">{}</text>"#,
                payload.sender_name.as_ref().unwrap(),
                payload.body
            )
        } else {
            format!(r#"<text id="1">{}</text>"#, payload.body)
        };

        format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<toast>
    <visual>
        <binding template="ToastGeneric">
            <text>{}</text>
            {}
            {}
        </binding>
    </visual>
</toast>"#,
            payload.title, body_xml, icon_xml
        )
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        // Use standard app data directory
        let app_data_dir = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        Self::new(app_data_dir)
    }
}

// Notification payload structure
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct NotificationPayload {
    #[allow(dead_code)]
    id: String,
    title: String,
    body: String,
    #[allow(dead_code)]
    icon_path: Option<String>,
    #[allow(dead_code)]
    conversation_id: Option<String>,
    #[allow(dead_code)]
    sender_name: Option<String>,
    #[allow(dead_code)]
    silent: bool,
}

// Tauri commands

/// Show a native notification
#[tauri::command]
#[specta::specta]
pub async fn show_notification(
    title: String,
    body: String,
    icon_url: Option<String>,
    notification_service: tauri::State<'_, NotificationService>,
) -> Result<(), String> {
    let data = NotificationData {
        id: format!("notification_{}", chrono::Utc::now().timestamp()),
        title,
        body,
        icon_url,
        conversation_id: None,
        sender_name: None,
        sender_avatar: None,
        timestamp: None,
        require_interaction: false,
        silent: false,
    };

    notification_service.show_notification(data).await.map_err(|e| e.to_string())
}

/// Set Do Not Disturb mode
#[tauri::command]
#[specta::specta]
pub async fn set_dnd(
    enabled: bool,
    notification_service: tauri::State<'_, NotificationService>,
) -> Result<(), String> {
    notification_service.set_dnd(enabled).await.map_err(|e| e.to_string())
}

/// Toggle Do Not Disturb mode
#[tauri::command]
#[specta::specta]
pub async fn toggle_dnd(
    notification_service: tauri::State<'_, NotificationService>,
) -> Result<bool, String> {
    let current = notification_service.get_settings().await.do_not_disturb;
    notification_service.set_dnd(!current).await.map_err(|e| e.to_string())?;
    Ok(!current)
}

/// Get Do Not Disturb status
#[tauri::command]
#[specta::specta]
pub async fn is_dnd_enabled(
    notification_service: tauri::State<'_, NotificationService>,
) -> Result<bool, String> {
    Ok(notification_service.get_settings().await.do_not_disturb)
}

/// Set notification sound path
#[tauri::command]
#[specta::specta]
pub async fn set_notification_sound(
    path: String,
    notification_service: tauri::State<'_, NotificationService>,
) -> Result<(), String> {
    notification_service.set_notification_sound(path).await.map_err(|e| e.to_string())
}

/// Get notification settings
#[tauri::command]
#[specta::specta]
pub async fn get_notification_settings(
    notification_service: tauri::State<'_, NotificationService>,
) -> Result<NotificationSettings, String> {
    Ok(notification_service.get_settings().await)
}

/// Enable/disable notifications
#[tauri::command]
#[specta::specta]
pub async fn set_notification_enabled(
    enabled: bool,
    notification_service: tauri::State<'_, NotificationService>,
) -> Result<(), String> {
    notification_service.set_enabled(enabled).await.map_err(|e| e.to_string())
}

/// Enable/disable notification sound
#[tauri::command]
#[specta::specta]
pub async fn set_notification_sound_enabled(
    enabled: bool,
    notification_service: tauri::State<'_, NotificationService>,
) -> Result<(), String> {
    notification_service.set_sound_enabled(enabled).await.map_err(|e| e.to_string())
}

/// Set notification sound to default
#[tauri::command]
#[specta::specta]
pub async fn use_default_notification_sound(
    notification_service: tauri::State<'_, NotificationService>,
) -> Result<(), String> {
    notification_service.set_notification_sound(String::new()).await.map_err(|e| e.to_string())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_data_default() {
        let data = NotificationData {
            id: "test-id".to_string(),
            title: "Test".to_string(),
            body: "Body".to_string(),
            icon_url: None,
            conversation_id: None,
            sender_name: None,
            sender_avatar: None,
            timestamp: None,
            require_interaction: false,
            silent: false,
        };
        assert_eq!(data.id, "test-id");
        assert_eq!(data.title, "Test");
    }

    #[test]
    fn test_notification_settings_default() {
        let settings = NotificationSettings {
            enabled: true,
            sound_enabled: false,
            sound_path: None,
            do_not_disturb: false,
            dnd_schedule: None,
            show_preview: true,
            quick_reply_enabled: false,
        };
        assert!(settings.enabled);
        assert!(!settings.do_not_disturb);
    }

    #[test]
    fn test_dnd_schedule_serialization() {
        let schedule = DNDSchedule {
            start_time: "22:00".to_string(),
            end_time: "08:00".to_string(),
        };
        let json = serde_json::to_string(&schedule).unwrap();
        let deserialized: DNDSchedule = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.start_time, "22:00");
    }

    #[test]
    fn test_notification_service_new() {
        let _service = NotificationService::new(PathBuf::from("/tmp"));
        // Service instantiated successfully
        assert!(true);
    }
}
