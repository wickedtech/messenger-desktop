# Messenger Desktop Code Audit Report
**Branch:** feat/round18-polish  
**Date:** 2026-02-25  
**Files Audited:** 19 Rust + 9 TypeScript = 28 files

---

## Bugs

### High Severity

- **[Severity: High]** `accounts.rs:143` — `state.lock().unwrap()` panics on poisoned mutex. **Fix:** Use `state.lock().map_err(|e| e.to_string())?` like other commands.

- **[Severity: High]** `accounts.rs:149` — `state.lock().unwrap()` panics on poisoned mutex. **Fix:** Handle poison error gracefully.

- **[Severity: High]** `accounts.rs:155` — `state.lock().unwrap()` panics on poisoned mutex. **Fix:** Handle poison error gracefully.

- **[Severity: High]** `accounts.rs:161` — `state.lock().unwrap()` panics on poisoned mutex. **Fix:** Handle poison error gracefully.

- **[Severity: High]** `commands.rs:6` — Type parameter `R` used but never defined ( Clipboard<R> ). **Fix:** Import the correct type parameter from tauri or use concrete type.

- **[Severity: High]** `commands.rs:12` — Same `R` type parameter issue in write_clipboard_text.

- **[Severity: High]** `lib.rs:46` — `spellcheck::SpellcheckManager::new(handle)?` propagates error via `?`, stopping app startup on spellcheck init failure. **Fix:** Log error and continue with disabled spellcheck, or wrap in `match` for graceful degradation.

- **[Severity: High]** `drag_drop.rs:75` — File injection creates empty File object: `new File([], '...')` with no actual content. **Fix:** Read file content via Tauri FS API and pass as ArrayBuffer, or use Tauri's native file dialog.

### Medium Severity

- **[Severity: Med]** `accounts.rs:74-78` — Profile picture processing reads entire image into memory without size validation; large images could cause OOM. **Fix:** Add max file size check before `ImageReader::open()`.

- **[Severity: Med]** `lib.rs:52` — `tokio::sync::Mutex::new(updater)` wrapped in Mutex but `UpdaterManager` has no interior mutability needed - could use RwLock. **Fix:** Remove unnecessary wrapping or use `std::sync::Mutex` if Send bound not needed.

- **[Severity: Med]** `privacy.rs:53` — `anyhow::anyhow!(e.to_string())` wraps PoisonError which is already a string representation, creating nested "poisoned lock: poisoned lock" messages. **Fix:** Use `e.into_inner()` to get the underlying data.

- **[Severity: Med]** `theme_manager.rs:21` — Unknown themes silently fall back to Light without logging. **Fix:** Add `log::warn!()` for unknown theme names.

- **[Severity: Med]** `tray.rs:17-18` — TrayIconId created but not stored in struct, then recreated in `update_unread_count` at line 73. **Fix:** Store `TrayIconId` in `TrayManager` struct.

- **[Severity: Med]** `updater.rs:51` — `check_update()` returns `Ok(None)` but never actually checks for updates - TODO says "implement proper updater". **Fix:** Implement actual update check against releases endpoint.

- **[Severity: Med]** `notifications.rs:135` — `NotificationPayload` struct doesn't include `sender_name` field that's passed in `show_native_macos`. **Fix:** Add missing struct field.

### Low Severity

- **[Severity: Low]** `accounts.rs:125-126` — `save()` returns `Ok(())` on store error silently (only propagates serialization errors). **Fix:** Check return value of `set()` and handle errors.

- **[Severity: Low]** `media.rs:52,57` — `request_camera()` and `request_microphone()` just set bool to true without actual OS permission request. **Fix:** Implement proper permission dialogs using tauri-plugin-os or native APIs.

- **[Severity: Low]** `media.rs:125` — `generate_preview()` creates empty file as placeholder, not actual preview. **Fix:** Implement actual preview generation or remove stub.

- **[Severity: Low]** `spellcheck.rs:81-83` — `get_suggestions()` returns empty vec even when hunspell is disabled - should return error or None. **Fix:** Return `Option<Vec<String>>` to indicate disabled state.

- **[Severity: Low]** `privacy.rs:20` — `block_link_previews` field is settable but getter command `get_privacy` doesn't return this field value (not included in struct return). **Fix:** Add to returned `PrivacyConfig`.

- **[Severity: Low]** `shortcuts.rs:15-18` — `register_all` manages HashMap but never actually registers shortcuts with Tauri global-shortcut plugin. **Fix:** Actually register shortcuts via `app.global_shortcut_manager()`.

