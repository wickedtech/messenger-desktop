#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_creation() {
        let theme = Theme {
            name: "Dark".to_string(),
            css: "body { background-color: #18191a; }".to_string(),
        };
        assert_eq!(theme.name, "Dark");
        assert_eq!(theme.css, "body { background-color: #18191a; }");
    }
    
    #[test]
    fn test_set_theme() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        let mut manager = ThemeManager::new(window);
        let theme = Theme {
            name: "Dark".to_string(),
            css: "body { background-color: #18191a; }".to_string(),
        };
        manager.set_theme(theme);
    }
    
    #[test]
    fn test_remove_theme() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        let mut manager = ThemeManager::new(window);
        manager.remove_theme();
    }
    
    #[test]
    fn test_get_themes() {
        let themes = ThemeManager::get_themes();
        assert!(themes.len() > 0);
    }
    
    #[test]
    fn test_set_custom_css() {
        let window = tauri::WindowBuilder::new(
            &tauri::AppHandle::default(),
            "test".to_string(),
            tauri::WindowUrl::App("index.html".into()),
        )
        .build()
        .unwrap();
        let manager = ThemeManager::new(window);
        manager.set_custom_css("body { background-color: red; }");
    }
}
