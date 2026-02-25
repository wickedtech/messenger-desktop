# Round 19: Multi-Platform Support Architecture Spec

**Status**: Draft  
**Target**: Tauri 2 + Rust  
**Scope**: Add support for Instagram, Messenger, Facebook, and X as separate messaging platforms

---

## Platform Enum

Rust enum defining supported messaging platforms:

```rust
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "messenger")]
    Messenger,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "x")]
    X,
}

impl Platform {
    /// Returns the string identifier used for data directory naming
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Instagram => "instagram",
            Platform::Messenger => "messenger",
            Platform::Facebook => "facebook",
            Platform::X => "x",
        }
    }

    /// Returns the display name for UI presentation
    pub fn display_name(&self) -> &'static str {
        match self {
            Platform::Instagram => "Instagram",
            Platform::Messenger => "Messenger",
            Platform::Facebook => "Facebook",
            Platform::X => "X",
        }
    }

    /// Returns the default URL for this platform
    pub fn default_url(&self) -> &'static str {
        match self {
            Platform::Instagram => "https://www.instagram.com/direct/inbox/",
            Platform::Messenger => "https://www.messenger.com",
            Platform::Facebook => "https://www.facebook.com/messages/",
            Platform::X => "https://x.com/messages",
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Platform {
    fn default() -> Self {
        Platform::Messenger
    }
}
```

---

## URL Mapping

Exact URLs for each supported platform:

| Platform   | URL                                        | Purpose                    |
|------------|--------------------------------------------|----------------------------|
| Instagram  | `https://www.instagram.com/direct/inbox/`  | Direct messages inbox      |
| Messenger  | `https://www.messenger.com`                | Standalone messenger       |
| Facebook   | `https://www.facebook.com/messages/`       | Facebook messages          |
| X          | `https://x.com/messages`                   | X/Twitter direct messages  |

---

## Navigate API

Tauri 2 provides the `WebviewWindow::navigate()` method for programmatic navigation:

### Cargo.toml Dependency

Add to `src-tauri/Cargo.toml` (if not already present):

```toml
[dependencies]
url = "2"
```

### Usage Pattern

```rust
use tauri::WebviewWindow;
use url::Url;

/// Navigate the webview window to a platform URL
pub fn navigate_to_platform(
    window: &WebviewWindow,
    platform: Platform,
) -> anyhow::Result<()> {
    let target_url = platform.default_url();
    let parsed_url = Url::parse(target_url)?;
    window.navigate(parsed_url);
    Ok(())
}

/// Generic navigate to any URL
pub fn navigate_to(
    window: &WebviewWindow,
    url: &str,
) -> anyhow::Result<()> {
    let parsed_url = Url::parse(url)?;
    window.navigate(parsed_url);
    Ok(())
}
```

### Alternative: Window Re-creation

For complete session isolation during platform switches, consider:

```rust
use tauri::{WebviewWindowBuilder, WebviewUrl};

/// Re-create the window with a new data directory
pub fn recreate_window_with_platform(
    app: &tauri::AppHandle,
    platform: Platform,
) -> anyhow::Result<WebviewWindow> {
    // Close existing window
    if let Some(existing) = app.get_webview_window("main") {
        existing.close()?;
    }
    
    // Create new window with platform-specific data directory
    let window = WebviewWindowBuilder::new(app, "main")
        .title(format!("Messenger Desktop - {}", platform.display_name()))
        .build()?;
    
    let url = Url::parse(platform.default_url())?;
    window.navigate(url);
    
    Ok(window)
}
```

---

## Privacy Model

### Session Isolation Strategy

1. **Separate Data Directories**
   - Each platform uses an isolated subdirectory under `app_data_dir/<platform>/`
   - Prevents cookie/localStorage bleed between platforms

2. **Directory Structure**
   ```
   app_data_dir/
   ├── instagram/
   │   ├── cookies/
   │   ├── localStorage/
   │   └── cache/
   ├── messenger/
   │   ├── cookies/
   │   ├── localStorage/
   │   └── cache/
   ├── facebook/
   │   └── ...
   └── x/
       └── ...
   ```

3. **Implementation in Rust**

