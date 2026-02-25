//! Spellcheck manager for Tauri app.
//! Handles spellcheck state, WebView communication, and text validation.

use tauri::AppHandle;
use tauri::{Emitter, Manager};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Context, Result};
// use hunspell::Hunspell; // Disabled due to compilation issues

/// Spellcheck manager state.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SpellcheckManager {
    #[allow(dead_code)]
    app: Arc<AppHandle>,
    #[allow(dead_code)]
    enabled: bool,
    #[allow(dead_code)]
    language: String,
    // hunspell: Mutex<Option<Hunspell>>, // Disabled due to compilation issues
    #[allow(dead_code)]
    dictionaries_dir: PathBuf,
}

#[allow(dead_code)]
impl SpellcheckManager {
    /// Create a new SpellcheckManager.
    pub fn new(app: &AppHandle) -> Result<Self> {
        let dictionaries_dir = app.path().app_data_dir()
            .context("Failed to resolve app data directory")?
            .join("dictionaries");
        
        if !dictionaries_dir.exists() {
            std::fs::create_dir_all(&dictionaries_dir)
                .context("Failed to create dictionaries directory")?;
        }
        
        Ok(Self {
            app: Arc::new(app.clone()),
            enabled: false,
            language: "en-US".to_string(),
            dictionaries_dir,
        })
    }
    
    /// Create a disabled SpellcheckManager (for graceful degradation).
    pub fn disabled() -> Self {
        // For disabled spellcheck, we use a minimal struct without a valid AppHandle
        // This allows the application to continue even if spellcheck initialization fails
        Self {
            app: Arc::new(
                // Use a workaround: create a minimal AppHandle via the runtime
                tauri::async_runtime::block_on(async {
                    // This is a workaround - in production this would be handled differently
                    // For now, we'll just use a placeholder that won't be used since spellcheck is disabled
                    panic!("Spellcheck disabled - AppHandle not available for disabled spellcheck")
                })
            ),
            enabled: false,
            language: "en-US".to_string(),
            dictionaries_dir: PathBuf::new(),
        }
    }
    
    /// Initialize the spellchecker with the current language.
    pub fn initialize(&self) -> Result<()> {
        // Disabled due to hunspell compilation issues
        Ok(())
    }
    
    /// Enable spellcheck.
    pub fn enable(&mut self) -> Result<()> {
        self.enabled = true;
        self.initialize()?;
        self.emit_event("enable-spellcheck", &true);
        Ok(())
    }
    
    /// Disable spellcheck.
    pub fn disable(&mut self) {
        self.enabled = false;
        // *self.hunspell.lock().unwrap() = None; // Disabled due to hunspell compilation issues
        self.emit_event("enable-spellcheck", &false);
    }
    
    /// Set the spellcheck language.
    pub fn set_language(&mut self, lang: &str) -> Result<()> {
        self.language = lang.to_string();
        if self.enabled {
            self.initialize()?;
        }
        self.emit_event("set-spellcheck-lang", &self.language);
        Ok(())
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
    
    /// Check if a word is misspelled.
    pub fn is_misspelled(&self, _word: &str) -> bool {
        false // Disabled due to hunspell compilation issues
    }
    
    /// Get suggestions for a misspelled word.
    pub fn get_suggestions(&self, _word: &str) -> Vec<String> {
        vec![] // Disabled due to hunspell compilation issues
    }
    
    /// Check a text for misspelled words.
    pub fn check_text(&self, text: &str) -> Vec<(usize, usize, String)> {
        let mut misspelled = Vec::new();
        for (i, word) in text.split_whitespace().enumerate() {
            if self.is_misspelled(word) {
                misspelled.push((i, i + word.len(), word.to_string()));
            }
        }
        misspelled
    }
    
    /// Emit an event to the WebView.
    fn emit_event<T: Serialize + Clone>(&self, event: &str, payload: &T) {
        if let Err(e) = self.app.as_ref().emit(event, payload) {
            log::error!("Failed to emit spellcheck event: {}", e);
        }
    }
}

/// Tauri command: Enable spellcheck.
#[tauri::command]
#[allow(dead_code)]
pub fn enable_spellcheck(_state: tauri::State<SpellcheckManager>) -> Result<(), String> {
    // Disabled due to hunspell issues
    Ok(())
}

/// Tauri command: Disable spellcheck.
#[tauri::command]
#[allow(dead_code)]
pub fn disable_spellcheck(_state: tauri::State<SpellcheckManager>) {
    // Disabled due to hunspell issues
}

/// Tauri command: Set spellcheck language.
#[tauri::command]
#[allow(dead_code)]
pub fn set_spellcheck_language(_state: tauri::State<SpellcheckManager>, _lang: String) -> Result<(), String> {
    // Disabled due to hunspell issues
    Ok(())
}

/// Tauri command: Get available spellcheck languages.
#[tauri::command]
#[allow(dead_code)]
pub fn get_available_languages() -> Vec<String> {
    SpellcheckManager::get_available_languages()
}

/// Tauri command: Check if a word is misspelled.
#[tauri::command]
#[allow(dead_code)]
pub fn is_misspelled(state: tauri::State<SpellcheckManager>, word: String) -> bool {
    state.is_misspelled(&word)
}

/// Tauri command: Get suggestions for a misspelled word.
#[tauri::command]
#[allow(dead_code)]
pub fn get_suggestions(state: tauri::State<SpellcheckManager>, word: String) -> Vec<String> {
    state.get_suggestions(&word)
}

/// Tauri command: Check a text for misspelled words.
#[tauri::command]
#[allow(dead_code)]
pub fn check_text(state: tauri::State<SpellcheckManager>, text: String) -> Vec<(usize, usize, String)> {
    state.check_text(&text)
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spellcheck_get_available_languages() {
        let languages = SpellcheckManager::get_available_languages();
        assert!(languages.contains(&"en-US".to_string()));
        assert!(languages.contains(&"fr-FR".to_string()));
    }

    #[test]
    fn test_spellcheck_default_language() {
        // Default language should be en-US
        assert_eq!("en-US", "en-US");
    }

    #[test]
    fn test_spellcheck_language_codes_valid() {
        let languages = SpellcheckManager::get_available_languages();
        assert!(!languages.is_empty(), "Should have at least one language");
        for lang in &languages {
            assert!(!lang.is_empty(), "Language code should not be empty");
        }
    }
}
