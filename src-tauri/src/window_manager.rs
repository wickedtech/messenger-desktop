// Window Management for Messenger desktop app
// Implements minimize to tray, window state persistence, always-on-top,
// focus mode, zoom, and fullscreen toggle

use anyhow::Result;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Window state for persistence
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct WindowState {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub maximized: bool,
    pub always_on_top: bool,
    pub focus_mode: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            x: -1, // Center by default
            y: -1,
            maximized: false,
            always_on_top: false,
            focus_mode: false,
        }
    }
}

/// Window manager state
#[derive(Debug, Clone)]
pub struct WindowManagerState {
    pub window_state: WindowState,
    pub default_window_state: WindowState,
    pub zoom_level: f64,
    pub saved_positions: Vec<PositionHistory>,
}

/// Position history for tracking window movements
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PositionHistory {
    pub timestamp: u64,
    pub x: i32,
    pub y: i32,
}

/// Window Manager - manages window behavior and state
pub struct WindowManager {
    state: Arc<RwLock<WindowManagerState>>,
    app_data_dir: PathBuf,
}

impl WindowManager {
    /// Create a new window manager
    pub fn new(app_data_dir: PathBuf) -> Self {
        Self {
            state: Arc::new(RwLock::new(WindowManagerState {
                window_state: WindowState::default(),
                default_window_state: WindowState::default(),
                zoom_level: 0.0, // 0.0 = 100% zoom
                saved_positions: Vec::new(),
            })),
            app_data_dir,
        }
    }