```rust
use std::path::PathBuf;
use tauri::AppHandle;

/// Get platform-specific data directory
pub fn get_platform_data_dir(
    app: &AppHandle,
    platform: Platform,
) -> anyhow::Result<PathBuf> {
    let app_data = app.path().app_data_dir()?;
    Ok(app_data.join(platform.as_str()))
}

/// Clear session data for a specific platform
pub fn clear_platform_session(
    app: &AppHandle,
    platform: Platform,
) -> anyhow::Result<()> {
    let platform_dir = get_platform_data_dir(app, platform)?;
    if platform_dir.exists() {
        std::fs::remove_dir_all(&platform_dir)?;
    }
    Ok(())
}

/// Clear all platform session data (on app quit)
pub fn clear_all_sessions(app: &AppHandle) -> anyhow::Result<()> {
    for platform in [Platform::Instagram, Platform::Messenger, Platform::Facebook, Platform::X] {
        clear_platform_session(app, platform)?;
    }
    Ok(())
}
```

### Cookie Clearing on Platform Switch

```rust
use tauri::Manager;

/// Clear cookies for the current webview window
pub fn clear_webview_cookies(window: &tauri::Window) -> anyhow::Result<()> {
    // Tauri 2 webview cookie clearing
    window.with_webview(|webview| {
        // Platform-specific cookie clearing
        #[cfg(target_os = "linux")]
        webview.clear_cookies().ok();
        
        #[cfg(target_os = "macos")]
        webview.clear_cookies().ok();
        
        #[cfg(target_os = "windows")]
        webview.clear_cookies().ok();
    })?;
    Ok(())
}
```

### Telemetry Blocking (CSP)

Telemetry domains to block via CSP:
- `*.doubleclick.net`
- `*.googlesyndication.com`
- `*.google-analytics.com`

