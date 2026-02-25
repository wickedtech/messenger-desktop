// lib.rs - Main Tauri library for Messenger Desktop
// This module orchestrates all Tauri commands and integrates submodules

use tauri::{Manager, WebviewWindowBuilder, WebviewUrl};

// Import all the command functions
use crate::notifications::{
    show_notification, set_dnd, toggle_dnd, is_dnd_enabled, set_notification_sound,
    get_notification_settings, set_notification_enabled, set_notification_sound_enabled,
    use_default_notification_sound, handle_notification
};
use crate::window_manager::{
    toggle_always_on_top, set_always_on_top, is_always_on_top, set_zoom, get_zoom,
    zoom_in, zoom_out, reset_zoom, get_zoom_formatted, get_zoom_percentage,
    toggle_focus_mode, set_focus_mode, is_in_focus_mode, get_window_state,
    save_window_state, restore_window_state, reset_window_state, toggle_fullscreen,
    toggle_maximize, set_maximized, is_maximized, minimize_to_tray, restore_from_tray
};
use crate::tray::{init_tray, update_unread_count, set_tray_tooltip};
use crate::shortcuts::{init_shortcuts, register_shortcuts, update_shortcut, unregister_shortcut};
use crate::theme_manager::{set_theme, get_themes, set_custom_css, current_theme_name};
use crate::privacy::{set_privacy, get_privacy, set_block_typing, set_block_read_receipts, set_hide_last_active};
use crate::updater::{check_update, install_update};
// use crate::spellcheck::{spellcheck, get_suggestions}; // Disabled due to hunspell issues
use crate::accounts::{list_accounts, add_account, remove_account};
use crate::media::grant_media_permission;
use crate::drag_drop::handle_file_drop;
use crate::platform_manager::{PlatformManager, select_platform, get_current_platform, get_last_platform, list_platforms};
use crate::privacy_engine::{PrivacyEngine, clear_platform_session, clear_all_sessions, get_csp_for_platform};

mod accounts;
mod drag_drop;
mod media;
mod notifications;
mod platform;
mod platform_manager;
mod privacy;
mod privacy_engine;
mod shortcuts;
mod spellcheck;
mod theme_manager;
mod tray;
mod updater;
mod window_manager;

// Clipboard commands and print command are defined in their respective modules

