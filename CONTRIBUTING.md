# Contributing

Thank you for considering contributing to Messenger Desktop! Here's how you can help:

## 📦 Module Map

The Rust backend (`src-tauri/src/`) is organized into focused modules. Each module handles a specific domain:

| Module | File | Purpose |
|--------|------|---------|
| **Accounts** | `accounts.rs` | Multi-account session management, account switching, profile data |
| **Commands** | `commands.rs` | Tauri command registration (clipboard, print) |
| **Drag & Drop** | `drag_drop.rs` | File upload via drag-and-drop, file validation and MIME type detection |
| **Media** | `media.rs` | Camera/microphone permissions for voice/video calls |
| **Notifications** | `notifications.rs` | Native system notifications with platform-specific implementations |
| **Privacy** | `privacy.rs` | Privacy guard (block typing indicators, read receipts, seen status) |
| **Shortcuts** | `shortcuts.rs` | Global keyboard shortcuts registration and management |
| **Spellcheck** | `spellcheck.rs` | Native spell check integration (currently disabled due to hunspark) |
| **Theme Manager** | `theme_manager.rs` | Theme switching (dark/light/system), CSS injection into WebView |
| **Tray** | `tray.rs` | System tray icon, unread badge count, tray menu |
| **Updater** | `updater.rs` | Automatic update checking and installation |
| **Window Manager** | `window_manager.rs` | Window lifecycle, positioning, zoom management, saved states |

### Platform-Specific Modules (`src-tauri/src/platform/`)

| Module | Platform | Purpose |
|--------|----------|---------|
| `linux.rs` | Linux | D-Bus integration, AppIndicator, freedesktop notifications, desktop file generation |
| `macos.rs` | macOS | NSStatusItem (dock icon), NSUserNotification, Touch Bar support |
| `windows.rs` | Windows | Windows taskbar badge, toast notifications, window handle retrieval |

## Development Setup

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Node.js 18+](https://nodejs.org/)
- Linux: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`

### Steps

1. Fork the repository and clone your fork:
   ```sh
   git clone https://github.com/your-username/messenger-desktop.git
   cd messenger-desktop
   ```

2. Install dependencies:
   ```sh
   npm install
   ```

3. Run in development mode:
   ```sh
   npm run tauri dev
   ```

4. Build for production:
   ```sh
   npm run tauri build
   ```

## Code Style

- **Rust**: Use `rustfmt` for formatting.
  ```sh
  cargo fmt
  ```

- **TypeScript**: Use `prettier` for formatting.
  ```sh
  npx prettier --write src/
  ```

## Pull Request Process

1. Create a new branch for your feature or bugfix:
   ```sh
   git checkout -b feature/your-feature-name
   ```

2. Commit your changes with a descriptive message:
   ```sh
   git commit -m "feat: add new theme support"
   ```

3. Push your branch to your fork:
   ```sh
   git push origin feature/your-feature-name
   ```

4. Open a Pull Request (PR) against the `main` branch of the upstream repository.

5. Wait for review and address any feedback.

## Adding a New Feature

### Example: Adding a New Tauri Command

If you need to expose a new Rust function to the frontend, follow these steps:

1. **Define the function in the appropriate module** (e.g., `src-tauri/src/your_module.rs`):

   ```rust
   use tauri::State;

   pub struct YourModule {
       // Module state
   }

   #[tauri::command]
   pub async fn your_command(
       state: State<Mutex<YourModule>>,
       param1: String,
       param2: i32,
   ) -> Result<String, String> {
       let module = state.lock().map_err(|e| e.to_string())?;

       // Your logic here
       let result = format!("Processed: {} + {}", param1, param2);

       Ok(result)
   }
   ```

2. **Register the command in `src-tauri/src/lib.rs`**:

   ```rust
   mod your_module;

   fn run() {
       tauri::Builder::default()
           .manage(Mutex::new(your_module::YourModule::new()))
           .invoke_handler(tauri::generate_handler![
               your_module::your_command,
               // ... other commands
           ])
           .run(tauri::generate_context!())
           .expect("error while running tauri application");
   }
   ```

3. **Call the command from the frontend (TypeScript/JavaScript)**:

   ```typescript
   import { invoke } from '@tauri-apps/api/core';

   async function doSomething() {
       try {
           const result = await invoke<string>('your_command', {
               param1: 'hello',
               param2: 42
           });
           console.log(result);
       } catch (error) {
           console.error('Error:', error);
       }
   }
   ```

4. **Test the command**:
   - Run `npm run tauri dev` to test in development
   - Write unit tests in the module (see Testing Guidelines below)
   - Test the frontend integration

### Example: Adding a New Theme

1. Create a new theme file in `src/themes/` (e.g., `src/themes/dracula.ts`).
2. Define the theme colors and styles:
   ```ts
   export const dracula = {
     background: "#282a36",
     text: "#f8f8f2",
     // ...
   };
   ```
3. Register the theme in `src/themes/index.ts`.
4. Test the theme locally:
   ```sh
   npm run tauri dev
   ```

## Testing Guidelines

### Running Tests

#### Rust Tests
```bash
cd src-tauri
cargo test --all

# Run specific test
cargo test test_feature_name

# Run tests with output
cargo test -- --nocapture

# Watch mode (requires cargo-watch: cargo install cargo-watch)
cargo watch -x "test --all"
```

#### Integration Tests
To test full IPC command flow:
```bash
# Run with logging
RUST_LOG=debug npm run tauri dev
```

### What to Test

#### New Features
1. **Happy path**: Verify the feature works as intended
2. **Edge cases**: Test with empty inputs, invalid values, boundary conditions
3. **Error handling**: Verify errors are gracefully reported to the frontend
4. **Platform-specific**: Test on all supported platforms if applicable

#### Tauri Commands
1. **Command registration**: Verify it's in `invoke_handler!` in `lib.rs`
2. **Input validation**: Test with valid and invalid inputs
3. **Error propagation**: Ensure errors are returned as `Result<T, String>`
4. **State management**: Test concurrent access if using `Mutex`/`RwLock`

#### Bug Fixes
1. **Regression**: Ensure existing tests still pass
2. **Reproduction**: Verify the bug is actually fixed with a test case
3. **Side effects**: Check that other features aren't broken

### Writing Unit Tests

Add unit tests at the bottom of each module file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_basic() {
        // Test implementation
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_feature_edge_case() {
        // Edge case implementation
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization() {
        // Verify data structures serialize/deserialize correctly
        let json = serde_json::to_string(&data).unwrap();
        let deserialized: DataType = serde_json::from_str(&json).unwrap();
        assert_eq!(data, deserialized);
    }
}
```

### Manual Testing Checklist

Before submitting a PR, verify:

- [ ] Build succeeds on all target platforms you can access
- [ ] App launches without errors
- [ ] Core features work (tray, notifications, window management)
- [ ] No console errors in DevTools (open with `Alt+Cmd+I` on macOS, `Ctrl+Shift+I` on Linux/Windows)
- [ ] Memory usage is reasonable (no leaks observed)
- [ ] Feature-specific tests pass

## Reporting Issues

- Use the [GitHub Issues](https://github.com/example/messenger-desktop/issues) page.
- Include steps to reproduce, screenshots, and logs if applicable.

## License

By contributing, you agree that your contributions will be licensed under the **MIT License**.