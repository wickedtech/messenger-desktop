# Messenger Desktop

[![Build Status](https://github.com/example/messenger-desktop/actions/workflows/build.yml/badge.svg)](https://github.com/example/messenger-desktop/actions/workflows/build.yml)
[![Release](https://img.shields.io/github/v/release/example/messenger-desktop)](https://github.com/example/messenger-desktop/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Native Facebook Messenger desktop app powered by Tauri 2.x. 10x smaller than Electron.**

## Features

- **WebView Wrapper**: Lightweight wrapper around messenger.com with native performance.
- **Native Notifications**: System notifications for new messages.
- **System Tray Integration**: Unread badge and quick access from the system tray.
- **Global Shortcuts**: Open the app with `Ctrl+Shift+M` (customizable).
- **Themes**: Dark mode, custom themes, and theme switching.
- **Privacy Controls**: Block typing indicators, read receipts, and more.
- **Multi-Account Support**: Switch between multiple Facebook accounts seamlessly.
- **Settings Panel**: Customize behavior, appearance, and privacy settings.
- **Auto-Updater**: Stay up-to-date with the latest features and fixes.
- **Cross-Platform CI**: Builds for Linux, Windows, and macOS via GitHub Actions.

## Screenshots

*(Coming soon!)*

## Download

Download the latest version from the [GitHub Releases](https://github.com/example/messenger-desktop/releases) page.

## Build From Source

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Node.js 18+](https://nodejs.org/)
- Linux: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`

### Steps

1. Clone the repository:
   ```sh
   git clone https://github.com/example/messenger-desktop.git
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

## Project Structure

```
.
├── src-tauri/          # Rust backend
│   ├── src/            # Tauri commands and modules
│   ├── build.rs        # Build script
│   └── Cargo.toml      # Rust dependencies
├── src/               # Frontend (TypeScript/React)
├── public/            # Static assets
├── .github/workflows/ # CI/CD pipelines
└── README.md          # This file
```

## Comparison with Caprine (Electron)

| Feature               | Messenger Desktop (Tauri) | Caprine (Electron) |
|-----------------------|---------------------------|--------------------|
| **Binary Size**       | ~10MB                     | ~100MB             |
| **Memory Usage**      | ~200MB                    | ~800MB             |
| **Startup Time**      | <1s                       | ~3s                |
| **Native Features**   | Yes (tray, notifications) | Limited            |
| **Privacy Controls**  | Yes                       | No                 |
| **Auto-Updater**      | Yes                       | Yes                |
| **Cross-Platform**    | Yes                       | Yes                |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.