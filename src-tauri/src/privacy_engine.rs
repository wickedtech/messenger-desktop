//! Privacy Engine for multi-platform session isolation and privacy enforcement.
//! Provides session directory management, cookie clearing, and Content Security Policy (CSP) per platform.

use std::path::PathBuf;

/// Privacy Engine for managing session isolation and privacy enforcement.
#[derive(Debug)]
pub struct PrivacyEngine {
    app_data_dir: PathBuf,
}

impl PrivacyEngine {
    /// Creates a new `PrivacyEngine` instance.
    ///
    /// # Arguments
    ///
    /// * `app_data_dir` - The application data directory path.
    ///
    /// # Returns
    ///
    /// A new `PrivacyEngine` instance.
    pub fn new(app_data_dir: PathBuf) -> Self {
        Self { app_data_dir }
    }

    /// Returns the session directory path for a given platform.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform name (e.g., "Instagram", "Messenger").
    ///
    /// # Returns
    ///
    /// The session directory path.
    pub fn session_dir(&self, platform: &str) -> PathBuf {
        self.app_data_dir.join("sessions").join(platform)
    }

    /// Clears the session for a specific platform by removing its directory and recreating it.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform name.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error message on failure.
    pub fn clear_session(&self, platform: &str) -> Result<(), String> {
        let dir = self.session_dir(platform);
        if dir.exists() {
            std::fs::remove_dir_all(&dir).map_err(|e| format!("clear_session: {e}"))?;
        }
        std::fs::create_dir_all(&dir).map_err(|e| format!("create session dir: {e}"))?;
        log::info!("[PrivacyEngine] cleared session for {}", platform);
        Ok(())
    }

    /// Clears all sessions by removing the entire sessions directory.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error message on failure.
    pub fn clear_all_sessions(&self) -> Result<(), String> {
        let sessions_dir = self.app_data_dir.join("sessions");
        if sessions_dir.exists() {
            std::fs::remove_dir_all(&sessions_dir).map_err(|e| format!("clear_all: {e}"))?;
        }
        log::info!("[PrivacyEngine] cleared all sessions");
        Ok(())
    }

    /// Returns the Content Security Policy (CSP) for a given platform.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform name.
    ///
    /// # Returns
    ///
    /// The CSP string for the platform.
    pub fn csp_for_platform(platform: &str) -> &'static str {
        match platform {
            "Instagram" => "default-src https://www.instagram.com https://*.instagram.com https://*.cdninstagram.com https://*.fbcdn.net; img-src * data:; media-src *;",
            "Messenger" => "default-src https://www.messenger.com https://*.messenger.com https://*.facebook.com https://*.fbcdn.net; img-src * data:;",
            "Facebook"  => "default-src https://www.facebook.com https://*.facebook.com https://*.fbcdn.net https://*.facebook.net; img-src * data:;",
            "X"         => "default-src https://x.com https://*.x.com https://*.twimg.com; img-src * data:;",
            _           => "default-src *;",
        }
    }

    /// Checks if a URL contains a blocked domain.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to check.
    ///
    /// # Returns
    ///
    /// `true` if the URL contains a blocked domain, `false` otherwise.
    #[allow(dead_code)]
    pub fn is_blocked_domain(url: &str) -> bool {
        ["doubleclick.net","googlesyndication.com","google-analytics.com",
         "analytics.facebook.com","pixel.facebook.com"]
            .iter().any(|d| url.contains(d))
    }
}

/// Clears the session for a specific platform.
///
/// # Arguments
///
/// * `platform` - The platform name.
/// * `engine` - The Tauri state containing the `PrivacyEngine` instance.
///
/// # Returns
///
/// `Ok(())` on success, or an error message on failure.
#[tauri::command]
pub fn clear_platform_session(platform: String, engine: tauri::State<'_, PrivacyEngine>) -> Result<(), String> {
    engine.clear_session(&platform)
}

/// Clears all sessions.
///
/// # Arguments
///
/// * `engine` - The Tauri state containing the `PrivacyEngine` instance.
///
/// # Returns
///
/// `Ok(())` on success, or an error message on failure.
#[tauri::command]
pub fn clear_all_sessions(engine: tauri::State<'_, PrivacyEngine>) -> Result<(), String> {
    engine.clear_all_sessions()
}

/// Returns the Content Security Policy (CSP) for a given platform.
///
/// # Arguments
///
/// * `platform` - The platform name.
///
/// # Returns
///
/// The CSP string for the platform.
#[tauri::command]
pub fn get_csp_for_platform(platform: String) -> String {
    PrivacyEngine::csp_for_platform(&platform).to_string()
}

// INTEGRATION NOTE (Wave 5): Register on_window_event in lib.rs:
// app.on_window_event(|window, event| {
//   if matches!(event, tauri::WindowEvent::CloseRequested { .. }) {
//     let _ = window.app_handle().state::<PrivacyEngine>().clear_all_sessions();
//   }
// });
