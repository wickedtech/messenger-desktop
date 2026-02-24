use tauri::{AppHandle, Manager, Emitter};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub data_dir: String,
    pub is_active: bool,
}

pub struct AccountManager {
    accounts: Vec<Account>,
    app: AppHandle,
}

impl AccountManager {
    pub fn new(app: &AppHandle) -> Self {
        let accounts = app.state::<tauri_plugin_store::Store>()
            .get("accounts")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        Self { accounts, app: app.clone() }
    }

    pub fn add_account(&mut self, name: String) -> tauri::Result<Account> {
        let id = Uuid::new_v4().to_string();
        let app_data = self.app.path().app_data_dir().unwrap();
        let data_dir = app_data.join("accounts").join(&id);
        std::fs::create_dir_all(&data_dir)?;
        
        let account = Account {
            id: id.clone(),
            name,
            data_dir: data_dir.to_string_lossy().into_owned(),
            is_active: self.accounts.is_empty(),
        };
        
        self.accounts.push(account.clone());
        self.save()?;
        Ok(account)
    }

    pub fn remove_account(&mut self, id: &str) -> tauri::Result<()> {
        if let Some(pos) = self.accounts.iter().position(|a| a.id == id) {
            let data_dir = &self.accounts[pos].data_dir;
            std::fs::remove_dir_all(data_dir)?;
            self.accounts.remove(pos);
            self.save()?;
        }
        Ok(())
    }

    pub fn switch_account(&mut self, id: &str) -> tauri::Result<()> {
        for account in &mut self.accounts {
            account.is_active = account.id == id;
        }
        self.save()?;
        self.app.emit("switch-account", id)?;
        if let Some(window) = self.app.get_window("main") {
            window.set_title(&format!("Messenger - {}", id))?;
        }
        Ok(())
    }

    pub fn list_accounts(&self) -> Vec<Account> {
        self.accounts.clone()
    }

    fn save(&self) {
        let _ = self.app.state::<tauri_plugin_store::Store>()
            .set("accounts", serde_json::to_value(&self.accounts).unwrap());
    }
}

#[tauri::command]
pub fn add_account(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, name: String) -> tauri::Result<Account> {
    state.lock().unwrap().add_account(name)
}

#[tauri::command]
pub fn remove_account(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, id: String) -> tauri::Result<()> {
    state.lock().unwrap().remove_account(&id)
}

#[tauri::command]
pub fn switch_account(state: tauri::State<'_, std::sync::Mutex<AccountManager>>, id: String) -> tauri::Result<()> {
    state.lock().unwrap().switch_account(&id)
}

#[tauri::command]
pub fn list_accounts(state: tauri::State<'_, std::sync::Mutex<AccountManager>>) -> Vec<Account> {
    state.lock().unwrap().list_accounts()
}