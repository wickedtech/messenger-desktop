# Security Audit - Messenger Desktop

**Audit Date**: 2026-02-25  
**Auditor**: Willy (Swarm Orchestrator)  
**Project**: messenger-desktop (`src-tauri`)  
**Scope**: Rust source files in `src-tauri/src/` + `tauri.conf.json`

---

## Executive Summary

| Category | Risk Level | Issues Found |
|----------|-------------|--------------|
| Input Validation | 🟡 MEDIUM | 3 |
| Path Traversal | 🟡 MEDIUM | 2 |
| XSS | 🟢 LOW | 0 |
| Command Injection | 🟢 LOW | 0 |
| CSP Policy | 🟡 MEDIUM | 1 |
| Capability Scoping | 🔴 HIGH | 1 |

**Overall Risk**: 🟡 MEDIUM  
**Recommended Actions**: 7

---

## Findings

### 1. Input Validation

#### 1.1 Account ID Not Validated (MEDIUM)
**File**: `src/accounts.rs`  
**Line**: `remove_account(&id)`, `switch_account(&id)`, `set_profile_picture(&id, &path)`

**Issue**: Account IDs passed to Tauri commands are not validated. The `id` parameter is used directly in `self.accounts.iter().find(|a| a.id == id)` without format/sanity checks.

**Impact**: Could cause panic on invalid UUID format or unexpected behavior.

**Recommendation**:
```rust
// Validate UUID format before use
fn validate_account_id(id: &str) -> Result<(), String> {
    Uuid::parse_str(id).map_err(|_| "Invalid account ID format".to_string())
}
```

---

#### 1.2 Media Permission Type Not Validated (MEDIUM)
**File**: `src/media.rs`  
**Line**: `grant_media_permission`

**Issue**: The `permission_type` string accepts any value. Only "camera" and "microphone" are handled, but no validation prevents unknown types from being processed.

**Code**:
```rust
match permission_type.as_str() {
    "camera" => Ok(state.lock().await.request_camera()),
    "microphone" => Ok(state.lock().await.request_microphone()),
    _ => { Ok(false) }  // Silent failure
}
```

**Recommendation**: Explicitly reject unknown permission types:
```rust
match permission_type.as_str() {
    "camera" | "microphone" => /* handle */,
    _ => Err("Invalid permission type".to_string())
}
```

---

#### 1.3 Theme Name Not Validated (LOW)
**File**: `src/theme_manager.rs`  
**Line**: `set_theme(&name)`

**Issue**: Theme names are matched against a allowlist but silently default to "light" for unknown values rather than returning an error.

**Recommendation**: Return error for unknown themes instead of silent fallback.

---

### 2. Path Traversal

#### 2.1 Profile Picture Path Not Validated (MEDIUM)
**File**: `src/accounts.rs`  
**Line**: `set_profile_picture(&id, &path)`

**Issue**: The `path` parameter is passed directly to `ImageReader::open(path)` without validation. An attacker could potentially read files outside the intended directory.

**Code**:
```rust
pub fn set_profile_picture(&mut self, id: &str, path: &str) -> Result<()> {
    let img = ImageReader::open(path)  // NO VALIDATION
        .context("Failed to open image")?
```

**Recommendation**: Validate path is within allowed directories:
```rust
fn validate_image_path(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    // Check path doesn't escape allowed directories
    if path.has_root() || path.components().any(|c| c == std::path::Component::ParentDir) {
        return Err("Path traversal detected".to_string());
    }
    Ok(path)
}
```

---

#### 2.2 Drag-Drop File Path Injection (MEDIUM)
**File**: `src/drag_drop.rs`  
**Line**: `handle_file_drop`

**Issue**: File paths from drag-drop events are processed and injected into JavaScript without sanitization. While paths come from the OS (trusted), the injected JavaScript uses `file.path` directly.