- **[Severity: Low]** `tray.rs:32,37,40` — Menu items "mute", "dnd" emit events but no handlers exist in the codebase. **Fix:** Add handlers or remove menu items.

- **[Severity: Low]** `tray.rs:50` — `window.emit("navigate", "settings")` emits to window but handler not found in injection code. **Fix:** Add listener in injection scripts.

- **[Severity: Low]** `linux.rs:38` — `generate_desktop_file` uses `dirs::home_dir()` which can return None, `.expect()` would panic. **Fix:** Use `?` operator with proper error handling.

- **[Severity: Low]** `windows.rs:29` — `get_app_window_handle` returns null pointer placeholder, not actual HWND. **Fix:** Implement actual window handle retrieval.

---

## Code Quality Issues

### Rust

1. **Dead code markers:** `#[allow(dead_code)]` used extensively on actually-used functions (accounts.rs:139, 145, 151, etc.). Remove these unnecessary attributes.

2. **Inconsistent error handling:** Some commands use `map_err(|e| e.to_string())`, others use `?` with anyhow, others panic with `unwrap()`. Standardize on `Result<T, String>` for Tauri commands.

3. **Unused imports:** 
   - `updater.rs:15` - `TokioMutex` imported but unused (shadowed by std::sync::Mutex in some contexts)
   - `linux.rs:10` - `home_dir` imported from dirs but `dirs::home_dir()` used

4. **Magic numbers:** `theme_manager.rs` CSS strings are hardcoded; consider external CSS files. `unread-counter.ts:45` - 2000ms polling interval is magic number.

5. **Overly long functions:** `theme_manager.rs:29-90` `get_css()` function is 60+ lines of inline CSS. Extract to static constants or file-based themes.

6. **Inconsistent naming:** `get_zoom_formatted` vs `format_zoom` in window_manager.rs - command vs method naming differs.

7. **Platform stubs:** Many `#[cfg(target_os = "...")]` functions have empty implementations with only log warnings. Document which features are actually implemented.

8. **Unbounded vectors:** `window_manager.rs:25` `saved_positions` vector grows without limit (though line 208 trims to 100, this is still implicit behavior).

### TypeScript

1. **Type safety:** `src/settings/settings.ts` uses `window.__TAURI__` without type declarations. Should use `@tauri-apps/api` imports.

2. **Null assertions:** `src/settings/settings.ts` uses `!` postfix (e.g., line 45 `document.getElementById('custom-css-row')!.classList`) without null checks. **Fix:** Add null checks or use optional chaining.

3. **Any types:** `src/injection/*.ts` uses `// @ts-ignore` and `any` casting extensively. Define proper interfaces for Tauri API.

4. **Memory leak:** `src/injection/privacy-guard.ts:42` replaces `XMLHttpRequest.prototype.open` and `window.fetch` but never stores original for restoration. Multiple `applyPrivacyRules()` calls stack overrides.

5. **Polling fallback:** `src/injection/unread-counter.ts:45` uses `setInterval` as fallback even when MutationObserver works. Consider removing redundant polling.

6. **Error handling:** `src/keyboard-shortcuts.ts` async functions don't handle unregister failures.

---

## Missing Doc Comments

Format: `module::Item` (file:line)

### Rust

- `accounts::AccountManager::set_session_token` (accounts.rs:97)
- `accounts::AccountManager::get_session_token` (accounts.rs:104)
- `accounts::AccountManager::update_last_sync` (accounts.rs:111)
- `accounts::AccountManager::save` (accounts.rs:125)
- `drag_drop::handle_file_drop` (drag_drop.rs:115)
- `drag_drop::validate_files` (drag_drop.rs:131)
- `commands::read_clipboard_text` (commands.rs:4)
- `commands::write_clipboard_text` (commands.rs:9)
- `commands::print_page` (commands.rs:14)
- `media::MediaManager::setup_permissions` (media.rs:41)
- `media::save_media_file` (media.rs:105)
- `media::get_media_file_command` (media.rs:130)
- `media::generate_preview_command` (media.rs:137)
- `media::delete_media_file_command` (media.rs:144)
- `spellcheck::SpellcheckManager::check_text` (spellcheck.rs:88)
- `platform::linux::send_dbus_notification` (platform/linux.rs:17)
- `platform::linux::generate_desktop_file` (platform/linux.rs:38)
- `platform::macos::set_dock_badge` (platform/macos.rs:17)
- `platform::macos::bounce_dock` (platform/macos.rs:35)
- `platform::windows::set_taskbar_badge` (platform/windows.rs:17)
- `platform::windows::show_toast_notification` (platform/windows.rs:50)

