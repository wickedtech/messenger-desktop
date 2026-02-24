# Rust Compilation Errors Fix Plan

## Root Cause 1: async_kind errors (27 errors)
Fix `#[tauri::command]` async functions to return `Result<T, String>` instead of `Result<T, anyhow::Error>`:
- [ ] `notifications.rs` (9 functions)
- [ ] `window_manager.rs` (16 functions)
- [ ] `media.rs` (1 function)

## Root Cause 2: shortcuts.rs (28 errors)
Fix global shortcut registration issues:
- [ ] Change `register_shortcut` to take `&AppHandle` instead of `&GlobalShortcut`
- [ ] Add `use tauri::Manager;`
- [ ] Fix error handling for `keys.parse()`
- [ ] Add type annotations

## Root Cause 3: Misc errors (13)
- [ ] `linux.rs:43` - string concat
- [ ] `lib.rs:164` - `block_link_previews` → `set_block_link_previews`
- [ ] `lib.rs:175` - `get_accounts` → `list_accounts`
- [ ] `lib.rs:186-187` - remove non-existent clipboard functions
- [ ] `lib.rs:70,73` - fix `PrivacyManager::new()` and `ThemeManager::new()` args
- [ ] `lib.rs:79` - fix `UpdaterManager::new()` arg
- [ ] `lib.rs:82` - fix `TrayManager::new()` arg
- [ ] `lib.rs:85` - fix `WindowManager::new()` arg
- [ ] `tauri.conf.json` - `devPath` → `devUrl`
- [ ] `spellcheck.rs:38` - remove non-existent `hunspell` field
- [ ] `spellcheck.rs:109` - `emit_all` → `emit`
- [ ] `tray.rs:17` - `default_icon()` → `default_window_icon()`
- [ ] `tray.rs:24` - fix callback app handle access
- [ ] `tray.rs:65,117` - fix `tray_by_id` Option handling
- [ ] `drag_drop.rs:118` - fix `paths.to_vec()`
- [ ] `updater.rs:105,111` - fix Mutex issue

## Verification
- [ ] Run `cargo check` after each file
- [ ] Run final `cargo check` - should have 0 errors
- [ ] Git commit and push