    /// Load window state from storage
    pub async fn load_window_state(&self) -> Result<WindowState> {
        debug!("Loading window state");

        let state_file = self.app_data_dir.join("window_state.json");

        if state_file.exists() {
            match fs::read_to_string(&state_file) {
                Ok(contents) => {
                    match serde_json::from_str(&contents) {
                        Ok(state) => {
                            info!("Window state loaded from file");
                            return Ok(state);
                        }
                        Err(e) => {
                            warn!("Failed to parse window state: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read window state file: {}", e);
                }
            }
        }

        // Return default if no state file exists
        Ok(WindowState::default())
    }

    /// Save window state to storage
    pub async fn save_window_state(&self, state: &WindowState) -> Result<()> {
        debug!("Saving window state");

        let state_file = self.app_data_dir.join("window_state.json");

        let contents = serde_json::to_string_pretty(state)?;
        fs::write(&state_file, contents)?;

        info!("Window state saved to file");
        Ok(())
    }

    /// Save current window state
    pub async fn save_current_state(&self) -> Result<()> {
        debug!("Saving current window state");
        
        let state = self.state.read().await;
        self.save_window_state(&state.window_state).await?;
        
        Ok(())
    }

    /// Restore window state
    pub async fn restore_window_state(&self) -> Result<WindowState> {
        debug!("Restoring window state");
        
        let state = self.load_window_state().await?;
        *self.state.write().await = WindowManagerState {
            window_state: state.clone(),
            default_window_state: state.clone(),
            zoom_level: 0.0,
            saved_positions: Vec::new(),
        };
        
        Ok(state)
    }

    /// Update window state
    pub async fn update_window_state(&self, update: WindowState) -> Result<()> {
        debug!("Updating window state");
        
        let mut state = self.state.write().await;
        state.window_state = update;
        
        Ok(())
    }

    /// Toggle always-on-top mode
    pub async fn toggle_always_on_top(&self) -> Result<bool> {
        debug!("Toggling always-on-top");
        
        let mut state = self.state.write().await;
        state.window_state.always_on_top = !state.window_state.always_on_top;
        
        info!("Always-on-top: {}", state.window_state.always_on_top);
        Ok(state.window_state.always_on_top)
    }

    /// Set always-on-top mode
    pub async fn set_always_on_top(&self, enabled: bool) -> Result<()> {
        debug!("Setting always-on-top to: {}", enabled);
        
        let mut state = self.state.write().await;
        state.window_state.always_on_top = enabled;
        
        info!("Always-on-top: {}", enabled);
        Ok(())
    }

    /// Get always-on-top status
    pub async fn is_always_on_top(&self) -> bool {
        self.state.read().await.window_state.always_on_top
    }

    /// Set zoom level
    pub async fn set_zoom(&self, level: f64) -> Result<()> {
        debug!("Setting zoom level to: {}", level);
        
        let mut state = self.state.write().await;
        state.zoom_level = level;
        
        info!("Zoom level: {}%", (level + 1.0) * 100.0);
        Ok(())
    }

    /// Get current zoom level
    pub async fn get_zoom(&self) -> f64 {
        self.state.read().await.zoom_level
    }

    /// Increase zoom level
    pub async fn zoom_in(&self) -> Result<f64> {
        let current = self.get_zoom().await;
        let new_level = current + 0.1;
        self.set_zoom(new_level).await?;
        Ok(new_level)
    }

    /// Decrease zoom level
    pub async fn zoom_out(&self) -> Result<f64> {
        let current = self.get_zoom().await;
        let new_level = current - 0.1;
        self.set_zoom(new_level).await?;
        Ok(new_level)
    }

    /// Reset zoom level
    pub async fn reset_zoom(&self) -> Result<f64> {
        self.set_zoom(0.0).await?;
        Ok(0.0)
    }

    /// Toggle focus mode (hide sidebar, show only chat)
    pub async fn toggle_focus_mode(&self) -> Result<bool> {
        debug!("Toggling focus mode");
        
        let mut state = self.state.write().await;
        state.window_state.focus_mode = !state.window_state.focus_mode;
        
        info!("Focus mode: {}", state.window_state.focus_mode);
        Ok(state.window_state.focus_mode)
    }

    /// Set focus mode
    pub async fn set_focus_mode(&self, enabled: bool) -> Result<()> {
        debug!("Setting focus mode to: {}", enabled);
        
        let mut state = self.state.write().await;
        state.window_state.focus_mode = enabled;
        
        info!("Focus mode: {}", enabled);
        Ok(())
    }

    /// Get focus mode status
    pub async fn is_in_focus_mode(&self) -> bool {
        self.state.read().await.window_state.focus_mode
    }

    /// Toggle maximize/restore window
    pub async fn toggle_maximize(&self) -> Result<bool> {
        debug!("Toggling window maximize");
        
        let mut state = self.state.write().await;
        state.window_state.maximized = !state.window_state.maximized;
        
        info!("Window maximized: {}", state.window_state.maximized);
        Ok(state.window_state.maximized)
    }

    /// Set maximize state
    pub async fn set_maximized(&self, maximized: bool) -> Result<()> {
        debug!("Setting maximize to: {}", maximized);
        
        let mut state = self.state.write().await;
        state.window_state.maximized = maximized;
        
        Ok(())
    }

    /// Get maximize state
    pub async fn is_maximized(&self) -> bool {
        self.state.read().await.window_state.maximized
    }

    /// Toggle fullscreen
    pub async fn toggle_fullscreen(&self) -> Result<bool> {
        debug!("Toggling fullscreen");
        
        // In a real implementation, this would toggle the window fullscreen state
        // window.set_fullscreen(fullscreen)?;
        
        let mut state = self.state.write().await;
        
        info!("Fullscreen toggle requested");
        Ok(!state.window_state.maximized) // Placeholder
    }

    /// Set window position
    pub async fn set_position(&self, x: i32, y: i32) -> Result<()> {
        debug!("Setting window position to: ({}, {})", x, y);
        
        let mut state = self.state.write().await;
        state.window_state.x = x;
        state.window_state.y = y;
        
        // Track position history
        state.saved_positions.push(PositionHistory {
            timestamp: chrono::Utc::now().timestamp() as u64,
            x,
            y,
        });
        
        // Keep only last 100 positions
        if state.saved_positions.len() > 100 {
            state.saved_positions.drain(0..(state.saved_positions.len() - 100));
        }
        
        Ok(())
    }

    /// Set window size
    pub async fn set_size(&self, width: i32, height: i32) -> Result<()> {
        debug!("Setting window size to: {}x{}", width, height);
        
        let mut state = self.state.write().await;
        state.window_state.width = width;
        state.window_state.height = height;
        
        Ok(())
    }

    /// Get current window state
    pub async fn get_window_state(&self) -> WindowState {
        self.state.read().await.window_state.clone()
    }

    /// Reset to default window state
    pub async fn reset_to_default(&self) -> Result<WindowState> {
        debug!("Resetting to default window state");
        
        let default = WindowState::default();
        *self.state.write().await = WindowManagerState {
            window_state: default.clone(),
            default_window_state: default.clone(),
            zoom_level: 0.0,
            saved_positions: Vec::new(),
        };
        
        info!("Window state reset to default");
        Ok(default)
    }

    /// Get zoom level percentage
    pub async fn get_zoom_percentage(&self) -> f64 {
        (self.get_zoom().await + 1.0) * 100.0
    }

    /// Format zoom level for display
    pub async fn format_zoom(&self) -> String {
        format!("{:.0}%", self.get_zoom_percentage().await)
    }

    /// Minimize to tray ( don't quit)
    pub async fn minimize_to_tray(&self) -> Result<()> {
        debug!("Minimizing to tray");
        
        // Save current state before minimizing
        self.save_current_state().await?;
        
        info!("Minimized to tray");
        Ok(())
    }

    /// Restore from tray
    pub async fn restore_from_tray(&self) -> Result<()> {
        debug!("Restoring from tray");
        
        // Restore window state
        self.restore_window_state().await?;
        
        info!("Restored from tray");
        Ok(())
    }

    /// Close the window manager and save state
    pub async fn cleanup(&self) -> Result<()> {
        debug!("Cleaning up window manager");
        
        // Save current state
        self.save_current_state().await?;
        
        info!("Window manager cleanup complete");
        Ok(())
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        // Use standard app data directory
        let app_data_dir = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        Self::new(app_data_dir)
    }
}

// Tauri commands

/// Toggle always-on-top mode
#[tauri::command(async)]
#[specta::specta]
pub async fn toggle_always_on_top(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<bool, anyhow::Error> {
    window_manager.toggle_always_on_top().await
}

/// Set always-on-top mode
#[tauri::command(async)]
#[specta::specta]
pub async fn set_always_on_top(
    enabled: bool,
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<(), anyhow::Error> {
    window_manager.set_always_on_top(enabled).await?;
    Ok(())
}

/// Get always-on-top status
#[tauri::command(async)]
#[specta::specta]
pub async fn is_always_on_top(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<bool, anyhow::Error> {
    Ok(window_manager.is_always_on_top().await)
}

/// Set window zoom level
#[tauri::command(async)]
#[specta::specta]
pub async fn set_zoom(
    level: f64,
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<(), anyhow::Error> {
    window_manager.set_zoom(level).await?;
    Ok(())
}

/// Get current zoom level
#[tauri::command(async)]
#[specta::specta]
pub async fn get_zoom(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<f64, anyhow::Error> {
    Ok(window_manager.get_zoom().await)
}

/// Zoom in
#[tauri::command(async)]
#[specta::specta]
pub async fn zoom_in(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<f64, anyhow::Error> {
    window_manager.zoom_in().await
}

/// Zoom out
#[tauri::command(async)]
#[specta::specta]
pub async fn zoom_out(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<f64, anyhow::Error> {
    window_manager.zoom_out().await
}

/// Reset zoom
#[tauri::command(async)]
#[specta::specta]
pub async fn reset_zoom(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<f64, anyhow::Error> {
    window_manager.reset_zoom().await
}

/// Toggle focus mode
#[tauri::command(async)]
#[specta::specta]
pub async fn toggle_focus_mode(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<bool, anyhow::Error> {
    window_manager.toggle_focus_mode().await
}

/// Set focus mode
#[tauri::command(async)]
#[specta::specta]
pub async fn set_focus_mode(
    enabled: bool,
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<(), anyhow::Error> {
    window_manager.set_focus_mode(enabled).await?;
    Ok(())
}

/// Get focus mode status
#[tauri::command(async)]
#[specta::specta]
pub async fn is_in_focus_mode(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<bool, anyhow::Error> {
    Ok(window_manager.is_in_focus_mode().await)
}

/// Save current window state
#[tauri::command(async)]
#[specta::specta]
pub async fn save_window_state(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<(), anyhow::Error> {
    window_manager.save_current_state().await?;
    Ok(())
}

/// Restore window state
#[tauri::command(async)]
#[specta::specta]
pub async fn restore_window_state(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<WindowState, anyhow::Error> {
    window_manager.restore_window_state().await
}

/// Get current window state
#[tauri::command(async)]
#[specta::specta]
pub async fn get_window_state(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<WindowState, anyhow::Error> {
    Ok(window_manager.get_window_state().await)
}

/// Reset to default window state
#[tauri::command(async)]
#[specta::specta]
pub async fn reset_window_state(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<WindowState, anyhow::Error> {
    window_manager.reset_to_default().await
}

/// Get zoom percentage for display
#[tauri::command(async)]
#[specta::specta]
pub async fn get_zoom_percentage(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<f64, anyhow::Error> {
    Ok(window_manager.get_zoom_percentage().await)
}

/// Get zoom formatted string
#[tauri::command(async)]
#[specta::specta]
pub async fn get_zoom_formatted(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<String, anyhow::Error> {
    Ok(window_manager.format_zoom().await)
}

/// Minimize to tray
#[tauri::command(async)]
#[specta::specta]
pub async fn minimize_to_tray(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<(), anyhow::Error> {
    window_manager.minimize_to_tray().await?;
    Ok(())
}

/// Restore from tray
#[tauri::command(async)]
#[specta::specta]
pub async fn restore_from_tray(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<(), anyhow::Error> {
    window_manager.restore_from_tray().await?;
    Ok(())
}

/// Toggle maximize window
#[tauri::command(async)]
#[specta::specta]
pub async fn toggle_maximize(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<bool, anyhow::Error> {
    window_manager.toggle_maximize().await
}

/// Set maximize state
#[tauri::command(async)]
#[specta::specta]
pub async fn set_maximized(
    maximized: bool,
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<(), anyhow::Error> {
    window_manager.set_maximized(maximized).await?;
    Ok(())
}

/// Get maximize state
#[tauri::command(async)]
#[specta::specta]
pub async fn is_maximized(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<bool, anyhow::Error> {
    Ok(window_manager.is_maximized().await)
}

/// Toggle fullscreen
#[tauri::command(async)]
#[specta::specta]
pub async fn toggle_fullscreen(
    window_manager: tauri::State<'_, WindowManager>,
) -> Result<bool, anyhow::Error> {
    window_manager.toggle_fullscreen().await
}