### TypeScript

- `NotificationInterceptor` class (injection/notification-interceptor.ts:4)
- `isInputActive` function (injection/keyboard-shortcuts.ts:4)
- `handleShortcut` function (injection/keyboard-shortcuts.ts:12)
- `applyPrivacyRules` function (injection/privacy-guard.ts:22)
- `updateConfig` function (injection/privacy-guard.ts:55)
- `parseUnreadCount` function (injection/unread-counter.ts:5)
- `updateUnreadCount` function (injection/unread-counter.ts:23)
- `registerShortcuts` function (keyboard-shortcuts.ts:8)
- `unregisterShortcuts` function (keyboard-shortcuts.ts:31)
- `handleKeyDown` function (keyboard-shortcuts.ts:44)

---

## README Gaps

1. **Missing screenshots:** README says "Screenshots coming soon!" but this is placeholder content.

2. **Non-existent ARCHITECTURE.md:** README references `ARCHITECTURE.md` but file does not exist.

3. **Unverified package manager claims:** Homebrew, Snap, AUR instructions provided but may not actually be published.

4. **Missing troubleshooting:** No section on common build failures (e.g., missing `libwebkit2gtk-4.1-dev` on Linux).

5. **Keyboard shortcuts mismatch:** README lists `Ctrl+,` for settings and `Ctrl+1-9` for accounts, but these shortcuts aren't implemented in the codebase.

6. **Missing Windows prerequisites:** No mention of MSVC build tools requirement for Windows.

7. **No mention of pnpm/yarn:** Only npm instructions provided, though pnpm is common in Tauri projects.

8. **Missing dev environment variables:** No documentation of env vars like `MESSENGER_RELEASE_CHANNEL`.

---

## CONTRIBUTING Gaps

### For Humans (CONTRIBUTING.md)

1. **Missing test instructions:** `npm test` mentioned but no test script in package.json.

2. **No pre-commit hooks:** No mention of husky or pre-commit setup.

3. **Missing changelog process:** No instructions on updating CHANGELOG.md.

4. **No debugging guide:** How to enable Rust logs (`RUST_LOG=debug`), how to open DevTools.

5. **Architecture decision records:** No mention of ADRs or where to document design decisions.

### For AI Agents (CONTRIBUTING-AGENTS.md)

1. **Missing troubleshooting section:** Common failures like "failed to run custom build command for gtk" not documented.

2. **No TypeScript build process:** Mentions editing TypeScript but not how it's built/bundled.

3. **Inaccurate function names:** 
   - Module map lists `toggle_typing_indicator()` and `toggle_read_receipts()` in privacy.rs, but actual functions are `set_block_typing()` etc.
   - Module map lists `drag_drop::validate_attachment()` but actual function is `validate_files()`.
   - Module map lists `register_global_hotkey()` but actual is `register_shortcuts()`.

4. **Missing file structure:** Doesn't explain `src/` (frontend) vs `src-tauri/src/` (Rust backend) clearly.

5. **No mention of specta:** Document generation with specta not mentioned.

6. **Platform testing guide:** No guidance on cross-compilation or testing other platforms.

---

## Wiki Topics

1. **Architecture Deep Dive: IPC Bridge** - How Tauri commands work, serialization with specta, error handling patterns

2. **Theme System: CSS Injection** - How themes are applied via JavaScript injection, CSS specificity issues, creating custom themes

3. **Privacy Guard Implementation** - Network interception techniques, XHR/fetch patching, limitations and detection by Facebook

4. **Multi-Account Session Isolation** - How accounts are stored, session token management, switching mechanics

5. **Platform-Specific Features** - What's implemented per platform (macOS dock badges, Windows taskbar, Linux D-Bus)

6. **Spell Check Architecture** - Why hunspell is disabled, alternatives considered, WebView-native spell check options

---

## Summary Statistics

| Category | Count |
|----------|-------|
| High Severity Bugs | 7 |
| Medium Severity Bugs | 7 |
| Low Severity Bugs | 11 |
| Code Quality Issues | 14 |
| Missing Doc Comments (Rust) | 21 |
| Missing Doc Comments (TS) | 10 |
| README Gaps | 8 |
| CONTRIBUTING Gaps | 11 |
