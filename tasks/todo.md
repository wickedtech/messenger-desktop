# Tray Menu Bug Fixes - Task List

## Bugs to Fix

- [x] Bug 1 (CRITICAL) — Add missing window label to tauri.conf.json
- [x] Bug 2 (CRITICAL) — Add `navigate` event listener to index.ts
- [x] Bug 3 (HIGH) — Add `global-shortcut-trigger` listener to index.ts
- [x] Bug 4 (MEDIUM) — Fix TrayManager state management in lib.rs
- [x] Bug 5 (MEDIUM) — Add macOS foreground activation
- [x] Bug 6 (LOW) — Create Tauri 2 capabilities file

## Verification Steps

- [x] Run `cargo check --manifest-path src-tauri/Cargo.toml` (N/A - Rust/Cargo not available)
- [x] Run `npm run build` or `npx vite build` ✓ Success
- [x] Fix any errors before committing (none found)

## Commit

- [ ] Commit all fixes with message: `fix(tray): fix macOS tray menu - window label, event listeners, state management`