**Code**:
```rust
Object.defineProperty(file, 'path', { value: '{}' }});  // Unsanitized
```

**Recommendation**: Escape special characters in filename/path before injecting into JavaScript.

---

### 3. XSS (Cross-Site Scripting)

**Status**: 🟢 NO ISSUES FOUND

The app loads `https://www.messenger.com` in a sandboxed WebView. No user-controlled content is rendered back to the WebView without sanitization.

---

### 4. Command Injection

**Status**: 🟢 NO ISSUES FOUND

No shell commands are executed with user input. The updater uses Tauri's built-in plugin, and file operations use Rust's safe filesystem APIs.

---

### 5. CSP Policy

#### 5.1 CSP Too Permissive (MEDIUM)
**File**: `tauri.conf.json`

**Current CSP**:
```
default-src https://www.messenger.com https://*.messenger.com https://*.facebook.com https://*.fbcdn.net;
```

**Issues**:
1. Wildcard `*.facebook.com` and `*.fbcdn.net` is overly broad - allows any Facebook subdomain
2. No `script-src` directive - defaults to `default-src`
3. No `connect-src` - WebSocket connections to arbitrary

**Recommendation**:
```
default-src ' domains may be allowedself';
script-src 'self';
connect-src https://www.messenger.com https://*.messenger.com https://*.facebook.com wss://*.messenger.com;
img-src https://www.messenger.com https://*.messenger.com https://*.facebook.com https://*.fbcdn.net data:;
frame-src https://www.messenger.com;
```

---

### 6. Tauri Capability Scoping

#### 6.1 No Capabilities Directory (HIGH)
**File**: `src-tauri/capabilities/`

**Issue**: No Tauri 2 capabilities file exists. This means the app uses default permissions which may be too permissive.

**Current Plugins** (from `lib.rs`):
- `tauri_plugin_shell` - Can execute shell commands
- `tauri_plugin_notification` - Can show notifications
- `tauri_plugin_store` - Can read/write persistent storage
- `tauri_plugin_updater` - Can download/execute updates
- `tauri_plugin_global_shortcut` - Can register global shortcuts
- `tauri_plugin_clipboard_manager` - Full clipboard access
- `tauri_plugin_autostart` - Can configure auto-start

**Risk**: If any of these plugins have vulnerabilities, the impact is maximized because no capability restrictions are defined.

**Recommendation**: Create `src-tauri/capabilities/default.json`:
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities for Messenger Desktop",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "shell:allow-open",
    "notification:default",
    "store:default",
    "global-shortcut:allow-register",
    "clipboard-manager:allow-read-text",
    "clipboard-manager:allow-write-text"
  ]
}
```

---

## Additional Findings

### 7. Privacy Concerns

#### 7.1 Session Tokens Stored in Plain Text (MEDIUM)
**File**: `src/accounts.rs`

**Issue**: Session tokens are stored in the store file without encryption.

```rust
pub struct Account {
    pub session_token: Option<String>,  // Stored in plain text
}
```

**Recommendation**: Use Tauri's secure storage or encrypt sensitive fields.

---

#### 7.2 Custom CSS Injection (LOW)
**File**: `src/theme_manager.rs`

**Issue**: `set_custom_css` allows injecting arbitrary CSS, which could be used for clickjacking overlays if the web content is compromised.

**Status**: Acceptable - CSS is injected into the app's own WebView, not a remote page.

---

## Remediation Priority

| Priority | Issue | Effort |
|----------|-------|--------|
| P1 | Create capabilities directory | Low |
| P1 | Fix CSP policy | Low |
| P2 | Validate account ID format | Medium |
| P2 | Validate profile picture path | Medium |
| P3 | Validate media permission types | Low |
| P3 | Encrypt session tokens | High |

---

## Audit Complete

**Auditor**: Willy 🤖  
**Date**: 2026-02-25  
**Next Steps**: Apply P1 fixes, then re-audit after changes.
