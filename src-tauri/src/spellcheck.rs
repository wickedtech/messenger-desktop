//! Spellcheck manager for Tauri app.
//! Handles spellcheck state, WebView communication, and text validation.

use tauri::AppHandle;
use tauri::Manager;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::sync::Mutex;
use anyhow::{Context, Result};
use hunspell::Hunspell;

/// Spellcheck manager state.
#[derive(Debug, Clone)]
pub struct SpellcheckManager {
    app: AppHandle,
    enabled: bool,
    language: String,
    hunspell: Mutex<Option<Hunspell>>,
    dictionaries_dir: PathBuf,
}

impl SpellcheckManager {
    /// Create a new SpellcheckManager.
    pub fn new(app: &AppHandle) -> Result<Self> {
        let dictionaries_dir = app.path_resolver().app_data_dir()
            .context("Failed to resolve app data directory")?
            .join("dictionaries");
        
        if !dictionaries_dir.exists() {
            std::fs::create_dir_all(&dictionaries_dir)
                .context("Failed to create dictionaries directory")?;
        }
        
        Ok(Self {
            app: app.clone(),
            enabled: false,
            language: "en-US".to_string(),
            hunspell: Mutex::new(None),
            dictionaries_dir,
        })
    }
    
    /// Initialize the spellchecker with the current language.
    pub fn initialize(&self) -> Result<()> {
        let aff_path = self.dictionaries_dir.join(format!("{}.aff", self.language));
        let dic_path = self.dictionaries_dir.join(format!("{}.dic", self.language));
        
        if !aff_path.exists() || !dic_path.exists() {
            log::warn!("Dictionary files not found for language: {}", self.language);
            return Ok(());
        }
        
        let hunspell = Hunspell::new(&aff_path, &dic_path)
            .context("Failed to initialize Hunspell")?;
        
        *self.hunspell.lock().unwrap() = Some(hunspell);
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
        *self.hunspell.lock().unwrap() = None;
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
    pub fn is_misspelled(&self, word: &str) -> bool {
        if !self.enabled {
            return false;
        }
        
        let hunspell = self.hunspell.lock().unwrap();
        if let Some(hunspell) = &*hunspell {
            !hunspell.spell(word)
        } else {
            false
        }
    }
    
    /// Get suggestions for a misspelled word.
    pub fn get_suggestions(&self, word: &str) -> Vec<String> {
        if !self.enabled {
            return vec![];
        }
        
        let hunspell = self.hunspell.lock().unwrap();
        if let Some(hunspell) = &*hunspell {
            hunspell.suggest(word)
        } else {
            vec![]
        }
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
        if let Err(e) = self.app.emit_all(event, payload) {
            log::error!("Failed to emit spellcheck event: {}", e);
        }
    }
}

/// Tauri command: Enable spellcheck.
#[tauri::command]
pub fn enable_spellcheck(state: tauri::State<SpellcheckManager>) -> Result<(), String> {
    state.enable().map_err(|e| e.to_string())
}

/// Tauri command: Disable spellcheck.
#[tauri::command]
pub fn disable_spellcheck(state: tauri::State<SpellcheckManager>) {
    state.disable();
}

/// Tauri command: Set spellcheck language.
#[tauri::command]
pub fn set_spellcheck_language(state: tauri::State<SpellcheckManager>, lang: String) -> Result<(), String> {
    state.set_language(&lang).map_err(|e| e.to_string())
}

/// Tauri command: Get available spellcheck languages.
#[tauri::command]
pub fn get_available_languages() -> Vec<String> {
    SpellcheckManager::get_available_languages()
}

/// Tauri command: Check if a word is misspelled.
#[tauri::command]
pub fn is_misspelled(state: tauri::State<SpellcheckManager>, word: String) -> bool {
    state.is_misspelled(&word)
}

/// Tauri command: Get suggestions for a misspelled word.
#[tauri::command]
pub fn get_suggestions(state: tauri::State<SpellcheckManager>, word: String) -> Vec<String> {
    state.get_suggestions(&word)
}

/// Tauri command: Check a text for misspelled words.
#[tauri::command]
pub fn check_text(state: tauri::State<SpellcheckManager>, text: String) -> Vec<(usize, usize, String)> {
    state.check_text(&text)
}