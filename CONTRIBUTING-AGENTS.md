# Contributing for AI Agents

Welcome, fellow AI agents! This guide is specifically for AI agents contributing to the **Messenger Desktop** project (`wickedtech/messenger-desktop`).

---

## 🤖 About This Project

**Messenger Desktop** is a native Facebook Messenger client built with:
- **Tauri 2.x** - Cross-platform desktop app framework
- **Rust** - Backend for system integrations (tray, notifications, themes, privacy)
- **TypeScript/JavaScript injection** - Script injection into the WebView to intercept Messenger behavior
- **WebView** - Loads `messenger.com` in a lightweight browser window

The app is ~10x smaller and 4x more memory-efficient than Electron alternatives.

---

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                WebView Layer (messenger.com)            │
│                                                           │
│  Injected Scripts:                                        │
│  - privacy.js (block typing indicators, read receipts)   │
│  - themes.js (apply custom themes)                        │
│  - shortcuts.js (intercept keyboard events)               │
│  - notifications.js (forward new message events)         │
└────────────────────┬────────────────────────────────────┘
                     │ Tauri IPC
                     │ invoke() / emit()
                     ▼
┌─────────────────────────────────────────────────────────┐
│                   Rust Backend                           │
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │    Tray      │  │ Notifications│  │  Shortcuts   │  │
│  │  (tray.rs)   │  │(notifications│  │(shortcuts.rs)│  │
│  │              │  │    .rs)      │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │    Privacy   │  │   Themes     │  │   Accounts   │  │
│  │ (privacy.rs) │  │ (theme_mgr   │  │ (accounts.rs)│  │
│  │              │  │    .rs)      │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │    Media     │  │  Spellcheck  │  │   Updater    │  │
│  │  (media.rs)  │  │(spellcheck.  │  │(updater.rs)  │  │
│  │              │  │    rs)       │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│                                                           │
│  ┌─────────────────────────────────────────────────┐    │
│  │           Platform Abstraction                  │    │
│  │  platform/linux.rs │ platform/macos.rs │ ...    │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

---

## 📦 Module Map: src-tauri/src/

Each module has a focused responsibility. When contributing, work on one module at a time.

| Module | File | Purpose | Key Functions |
|--------|------|---------|---------------|
| **Accounts** | `accounts.rs` | Multi-account session management | `switch_account()`, `list_accounts()`, `remove_account()` |
| **Commands** | `commands.rs` | Tauri command registration | Registration exports for all modules |
| **Drag & Drop** | `drag_drop.rs` | File upload via drag-and-drop | `handle_file_drop()`, `validate_attachment()` |
| **Media** | `media.rs` | Camera/microphone permissions | `request_media_permission()`, `check_permission_status()` |
| **Notifications** | `notifications.rs` | Native system notifications | `send_notification()`, `set_notification_actions()` |
| **Privacy** | `privacy.rs` | Block typing/seen/read receipts | `toggle_typing_indicator()`, `toggle_read_receipts()` |
| **Shortcuts** | `shortcuts.rs` | Global keyboard shortcuts | `register_global_hotkey()`, `unregister_hotkey()` |
| **Spellcheck** | `spellcheck.rs` | Native spell check | `enable_spellcheck()`, `get_suggestions()` |
| **Theme Manager** | `theme_manager.rs` | Theme switching | `set_theme()`, `get_theme()`, `apply_theme_to_webview()` |
| **Tray** | `tray.rs` | System tray icon + badge | `create_tray_icon()`, `update_badge()`, `show_tray_menu()` |
| **Updater** | `updater.rs` | Auto-update checking | `check_for_updates()`, `download_update()`, `install_update()` |
| **Window Manager** | `window_manager.rs` | Window lifecycle | `create_window()`, `toggle_window()`, `position_window()` |

### Platform-Specific Modules

| Module | Platform | Purpose |
|--------|----------|---------|
| `platform/linux.rs` | Linux | D-Bus integration, AppIndicator, freedesktop notifications |
| `platform/macos.rs` | macOS | NSStatusItem, NSUserNotification, Touch Bar support |
| `platform/windows.rs` | Windows | Tray icon, Windows Notifications, Taskbar badge |

---

## 🔄 Development Workflow

### Phase 1: Clone & Setup

```bash
# 1. Clone the repository
git clone https://github.com/wickedtech/messenger-desktop.git
cd messenger-desktop

# 2. Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Install Node.js 18+
nvm install 20
nvm use 20

# 4. Install dependencies
npm install

# 5. Run in development mode (hot reload enabled)
npm run tauri dev
```

### Phase 2: Build & Test

```bash
# Run Rust tests
cd src-tauri
cargo test --all

# Run with logging (debug mode)
RUST_LOG=debug npm run tauri dev

# Build for release
npm run tauri build

# Check build artifacts
ls -lh src-tauri/target/release/bundle/
```

### Phase 3: Make Changes

1. **Rust changes**: Edit `src-tauri/src/*.rs`
2. **Injection script changes**: Edit `src/` or `src-tauri/src/injections/`
3. **Build and test**: `npm run tauri dev`

### Phase 4: Submit Changes

```bash
# Create a new branch
git checkout -b agent/feature-name

# Commit with conventional commits
git add .
git commit -m "feat(modulename): add feature description"

# Push and open PR
git push origin agent/feature-name
```

---

## 📐 Code Style Guidelines

### Rust

Follow these conventions:

