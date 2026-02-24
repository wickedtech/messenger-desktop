use tauri::Manager;

// Import all the command functions
use crate::notifications::{
    show_notification, set_dnd, toggle_dnd, is_dnd_enabled, set_notification_sound,
    get_notification_settings, set_notification_enabled, set_notification_sound_enabled,
    use_default_notification_sound
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

// Clipboard commands and print command are defined in their respective modules

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
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
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

            // Initialize spellchecker
            let spellchecker = crate::spellcheck::SpellcheckManager::new(&handle)?;

            // Initialize updater
            let updater = crate::updater::UpdaterManager::new(&handle);

            // Initialize tray
            let tray = crate::tray::TrayManager::new(&handle);

            // Initialize window manager
            let window_manager = crate::window_manager::WindowManager::new(app_data_dir.clone());

            // Initialize shortcut manager
            let shortcut_manager = crate::shortcuts::ShortcutManager::new();

            app.manage(notif_service);
            app.manage(privacy_manager);
            app.manage(theme_manager);
            app.manage(spellchecker);
            app.manage(tokio::sync::Mutex::new(updater));
            app.manage(tray);
            app.manage(window_manager);
            app.manage(Mutex::new(shortcut_manager));

            // Initialize platform-specific features
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}