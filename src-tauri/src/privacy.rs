use tauri::{AppHandle, Emitter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PrivacyConfig {
    pub block_typing: bool,
    pub block_read_receipts: bool,
    pub hide_last_active: bool,
    pub block_link_previews: bool,
}

pub struct PrivacyManager {
    pub config: PrivacyConfig,
    app: AppHandle,
}

impl PrivacyManager {
    pub fn new(app: &AppHandle) -> Self {
        Self {
            config: PrivacyConfig::default(),
            app: app.clone(),
        }
    }

    pub fn apply(&self) -> tauri::Result<()> {
        self.app.emit("update-privacy", &self.config)?;
        Ok(())
    }

    pub fn update(&mut self, config: PrivacyConfig) -> tauri::Result<()> {
        self.config = config;
        self.apply()
    }

    pub fn set_block_typing(&mut self, value: bool) -> tauri::Result<()> {
        self.config.block_typing = value;
        self.apply()
    }

    pub fn set_block_read_receipts(&mut self, value: bool) -> tauri::Result<()> {
        self.config.block_read_receipts = value;
        self.apply()
    }

    pub fn set_hide_last_active(&mut self, value: bool) -> tauri::Result<()> {
        self.config.hide_last_active = value;
        self.apply()
    }

    #[allow(dead_code)]
    pub fn set_block_link_previews(&mut self, value: bool) -> tauri::Result<()> {
        self.config.block_link_previews = value;
        self.apply()
    }

    pub fn config(&self) -> &PrivacyConfig {
        &self.config
    }
}

#[tauri::command]
pub fn set_privacy(
    state: tauri::State<std::sync::Mutex<PrivacyManager>>,
    block_typing: bool,
    block_read_receipts: bool,
    hide_last_active: bool,
    block_link_previews: bool,
) -> tauri::Result<()> {
    let new_config = PrivacyConfig {
        block_typing,
        block_read_receipts,
        hide_last_active,
        block_link_previews,
    };

    let mut manager = state.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    manager.update(new_config)
}

#[tauri::command]
pub fn get_privacy(
    state: tauri::State<std::sync::Mutex<PrivacyManager>>,
) -> tauri::Result<PrivacyConfig> {
    let manager = state.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(manager.config().clone())
}

#[tauri::command]
pub fn set_block_typing(
    state: tauri::State<std::sync::Mutex<PrivacyManager>>,
    value: bool,
) -> tauri::Result<()> {
    let mut manager = state.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    manager.set_block_typing(value)
}

#[tauri::command]
pub fn set_block_read_receipts(
    state: tauri::State<std::sync::Mutex<PrivacyManager>>,
    value: bool,
) -> tauri::Result<()> {
    let mut manager = state.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    manager.set_block_read_receipts(value)
}

#[tauri::command]
pub fn set_hide_last_active(
    state: tauri::State<std::sync::Mutex<PrivacyManager>>,
    value: bool,
) -> tauri::Result<()> {
    let mut manager = state.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    manager.set_hide_last_active(value)
}

#[tauri::command]
#[allow(dead_code)]
pub fn set_block_link_previews(
    state: tauri::State<std::sync::Mutex<PrivacyManager>>,
    value: bool,
) -> tauri::Result<()> {
    let mut manager = state.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    manager.set_block_link_previews(value)
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_config_default() {
        let config = PrivacyConfig::default();
        assert!(!config.block_typing);
        assert!(!config.block_read_receipts);
        assert!(!config.hide_last_active);
        assert!(!config.block_link_previews);
    }

    #[test]
    fn test_privacy_config_clone() {
        let config = PrivacyConfig {
            block_typing: true,
            block_read_receipts: true,
            hide_last_active: true,
            block_link_previews: true,
        };
        let cloned = config.clone();
        assert_eq!(config.block_typing, cloned.block_typing);
        assert_eq!(config.block_read_receipts, cloned.block_read_receipts);
    }

    #[test]
    fn test_privacy_config_serialization() {
        let config = PrivacyConfig {
            block_typing: true,
            block_read_receipts: false,
            hide_last_active: true,
            block_link_previews: false,
        };
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: PrivacyConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.block_typing, deserialized.block_typing);
    }

    #[test]
    fn test_privacy_manager_new() {
        // Need AppHandle for testing, so skip actual instantiation
        // This is just a placeholder test
        assert!(true);
    }
}