use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct UpdateInfo {
    pub version: String,
    pub body: String,
}

pub struct UpdaterManager {
    app: AppHandle,
    pub channel: String,
}

impl UpdaterManager {
    pub fn new(app: &AppHandle) -> Self {
        let channel = std::env::var("MESSENGER_RELEASE_CHANNEL").unwrap_or_else(|_| "stable".to_string());
        Self { app: app.clone(), channel }
    }

    pub async fn check_update(&self) -> tauri::Result<Option<UpdateInfo>> {
        let updater = self.app.updater()?;
        if let Some(update) = updater.check().await? {
            Ok(Some(UpdateInfo {
                version: update.version.to_string(),
                body: update.body,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn install_update(&self) -> tauri::Result<()> {
        let updater = self.app.updater()?;
        let handle = updater.download_and_install(|downloaded, total| {
            self.app.emit("update-progress", {
                "downloaded": downloaded,
                "total": total,
            }).unwrap();
        }).await?;
        handle.await?;
        Ok(())
    }

    pub fn get_current_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

#[tauri::command]
pub async fn check_update(state: tauri::State<'_, std::sync::Mutex<UpdaterManager>>) -> tauri::Result<Option<UpdateInfo>> {
    state.lock().unwrap().check_update().await
}

#[tauri::command]
pub async fn install_update(state: tauri::State<'_, std::sync::Mutex<UpdaterManager>>) -> tauri::Result<()> {
    state.lock().unwrap().install_update().await
}

#[tauri::command]
pub fn get_current_version(state: tauri::State<'_, std::sync::Mutex<UpdaterManager>>) -> String {
    state.lock().unwrap().get_current_version()
}