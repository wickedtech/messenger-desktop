# Architecture

## Overview

Messenger Desktop is a **Tauri 2.x** application that wraps `messenger.com` in a native WebView while extending its functionality with a Rust backend. The architecture consists of:

1. **WebView Layer**: A lightweight browser window that loads `messenger.com`.
2. **JS Injection Layer**: Custom JavaScript scripts injected into the WebView to intercept and modify Messenger's behavior.
3. **Rust Backend**: Native modules for system tray, notifications, shortcuts, themes, and privacy controls.
4. **IPC Layer**: Communication bridge between the WebView and Rust backend using Tauri's IPC system.

```
┌───────────────────────────────────────────────────────┐
│                   WebView (messenger.com)              │
│                                                       │
│  ┌─────────────┐    ┌─────────────┐    ┌───────────┐  │
│  │  JS Script  │    │  JS Script  │    │  JS Script│  │
│  │ (Privacy)   │    │ (Themes)    │    │ (Shortcuts)│  │
│  └─────────────┘    └─────────────┘    └───────────┘  │
│                                                       │
└───────────────────────┬───────────────────────────────┘
                        │
                        ▼
┌───────────────────────────────────────────────────────┐
│                     Rust Backend                       │
│                                                       │
│  ┌─────────────┐    ┌─────────────┐    ┌───────────┐  │
│  │  Tray       │    │  Notifications│  │  Updater  │  │
│  │  Module     │    │  Module      │  │  Module   │  │
│  └─────────────┘    └─────────────┘    └───────────┘  │
│  ┌─────────────┐    ┌─────────────┐                  │
│  │  Shortcuts  │    │  Privacy    │                  │
│  │  Module     │    │  Module     │                  │
│  └─────────────┘    └─────────────┘                  │
│                                                       │
└───────────────────────────────────────────────────────┘
```

## JS Injection Layer

The JS Injection Layer consists of scripts injected into the WebView to:

- **Intercept Network Requests**: Block typing indicators, read receipts, and other privacy-invasive features.
- **Modify DOM**: Apply custom themes, hide elements, and inject UI controls.
- **Forward Events**: Send events (e.g., new messages, unread counts) to the Rust backend via Tauri IPC.

### Example: Privacy Script

```javascript
// Block typing indicators
const originalSend = XMLHttpRequest.prototype.send;
XMLHttpRequest.prototype.send = function(body) {
  if (body.includes('typing')) {
    return;
  }
  originalSend.call(this, body);
};
```

## Rust Backend

The Rust backend is organized into modules:

| Module          | Responsibility                                                                 |
|-----------------|--------------------------------------------------------------------------------|
| **Tray**        | System tray icon, unread badge, and context menu.                              |
| **Notifications**| Native system notifications for new messages.                                  |
| **Shortcuts**   | Global keyboard shortcuts (e.g., `Ctrl+Shift+M`).                              |
| **Themes**      | Custom themes and dark mode support.                                            |
| **Privacy**     | Block typing indicators, read receipts, and other tracking features.            |
| **Accounts**    | Multi-account switching and session management.                                |
| **Updater**     | Auto-updater for new releases.                                                  |
| **Platform**    | Platform-specific code (Linux, Windows, macOS).                                |

## IPC Flow

Communication between the WebView and Rust backend uses Tauri's IPC system:

1. **WebView → Rust**:
   - JavaScript calls `invoke('command_name', payload)` to send data to Rust.
   - Example: Sending a new message event to update the unread badge.

2. **Rust → WebView**:
   - Rust emits events using `emit('event_name', payload)`.
   - Example: Emitting a theme change event to update the WebView.

```
WebView (JS)                     Rust Backend
     │                                  │
     │── invoke('new_message', {}) ───▶│
     │                                  │
     │◀── emit('theme_changed', {}) ────│
```

## Data Flow: Notification Interception

```
┌───────────────────────────────────────────────────────┐
│                   WebView (messenger.com)              │
│                                                       │
│  1. New message received                              │
│     │                                                  │
│     ▼                                                  │
│  2. JS script intercepts message event                 │
│     │                                                  │
│     ▼                                                  │
│  3. invoke('new_message', { text, sender })            │
│     │                                                  │
└───────────┬───────────────────────────────────────────┘
            │
            ▼
┌───────────────────────────────────────────────────────┐
│                     Rust Backend                       │
│                                                       │
│  4. Tauri command 'new_message' receives payload       │
│     │                                                  │
│     ▼                                                  │
│  5. Notification module creates native notification     │
│     │                                                  │
│     ▼                                                  │
│  6. Tray module updates unread badge                    │
│                                                       │
└───────────────────────────────────────────────────────┘
```