See [CSP Per Platform](#csp-per-platform) for full CSP strings.

---

## Platform Selector Flow

### Startup Sequence

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   App Launch    │────▶│  Load Selector   │────▶│  User Selects   │
│                 │     │  (local HTML)    │     │    Platform     │
└─────────────────┘     └──────────────────┘     └────────┬────────┘
                                                          │
                              ┌───────────────────────────┘
                              ▼
                    ┌────────────────────┐     ┌─────────────────┐
                    │ Re-init Injection  │────▶│  Navigate to    │
                    │   (Rust/JS bridge) │     │  Platform URL   │
                    └────────────────────┘     └─────────────────┘
```

### Step-by-Step Flow

1. **App Launch** → Tauri creates the main window
2. **Load Selector** → Navigate to local `selector.html` embedded in app bundle
3. **User Selection** → User clicks platform button; Rust receives the event
4. **Navigate** → Rust calls `webview_window.navigate()` to platform URL
5. **Re-init Injection** → Re-initialize JavaScript injection for the new platform

### Implementation Skeleton

```rust
use tauri::{command, AppHandle, Manager, State};
use std::sync::Mutex;

/// Current platform state
pub struct PlatformState {
    pub current: Mutex<Option<Platform>>,
}

#[command]
pub async fn select_platform(
    app: AppHandle,
    platform: Platform,
) -> anyhow::Result<()> {
    clear_previous_session(&app).await?;
    
    if let Some(window) = app.get_webview_window("main") {
        let url = Url::parse(platform.default_url())?;
        window.navigate(url);
    }
    
    // Trigger injection re-initialization
    // (handled by injection system)
    
    Ok(())
}

async fn clear_previous_session(app: &AppHandle) -> anyhow::Result<()> {
    let state = app.state::<PlatformState>();
    let current = state.current.lock().unwrap();
    
    if let Some(platform) = *current {
        clear_platform_session(app, platform)?;
    }
    
    Ok(())
}
```

### Selector HTML Structure

```html
<!-- src-tauri/selector.html (bundled resource) -->
<!DOCTYPE html>
<html>
<head>
    <title>Select Platform</title>
    <style>
        /* Minimal inline styles for selector UI */
    </style>
</head>
<body>
    <div class="platform-grid">
        <button onclick="select('instagram')">Instagram</button>
        <button onclick="select('messenger')">Messenger</button>
        <button onclick="select('facebook')">Facebook</button>
        <button onclick="select('x')">X</button>
    </div>
    <script>
        async function select(platform) {
            await invoke('select_platform', { platform });
        }
    </script>
</body>
</html>
```

---

## CSP Per Platform

Content Security Policy strings allowing platform CDNs while blocking telemetry:

### Instagram CSP

```rust
const INSTAGRAM_CSP: &str = concat!(
    "default-src 'self' 'unsafe-inline' 'unsafe-eval'; ",
    "script-src 'self' 'unsafe-inline' 'unsafe-eval' *.instagram.com *.cdninstagram.com *.facebook.com *.fbcdn.net; ",
    "style-src 'self' 'unsafe-inline' *.instagram.com *.cdninstagram.com *.facebook.com; ",
    "img-src 'self' blob: data: *.instagram.com *.cdninstagram.com *.facebook.com *.fbcdn.net; ",
    "connect-src 'self' *.instagram.com *.facebook.com; ",
    "frame-src 'self' *.instagram.com; ",
    "font-src 'self' fonts.gstatic.com;",
);
```

### Messenger CSP

```rust
const MESSENGER_CSP: &str = concat!(
    "default-src 'self' 'unsafe-inline' 'unsafe-eval'; ",
    "script-src 'self' 'unsafe-inline' 'unsafe-eval' *.messenger.com *.facebook.com *.fbcdn.net; ",
    "style-src 'self' 'unsafe-inline' *.messenger.com *.facebook.com; ",
    "img-src 'self' blob: data: *.messenger.com *.facebook.com *.fbcdn.net; ",
    "connect-src 'self' *.messenger.com *.facebook.com; ",
    "frame-src 'self' *.messenger.com; ",
    "font-src 'self' fonts.gstatic.com;",
);
```

### Facebook CSP

```rust
const FACEBOOK_CSP: &str = concat!(
    "default-src 'self' 'unsafe-inline' 'unsafe-eval'; ",
    "script-src 'self' 'unsafe-inline' 'unsafe-eval' *.facebook.com *.fbcdn.net; ",
    "style-src 'self' 'unsafe-inline' *.facebook.com; ",
    "img-src 'self' blob: data: *.facebook.com *.fbcdn.net; ",
    "connect-src 'self' *.facebook.com; ",
    "frame-src 'self' *.facebook.com; ",
    "font-src 'self' fonts.gstatic.com;",
);
```

### X (Twitter) CSP

```rust
const X_CSP: &str = concat!(
    "default-src 'self' 'unsafe-inline' 'unsafe-eval'; ",
    "script-src 'self' 'unsafe-inline' 'unsafe-eval' *.x.com *.twimg.com; ",
    "style-src 'self' 'unsafe-inline' *.x.com; ",
    "img-src 'self' blob: data: *.x.com *.twimg.com; ",
    "connect-src 'self' *.x.com; ",
    "frame-src 'self' *.x.com; ",
    "font-src 'self' fonts.gstatic.com;",
);
```

### CSP Helper

```rust
impl Platform {
    pub fn csp(&self) -> &'static str {
        match self {
            Platform::Instagram => INSTAGRAM_CSP,
            Platform::Messenger => MESSENGER_CSP,
            Platform::Facebook => FACEBOOK_CSP,
            Platform::X => X_CSP,
        }
    }
}
```

**Note**: All CSPs exclude:
- `*.doubleclick.net`
- `*.googlesyndication.com`
- `*.google-analytics.com`

---

## Implementation Plan

### Wave 2: Rust Core

- [ ] Add `Platform` enum with serde support
- [ ] Add `url = "2"` to Cargo.toml
- [ ] Create platform URL mapping helper
- [ ] Implement `get_platform_data_dir()` and friends
- [ ] Add Tauri commands: `select_platform`
- [ ] Create `PlatformState` for current runtime state
- [ ] Add CSP getter for each platform

### Wave 3: UI Layer

- [ ] Create `selector.html` bundled resource
- [ ] Add minimal platform selection UI
- [ ] Wire platform selection to Tauri commands
- [ ] Add CSS styling for selector (no external deps)

### Wave 4: Privacy & Session

- [ ] Implement session data clearing on switch
- [ ] Implement session data clearing on app quit
- [ ] Add cookie clearing helper
- [ ] Verify data isolation between platforms
- [ ] Test telemetry blocking

### Wave 5: Integration

- [ ] Wire platform selector into startup flow
- [ ] Add platform persistence (remember last used)
- [ ] Handle platform switch edge cases
- [ ] Update tray/menu with platform indicator

### Wave 6: Testing

- [ ] Unit tests for Platform enum
- [ ] Unit tests for data directory helpers
- [ ] Integration test: platform switch flow
- [ ] Manual test: verify session isolation per platform
- [ ] Manual test: verify telemetry blocked

---

## Dependencies to Add

| Dependency | Version | Purpose |
|------------|---------|---------|
| `url`      | "2"     | URL parsing for `WebviewWindow::navigate()` |

No other Rust dependencies required.

---

## Notes

- Platform selector is a **local HTML file** (no TypeScript changes in Wave 1)
- Session isolation prevents cross-platform data contamination
- CSP blocking prevents telemetry from each platform
- `url` crate is lightweight and already used by Tauri internally