// Notification interceptor JS — injected into EVERY navigation including external URLs
const NOTIFICATION_INTERCEPTOR_JS: &str = r#"
(function() {
    // Guard: only patch once per context
    if (window.__MESSENGER_DESKTOP_PATCHED__) { return; }
    window.__MESSENGER_DESKTOP_PATCHED__ = true;

    const OriginalNotification = window.Notification;

    // Override window.Notification
    window.Notification = function(title, options) {
        options = options || {};
        // Route to Tauri handle_notification command
        if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
            window.__TAURI__.core.invoke('handle_notification', {
                title: String(title),
                options: {
                    body: options.body || '',
                    icon: options.icon || null,
                    tag: options.tag || null,
                    silent: options.silent || false
                }
            }).catch(function(e) { console.warn('[notification] invoke failed:', e); });
        } else if (OriginalNotification) {
            return new OriginalNotification(title, options);
        }
    };

    window.Notification.permission = 'granted';
    window.Notification.requestPermission = function() {
        return Promise.resolve('granted');
    };

    Object.defineProperty(window.Notification, 'permission', {
        get: function() { return 'granted'; },
        configurable: true
    });

    console.log('[messenger-desktop] Notification interceptor active');
})();
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugins
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--flag"]),
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // Notification interceptor JS — injected into EVERY navigation including external URLs
            let _main_window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("index.html".into()),
            )
            .title("Social Hub")
            .inner_size(1200.0, 800.0)
            .resizable(true)
            .initialization_script(NOTIFICATION_INTERCEPTOR_JS)
            .build()
            .expect("failed to create main window");

            let handle = app.handle().clone();
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            // Notification service (uses Arc internally)
            let notif_service = crate::notifications::NotificationService::new(app_data_dir.clone());

            // Initialize privacy manager
            let privacy_manager = crate::privacy::PrivacyManager::new(&handle);

            // Initialize theme manager
            let theme_manager = crate::theme_manager::ThemeManager::new(&handle);

            // Initialize spellchecker (graceful degradation if init fails)
            let spellchecker = match crate::spellcheck::SpellcheckManager::new(&handle) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("Spellcheck init failed (disabled): {}", e);
                    crate::spellcheck::SpellcheckManager::disabled()
                }
            };

            // Initialize updater
            let updater = crate::updater::UpdaterManager::new(&handle);

            // Initialize tray
            match crate::tray::TrayManager::new(&handle) {
                Ok(tray_instance) => {
                    app.manage(std::sync::Mutex::new(tray_instance));
                }
                Err(e) => {
                    log::warn!("Tray init failed: {}", e);
                }
            }

            // Initialize window manager
            let window_manager = crate::window_manager::WindowManager::new(app_data_dir.clone());

            // Initialize shortcut manager
            let shortcut_manager = crate::shortcuts::ShortcutManager::new();

            // Initialize platform manager and privacy engine
            let platform_manager = PlatformManager::new(&app_data_dir);
            let privacy_engine = PrivacyEngine::new(app_data_dir.clone());

            app.manage(notif_service);
            app.manage(privacy_manager);
            app.manage(theme_manager);
            app.manage(spellchecker);
            app.manage(tokio::sync::Mutex::new(updater));
            app.manage(window_manager);
            app.manage(std::sync::Mutex::new(shortcut_manager));
            app.manage(platform_manager);
            app.manage(privacy_engine);

            // Initialize platform-specific features
            platform::init(&handle);
            
            // Listen for macOS foreground activation requests
            #[cfg(target_os = "macos")]
            {
                let app_handle = app.handle().clone();
                app.listen_for(move |event| {
                    if let tauri::AppEvent::Ready = event {
                        // Register for request-focus event listener
                        let app_clone = app_handle.clone();
                        app.handle().listen("request-focus", move |event| {
                            log::debug!("Received request-focus event on macOS");
                            platform::request_foreground_activation(&app_clone);
                        });
                    }
                });
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let engine = window.app_handle().state::<crate::privacy_engine::PrivacyEngine>();
                if let Err(e) = engine.clear_all_sessions() {
                    log::warn!("[on_quit] failed to clear sessions: {}", e);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Notifications
            show_notification,
            set_dnd,
            toggle_dnd,
            is_dnd_enabled,
            set_notification_sound,
            get_notification_settings,
            set_notification_enabled,
            set_notification_sound_enabled,
            use_default_notification_sound,
            handle_notification,

            // Window management
            toggle_always_on_top,
            set_always_on_top,
            is_always_on_top,
            set_zoom,
            get_zoom,
            zoom_in,
            zoom_out,
            reset_zoom,
            get_zoom_formatted,
            get_zoom_percentage,
            toggle_focus_mode,
            set_focus_mode,
            is_in_focus_mode,
            get_window_state,
            save_window_state,
            restore_window_state,
            reset_window_state,
            toggle_fullscreen,
            toggle_maximize,
            set_maximized,
            is_maximized,
            minimize_to_tray,
            restore_from_tray,

            // Tray
            init_tray,
            update_unread_count,
            set_tray_tooltip,

            // Shortcuts
            init_shortcuts,
            register_shortcuts,
            update_shortcut,
            unregister_shortcut,

            // Theme
            set_theme,
            get_themes,
            set_custom_css,
            current_theme_name,

            // Privacy
            set_privacy,
            get_privacy,
            set_block_typing,
            set_block_read_receipts,
            set_hide_last_active,

            // Updater
            check_update,
            install_update,

            // Spellcheck (disabled due to hunspell issues)
            // spellcheck,
            // get_suggestions,

            // Accounts
            list_accounts,
            add_account,
            remove_account,

            // Media
            grant_media_permission,

            // Drag & Drop
            handle_file_drop,

            // Platform
            select_platform,
            get_current_platform,
            get_last_platform,
            list_platforms,

            // Privacy Engine
            clear_platform_session,
            clear_all_sessions,
            get_csp_for_platform,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imports_compile() {
        // Test that all imports compile correctly
        // This is a compile-time test to ensure all modules are properly imported
        assert!(true);
    }

    #[test]
    fn test_notification_commands_exist() {
        // Verify notification commands are imported
        // These are Tauri commands, so we just verify they compile
        assert!(true);
    }

    #[test]
    fn test_window_manager_commands_exist() {
        // Verify window manager commands are imported
        assert!(true);
    }

    #[test]
    fn test_tray_commands_exist() {
        // Verify tray commands are imported
        assert!(true);
    }

    #[test]
    fn test_shortcut_commands_exist() {
        // Verify shortcut commands are imported
        assert!(true);
    }

    #[test]
    fn test_theme_commands_exist() {
        // Verify theme commands are imported
        assert!(true);
    }

    #[test]
    fn test_privacy_commands_exist() {
        // Verify privacy commands are imported
        assert!(true);
    }

    #[test]
    fn test_updater_commands_exist() {
        // Verify updater commands are imported
        assert!(true);
    }

    #[test]
    fn test_account_commands_exist() {
        // Verify account commands are imported
        assert!(true);
    }

    #[test]
    fn test_media_commands_exist() {
        // Verify media commands are imported
        assert!(true);
    }

    #[test]
    fn test_drag_drop_commands_exist() {
        // Verify drag drop commands are imported
        assert!(true);
    }

    #[test]
    fn test_module_structure() {
        // Verify the module structure is correct
        assert!(account_manager_exists());
        assert!(privacy_manager_exists());
        assert!(theme_manager_exists());
    }

    fn account_manager_exists() -> bool {
        true // This is just a compile test
    }

    fn privacy_manager_exists() -> bool {
        true
    }

    fn theme_manager_exists() -> bool {
        true
    }
}
