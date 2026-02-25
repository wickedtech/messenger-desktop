# Messenger Desktop

![CI](https://github.com/wickedtech/messenger-desktop/actions/workflows/build.yml/badge.svg)
[![Release](https://img.shields.io/github/v/release/wickedtech/messenger-desktop)](https://github.com/wickedtech/messenger-desktop/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**A native Facebook Messenger desktop client built with Tauri 2.x and Rust. 10x smaller than Electron.**

Messenger Desktop wraps [`messenger.com`](https://messenger.com) in a native WebView while extending its functionality with powerful system integrations. Get notifications, system tray support, privacy controls, and multi-account support without the bloat of Electron.

---

## ✨ Features

- **🔔 Native Notifications** - Get system notifications for new messages immediately
- **📊 System Tray Integration** - Access Messenger quickly with unread badge count
- **🎯 Keyboard Shortcuts** - Global shortcuts to toggle the window (`Ctrl+Shift+M`)
- **🌙 Themes** - Dark mode, light mode, and system theme support
- **🔒 Privacy Guard** - Block typing indicators, read receipts, and seen status
- **👥 Multi-Account Support** - Switch between multiple Facebook accounts seamlessly
- **📝 Spell Check** - Native spell check support while typing
- **🎥 Media Permissions** - Camera and microphone access for voice/video calls
- **🔄 Auto-Updater** - Stay up-to-date with automatic updates
- **🚀 Lightweight** - Only ~10MB compared to ~100MB for Electron apps
- **💾 Low Memory** - ~200MB RAM vs ~800MB for Electron alternatives

---

## 📸 Screenshots

<!-- Screenshots coming soon! -->

---

## 🚀 Quick Start

```bash
# Clone and install
git clone https://github.com/wickedtech/messenger-desktop.git
cd messenger-desktop
npm install

# Run in development mode (hot reload)
npm run tauri dev

# Run tests
cd src-tauri
cargo test --all

# Build for production
cd ..
npm run tauri build

# Build artifacts are in: src-tauri/target/release/bundle/
```

See [Build from Source](#-build-from-source) for detailed prerequisites and platform-specific instructions.

---

## 📥 Installation

### Pre-built Releases

Download the latest stable release for your platform from the [GitHub Releases page](https://github.com/wickedtech/messenger-desktop/releases).

| Platform | File Size |
|----------|-----------|
| **Windows** (x64) | ~10 MB |
| **macOS** (Apple Silicon x86_64) | ~12 MB |
| **Linux** (AppImage, DEB, RPM) | ~11 MB |

### Package Managers

#### Homebrew (macOS/Linux)

```bash
brew install --cask messenger-desktop
```

#### Snap (Linux)

```bash
sudo snap install messenger-desktop
```

#### AUR (Arch Linux)

```bash
paru -S messenger-desktop
```

---

## 🛠️ Build from Source

### Prerequisites

- **Rust** 1.70+ (stable toolchain)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Node.js** 18+ or 20+
  ```bash
  # Using nvm (recommended)
  nvm install 20
  nvm use 20
  ```
- **System Dependencies** (Linux only)
  ```bash
  # Debian/Ubuntu
  sudo apt update
  sudo apt install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

  # Fedorap
  sudo dnf install -y webkit2gtk4.1-devel appindicator-gtk3-devel

  # Arch Linux
  sudo pacman -S webkit2gtk-4.1 libappindicator-gtk3 librsvg
  ```

### Clone & Build

```bash
# Clone the repository
git clone https://github.com/wickedtech/messenger-desktop.git
cd messenger-desktop

# Install JavaScript dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

The production build artifacts will be in `src-tauri/target/release/bundle/`.

---

## ⚙️ Configuration

Messenger Desktop stores configuration in the platform-specific app data directory:

| Platform | Config Location |
|----------|-----------------|
| **Linux** | `~/.config/messenger-desktop/` |
| **macOS** | `~/Library/Application Support/messenger-desktop/` |
| **Windows** | `%APPDATA%\messenger-desktop\` |

### Key Settings

- **Theme**: Choose between light, dark, or follow system theme
- **Privacy**: Enable/disable typing indicators, read receipts, seen status
- **Notifications**: Configure notification style (native/banners/sounds)
- **Shortcuts**: Customize global hotkeys for toggling window
- **Accounts**: Manage logged-in accounts and switching behavior
- **Startup**: Launch app on system boot

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+M` | Toggle window show/hide (global) |
| `Ctrl+N` | Open new conversation |
| `Ctrl+,` | Open settings |
| `Ctrl+1`...`Ctrl+9` | Switch to account 1-9 |
| `Ctrl+Tab` | Switch to next open chat |
| `Ctrl+Shift+Tab` | Switch to previous open chat |
| `Ctrl+W` | Close current chat |
| `Escape` | Close window or dismiss dialogs |
| `Ctrl+Q` | Quit application (macOS: `Cmd+Q`) |

*Note: On macOS, replace `Ctrl` with `Cmd`.*

---

## 🔒 Privacy Features

Messenger Desktop includes a robust privacy guard that intercepts and blocks tracking features:

- **🚫 Block Typing Indicators** - The other user won't see when you're typing
- **🚫 Block Read Receipts** - Messages won't be marked as "seen"
- **🚫 Block Seen Status** - Your presence status won't be updated
- **🚫 Block Delivery Receipts** - Message delivery status won't be sent
- **🚫 Block Message Reactions** - Emoji reactions won't be transmitted

These features work at the network layer by intercepting XHR/fetch requests before they leave the WebView. The original Messenger UI is not modified, so the experience feels authentic.

---

## 🏗️ Architecture

Messenger Desktop uses a hybrid architecture with Tauri's IPC bridge connecting the WebView frontend to the Rust backend:

```
┌─────────────────────────────────────────────────────────────┐
│                    WebView Layer                            │
│              (messenger.com + Injected JS)                  │
│                                                              │
│  Injected Scripts:                                          │
│  • privacy-guard.js  (block typing, read receipts)         │
│  • theme-injection.js  (custom CSS themes)                 │
│  • keyboard-shortcuts.js  (intercept key events)           │
│  • notification-interceptor.js  (forward events)           │
└─────────────────────────────┬───────────────────────────────┘
                              │ Tauri IPC (invoke/emit)
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Rust Backend Modules                      │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   accounts   │  │  tray        │  │ notifications│      │
│  │ (accounts.rs)│  │ (tray.rs)    │  │ (notifications│      │
│  │              │  │              │  │    .rs)      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   privacy    │  │  shortcuts   │  │ spellcheck   │      │
│  │ (privacy.rs) │  │(shortcuts.rs)│  │(spellcheck.  │      │
│  │              │  │              │  │    rs)       │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │theme_manager │  │    media     │  │   updater    │      │
│  │(theme_mgr.rs)│  │  (media.rs)  │  │ (updater.rs) │      │
│  │              │  │              │  │              │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐                       │
│  │drag_drop     │  │window_manager│                       │
│  │(drag_drop.rs)│  │(window_mgr.rs│                       │
│  │              │  │    .rs)      │                       │
│  └──────────────┘  └──────────────┘                       │
│                                                              │
│  ┌────────────────────────────────────────────┐           │
│  │         Platform Abstraction Layer         │           │
│  │  platform/linux.rs │ macos.rs │ windows.rs │          │
│  └────────────────────────────────────────────┘           │
└─────────────────────────────────────────────────────────────┘
```

**How it works:**

1. **WebView Layer** - Loads `messenger.com` in a lightweight browser window
2. **JS Injection Layer** - Custom scripts injected to intercept network requests and DOM events
3. **Rust Backend** - Native system integrations (tray, notifications, themes, etc.)
4. **IPC Bridge** - Bidirectional communication between WebView and Rust via Tauri's IPC system

---

## 📦 Module Overview

The Rust backend is organized into focused modules:

| Module | Description |
|--------|-------------|
| `accounts` | Multi-account session management |
| `commands` | Tauri command handlers |
| `drag_drop` | File drag-and-drop support |
| `media` | Camera/microphone permissions for calls |
| `notifications` | Native system notifications |
| `privacy` | Privacy guard (block typing, read receipts) |
| `shortcuts` | Global keyboard shortcuts |
| `spellcheck` | Native spell check integration |
| `theme_manager` | Theme switching (dark/light/system) |
| `tray` | System tray icon and unread badge |
| `updater` | Automatic update checking |
| `window_manager` | Window lifecycle and positioning |

---

## 🤝 Contributing

We welcome contributions from both humans and AI agents! For detailed guidelines, see [`CONTRIBUTING.md`](CONTRIBUTING.md) for human contributors and [`CONTRIBUTING-AGENTS.md`](CONTRIBUTING-AGENTS.md) for AI agents.

### Quick Start for Humans

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test` and `npm test`)
5. Commit with conventional commits (`feat: add amazing feature`)
6. Push to your fork (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Quick Start for AI Agents

See [`CONTRIBUTING-AGENTS.md`](CONTRIBUTING-AGENTS.md) for:
- Architecture overview and module map
- Development workflow (clone → setup → build → test)
- Code style guidelines
- PR conventions
- Testing requirements

---

## 🐛 Troubleshooting

### Build Errors

#### Linux: `failed to run custom build command for gtk`
```
error: failed to run custom build command for gtk4-sys v0.x
```
**Solution:** Install required GTK dependencies:
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

#### Linux: `package libwebkit2gtk-4.1-0 was not found`
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel

# Arch Linux
sudo pacman -S webkit2gtk-4.1
```

#### macOS: `error: linker 'cc' not found`
**Solution:** Install Xcode Command Line Tools:
```bash
xcode-select --install
```

#### Windows: `error: linker 'link.exe' not found`
**Solution:** Install [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) or Visual Studio with C++ workload.

#### Rust: `error: package 'xxx' failed to resolve`
**Solution:** Update Rust toolchain and clear cargo cache:
```bash
rustup update stable
cargo clean
```

#### Tauri CLI errors
```bash
# Reinstall Tauri CLI
npm uninstall -D @tauri-apps/cli
npm install -D @tauri-apps/cli@latest
```

### Runtime Issues

#### Notifications not showing on Linux
Ensure you have `libnotify` and Gnome/KDE notification daemon running:
```bash
# Ubuntu/Debian
sudo apt install libnotify-bin

# Test notifications
notify-send "Test" "Notification works!"
```

### Spell check not working
Install system spell check dictionaries:
```bash
# Linux (hunspell)
sudo apt install hunspell-en-us  # English
sudo apt install hunspell-fr     # French

# macOS
# Spell check uses built-in dictionaries

# Windows
# Spell check uses OS spell check (Windows 10/11 only)
```

### App won't launch (permissions denied)
Ensure the binary has execute permissions:
```bash
chmod +x messenger-desktop
```

For Linux AppImage, extract and run directly:
```bash
./Messenger-Desktop-x.y.z.AppImage
```

### Theme not following system
Set the theme to "System" in settings, then ensure your OS theme is properly configured:
- **GNOME (Linux)**: Settings → Appearance
- **KDE (Linux)**: System Settings → Appearance → Global Theme
- **macOS**: System Settings → Appearance
- **Windows**: Settings → Personalization → Colors

---

## 🔭 Comparison with Caprine (Electron)

| Feature | Messenger Desktop | Caprine |
|---------|-------------------|---------|
| **Binary Size** | ~10 MB | ~100 MB |
| **Memory Usage** | ~200 MB | ~800 MB |
| **Startup Time** | <1s | ~3s |
| **Custom Themes** | Yes | Yes |
| **Privacy Controls** | Advanced | Basic |
| **Multi-Account** | Yes | No |
| **Spell Check** | Native | Limited |
| **Auto-Updater** | Yes | Yes |
| **Tech Stack** | Tauri + Rust | Electron + Node |

---

## 📜 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - The framework that makes this possible
- [Caprine](https://github.com/sindresorhus/caprine) - Inspiration and reference
- [Messenger](https://messenger.com) - The web app we wrap

---

## 📞 Support

- **GitHub Issues**: [Open an issue](https://github.com/wickedtech/messenger-desktop/issues) for bugs and feature requests
- **Discussions**: [Join discussions](https://github.com/wickedtech/messenger-desktop/discussions) for community support
- **Documentation**: Check [`ARCHITECTURE.md`](ARCHITECTURE.md) for technical deep dives

---

<p align="center">
  <i>Made with ❤️ by the wickedtech team</i>
</p>