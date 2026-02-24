use tauri::Manager;

mod accounts;
mod drag_drop;
mod media;
mod notifications;
mod platform;
mod privacy;
mod shortcuts;
mod spellcheck;
mod theme_manager;
mod tray;
mod updater;
mod window_manager;

use accounts::*;
use drag_drop::*;
use media::*;
use notifications::*;
use privacy::*;
use shortcuts::*;
use spellcheck::*;
use theme_manager::*;
use tray::*;
use updater::*;
use window_manager::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugins
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--flag".to_owned()]),
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            // Notification service (uses Arc internally)
            let notif_service = NotificationService::new(app_data_dir.clone());
            app.manage(notif_service);

            // Window manager (uses Arc internally)
            let win_mgr = WindowManager::new(app_data_dir.clone());
            app.manage(win_mgr);

            // Tray
            let tray_mgr = TrayManager::new(&handle)?;
            app.manage(std::sync::Mutex::new(tray_mgr));

            // Shortcuts
            let shortcut_mgr = ShortcutManager::new();
            app.manage(std::sync::Mutex::new(shortcut_mgr));
            ShortcutManager::register_all(&handle)?;

            // Theme
            let theme_mgr = ThemeManager::new(&handle);
            app.manage(std::sync::Mutex::new(theme_mgr));

            // Privacy
            let privacy_mgr = PrivacyManager::new(&handle);
            app.manage(std::sync::Mutex::new(privacy_mgr));

            // Accounts
            let account_mgr = AccountManager::new(&handle);
            app.manage(std::sync::Mutex::new(account_mgr));

            // Updater
            let updater_mgr = UpdaterManager::new(&handle);
            app.manage(std::sync::Mutex::new(updater_mgr));

            // Spellcheck
            let spellcheck_mgr = SpellcheckManager::new(&handle);
            app.manage(std::sync::Mutex::new(spellcheck_mgr));

            // Media
            let media_mgr = MediaManager::new(&handle);
            app.manage(std::sync::Mutex::new(media_mgr));

            // Platform-specific init
            platform::init(&handle);

            Ok(())
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
            // Window management
            toggle_always_on_top,
            set_always_on_top,
            is_always_on_top,
            set_zoom,
            get_zoom,
            zoom_in,
            zoom_out,
            reset_zoom,
            toggle_focus_mode,
            set_focus_mode,
            is_in_focus_mode,
            get_window_state,
            restore_window_state,
            save_window_state,
            reset_window_state,
            toggle_fullscreen,
            toggle_maximize,
            set_maximized,
            is_maximized,
            minimize_to_tray,
            restore_from_tray,
            get_zoom_formatted,
            get_zoom_percentage,
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
            set_block_link_previews,
            // Accounts
            add_account,
            remove_account,
            switch_account,
            list_accounts,
            // Updater
            check_update,
            install_update,
            get_current_version,
            // Spellcheck
            set_spellcheck_language,
            get_available_languages,
            // Media
            get_media_permissions,
            grant_media_permission,
            // Drag & drop
            handle_file_drop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