- **Use `cargo fmt`** before committing:
  ```bash
  cargo fmt --all
  ```

- **Use `cargo clippy`** for linting:
  ```bash
  cargo clippy --all-targets --all-features -- -D warnings
  ```

- **Prefer Result<T, E> over Option<T>** for errors
- **Use `thiserror` crate** for custom error types
- **Document public functions** with `///` doc comments
- **Keep modules focused** — each module should have one primary responsibility

Example:

```rust
/// Switches to the given account ID.
///
/// # Arguments
/// * `account_id` - The account to switch to
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(AccountError)` if the account doesn't exist or switching fails
pub async fn switch_account(account_id: &str) -> Result<(), AccountError> {
    // ...
}
```

### TypeScript/JavaScript Injection

- **Use ES modules**: `export`/`import`
- **Minimize DOM polling** — use `MutationObserver` instead
- **Avoid inline styles** — use CSS classes
- **Clean up on unload** — remove event listeners and observers

Example:

```typescript
// ✅ Good: Use MutationObserver
const observer = new MutationObserver((mutations) => {
  // Handle DOM changes
  handleNewMessages();
});

// ❌ Bad: Polling
setInterval(() => {
  if (document.querySelector('.new-message')) {
    handleNewMessages();
  }
}, 1000);

// Always clean up
window.addEventListener('unload', () => {
  observer.disconnect();
});
```

---

## 🔀 PR Conventions

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `refactor` - Code refactoring (no behavior change)
- `perf` - Performance improvement
- `test` - Test additions/modifications
- `chore` - Build process, tooling, dependency updates

**Scopes:** Use the module name (e.g., `tray`, `notifications`, `privacy`)

Examples:
```
feat(tray): add support for dark mode tray icon on Linux
fix(notifications): prevent duplicate notifications when app is focused
refactor(accounts): simplify session storage logic
```

### PR Title Format

Use the same format as commit messages:
```
feat(tray): add support for dark mode tray icon on Linux
```

### PR Description Template

```markdown
## Description
Brief description of what this PR does.

## Changes
- [ ] Added X
- [ ] Fixed Y
- [ ] Z now works

## Testing
- [x] Tested on Linux
- [x] Tested on macOS
- [ ] Tested on Windows

## Checklist
- [x] Code follows style guidelines
- [x] Cargo fmt/clippy passes
- [x] Tests pass
- [x] Documentation updated (if needed)
```

---

## 🧪 Testing Requirements

### Unit Tests

Rust modules should have unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_account_valid() {
        // Test implementation
    }

    #[test]
    fn test_switch_account_invalid() {
        // Test implementation
    }
}
```

### Integration Tests

Test the full flow via Tauri's IPC:

```rust
#[cfg(test)]
mod integration_tests {
    use tauri::Manager;

    #[tokio::test]
    async fn test_account_switching() {
        let app = tauri::test::mock_app();
        // Test IPC commands
    }
}
```

### Manual Testing

Before submitting a PR:

1. **Test on your target platform** (Linux/macOS/Windows)
2. **Test edge cases** (empty state, invalid inputs, network failures)
3. **Test UI behavior** (tray icon, notifications, window positioning)
4. **Test all shortcuts** (global hotkeys work as expected)
5. **Test memory usage** (no leaks, clean shutdown)

---

## 🚨 Common Pitfalls

### ❌ Deadlocking the UI Thread
- **Don't** block the main thread with `thread::sleep()` or sync I/O
- **Do** use `async`/`await` and spawn tokio tasks for background work

```rust
// ❌ Bad
fn block_ui() {
    std::thread::sleep(Duration::from_secs(5)); // Blocks UI!
}

// ✅ Good
async fn non_block() {
    tokio::time::sleep(Duration::from_secs(5)).await; // Non-blocking
}
```

### ❌ Leaking Event Listeners
- **Don't** forget to remove listeners on unload
- **Do** store listener handles and clean up

```rust
// In JS injection scripts
window.addEventListener('unload', () => {
  // Clean up observers, event listeners, intervals
});
```

### ❌ Hardcoding Platform Paths
- **Don't** use `/home/user/.config/messenger-desktop` directly
- **Do** use `tauri::api::path::app_data_dir()`

```rust
// ❌ Bad
let config_path = "/home/user/.config/messenger-desktop";

// ✅ Good
let config_path = app_data_dir(&config).unwrap().join("messenger-desktop");
```

---

## 📚 Additional Resources

- **Tauri Docs**: https://tauri.app/v1/guides/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Project Architecture**: [`ARCHITECTURE.md`](ARCHITECTURE.md)
- **Human Contributors Guide**: [`CONTRIBUTING.md`](CONTRIBUTING.md)

---

## 🎯 Suggested Tasks for AI Agents

Looking for a starting point? Here are some good first tasks:

1. **Add keyboard shortcut customization** - Allow users to remap hotkeys (`shortcuts.rs`)
2. **Implement notification grouping** - Group notifications from the same sender (`notifications.rs`)
3. **Add theme editor** - UI for creating custom CSS themes (`theme_manager.rs`)
4. **Improve spell check** - Add context-aware suggestions (`spellcheck.rs`)
5. **Add message search** - Search across message history (new module: `search.rs`)

Check the [GitHub Issues](https://github.com/wickedtech/messenger-desktop/issues) for labeled `good first issue`.

---

<p align="center">
  <i>Happy hacking, fellow agents! 🤖</i>
</p>