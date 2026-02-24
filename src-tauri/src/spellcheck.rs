//! Spellcheck manager for Tauri app.
//! Handles spellcheck state and WebView communication.

use tauri::AppHandle;
use tauri::Manager;
use serde::{Serialize, Deserialize};

/// Spellcheck manager state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellcheckManager {
    app: AppHandle,
    enabled: bool,
    language: String,
}

impl SpellcheckManager {
    /// Create a new SpellcheckManager.
    pub fn new(app: &AppHandle) -> Self {
        Self {
            app: app.clone(),
            enabled: false,
            language: "en-US".to_string(),
        }
    }
    
    /// Enable spellcheck.
    pub fn enable(&mut self) {
        self.enabled = true;
        self.emit_event("enable-spellcheck", &true);
    }
    
    /// Disable spellcheck.
    pub fn disable(&mut self) {
        self.enabled = false;
        self.emit_event("enable-spellcheck", &false);
    }
    
    /// Set the spellcheck language.
    pub fn set_language(&mut self, lang: &str) {
        self.language = lang.to_string();
        self.emit_event("set-spellcheck-lang", &self.language);
    }
    
    /// Get available spellcheck languages.
    pub fn get_available_languages() -> Vec<String> {
        vec![
            "en-US".to_string(),
            "en-GB".to_string(),
            "fr-FR".to_string(),
            "ar".to_string(),
            "de".to_string(),
            "es".to_string(),
        ]
    }
    
    /// Emit an event to the WebView.
    fn emit_event<T: Serialize + Clone>(&self, event: &str, payload: &T) {
        if let Err(e) = self.app.emit_all(event, payload) {
            log::error!("Failed to emit spellcheck event: {}", e);
        }
    }
}

/// Tauri command: Set spellcheck language.
#[tauri::command]
pub fn set_spellcheck_language(state: tauri::State<SpellcheckManager>, lang: String) {
    state.set_language(&lang);
}

/// Tauri command: Get available spellcheck languages.
#[tauri::command]
pub fn get_available_languages() -> Vec<String> {
    SpellcheckManager::get_available_languages()
}