use tauri::{AppHandle, Emitter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    Light,
    Dark,
    Darker,
    OledBlack,
    Custom(String),
}

#[derive(Serialize, Clone)]
struct ThemePayload {
    name: String,
    css: String,
}

pub struct ThemeManager {
    current: Theme,
    app: AppHandle,
}

impl ThemeManager {
    pub fn new(app: &AppHandle) -> Self {
        Self {
            current: Theme::Light,
            app: app.clone(),
        }
    }

    pub fn set_theme(&mut self, name: &str) -> tauri::Result<()> {
        let theme = match name {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            "darker" => Theme::Darker,
            "oled-black" => Theme::OledBlack,
            "custom" => Theme::Custom(String::new()),
            _ => Theme::Light,
        };

        self.current = theme.clone();
        let css = Self::get_css(&theme);

        self.app.emit("set-theme", ThemePayload {
            name: name.to_string(),
            css,
        })?;

        Ok(())
    }

    pub fn set_custom_css(&mut self, css: String) -> tauri::Result<()> {
        self.current = Theme::Custom(css.clone());
        self.app.emit("set-theme", ThemePayload {
            name: "custom".to_string(),
            css,
        })?;
        Ok(())
    }

    pub fn get_css(theme: &Theme) -> String {
        match theme {
            Theme::Light => String::new(),
            Theme::Dark => r#"
                body{background:#1a1a2e!important;color:#e0e0e0!important;}
                [role="main"]{background:#1a1a2e!important;}
                [role="navigation"]{background:#16213e!important;border-color:#0f3460!important;}
                div[role="button"]{background:#16213e!important;color:#e0e0e0!important;}
                [data-testid="mwthreadlist"]{background:#1a1a2e!important;}
                [data-testid="mwthreadlist_item"]{background:#16213e!important;border-color:#0f3460!important;}
                input,textarea{background:#16213e!important;color:#e0e0e0!important;border-color:#0f3460!important;}
                [role="banner"]{background:#16213e!important;border-color:#0f3460!important;}
                span:not([role="img"]){color:#e0e0e0!important;}
                [role="heading"]{color:#ffffff!important;}
                [role="listitem"]{background:#16213e!important;border-color:#0f3460!important;}
                svg[role="img"]{color:#e0e0e0!important;}
                [data-testid="mwcomposer"]{background:#16213e!important;}
                [data-testid="mwthreadlist_header"]{background:#1a1a2e!important;border-color:#0f3460!important;}
                ::-webkit-scrollbar{background:#1a1a2e!important;}
                ::-webkit-scrollbar-thumb{background:#0f3460!important;}
            "#.to_string(),
            Theme::Darker => r#"
                body{background:#0d0d1a!important;color:#e0e0e0!important;}
                [role="main"]{background:#0d0d1a!important;}
                [role="navigation"]{background:#0a0a14!important;border-color:#1a1a2e!important;}
                div[role="button"]{background:#0a0a14!important;color:#e0e0e0!important;}
                [data-testid="mwthreadlist"]{background:#0d0d1a!important;}
                [data-testid="mwthreadlist_item"]{background:#0a0a14!important;border-color:#1a1a2e!important;}
                input,textarea{background:#0a0a14!important;color:#e0e0e0!important;border-color:#1a1a2e!important;}
                [role="banner"]{background:#0a0a14!important;border-color:#1a1a2e!important;}
                span:not([role="img"]){color:#e0e0e0!important;}
                [role="heading"]{color:#ffffff!important;}
                [role="listitem"]{background:#0a0a14!important;border-color:#1a1a2e!important;}
                svg[role="img"]{color:#e0e0e0!important;}
                [data-testid="mwcomposer"]{background:#0a0a14!important;}
                [data-testid="mwthreadlist_header"]{background:#0d0d1a!important;border-color:#1a1a2e!important;}
                ::-webkit-scrollbar{background:#0d0d1a!important;}
                ::-webkit-scrollbar-thumb{background:#1a1a2e!important;}
            "#.to_string(),
            Theme::OledBlack => r#"
                body{background:#000000!important;color:#e0e0e0!important;}
                [role="main"]{background:#000000!important;}
                [role="navigation"]{background:#0a0a0a!important;border-color:#1a1a1a!important;}
                div[role="button"]{background:#0a0a0a!important;color:#e0e0e0!important;}
                [data-testid="mwthreadlist"]{background:#000000!important;}
                [data-testid="mwthreadlist_item"]{background:#0a0a0a!important;border-color:#1a1a1a!important;}
                input,textarea{background:#0a0a0a!important;color:#e0e0e0!important;border-color:#1a1a1a!important;}
                [role="banner"]{background:#0a0a0a!important;border-color:#1a1a1a!important;}
                span:not([role="img"]){color:#e0e0e0!important;}
                [role="heading"]{color:#ffffff!important;}
                [role="listitem"]{background:#0a0a0a!important;border-color:#1a1a1a!important;}
                svg[role="img"]{color:#e0e0e0!important;}
                [data-testid="mwcomposer"]{background:#0a0a0a!important;}
                [data-testid="mwthreadlist_header"]{background:#000000!important;border-color:#1a1a1a!important;}
                ::-webkit-scrollbar{background:#000000!important;}
                ::-webkit-scrollbar-thumb{background:#1a1a1a!important;}
            "#.to_string(),
            Theme::Custom(css) => css.clone(),
        }
    }

    pub fn get_themes() -> Vec<String> {
        vec![
            "light".to_string(),
            "dark".to_string(),
            "darker".to_string(),
            "oled-black".to_string(),
            "custom".to_string(),
        ]
    }

    pub fn current_theme(&self) -> &Theme {
        &self.current
    }
}

#[tauri::command]
pub fn set_theme(
    state: tauri::State<std::sync::Mutex<ThemeManager>>,
    name: String,
) -> tauri::Result<()> {
    let mut manager = state.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    manager.set_theme(&name)
}

#[tauri::command]
pub fn get_themes() -> Vec<String> {
    ThemeManager::get_themes()
}

#[tauri::command]
pub fn set_custom_css(
    state: tauri::State<std::sync::Mutex<ThemeManager>>,
    css: String,
) -> tauri::Result<()> {
    let mut manager = state.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    manager.set_custom_css(css)
}

#[tauri::command]
pub fn current_theme_name(
    state: tauri::State<std::sync::Mutex<ThemeManager>>,
) -> tauri::Result<String> {
    let manager = state.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(match manager.current_theme() {
        Theme::Light => "light",
        Theme::Dark => "dark",
        Theme::Darker => "darker",
        Theme::OledBlack => "oled-black",
        Theme::Custom(_) => "custom",
    }.to_string())
}