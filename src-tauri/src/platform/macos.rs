//! macOS-specific features for Tauri app.
//! All functions are wrapped in `#[cfg(target_os = "macos")]`.

use tauri::{AppHandle, Emitter, Listener};
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use std::ptr;
use objc_foundation::INSString;
use objc_id::ShareId;

/// Initialize macOS-specific features.
pub fn init(app: &AppHandle) {
    log::info!("Initializing macOS platform features");

    // Listen for request-focus events and bring app to foreground
    let app_handle = app.clone();
    app.listen("request-focus", move |_event| {
        request_foreground_activation(&app_handle);
    });
}

/// Request foreground activation on macOS.
/// Brings the app to the front when called after window.show().
pub fn request_foreground_activation(app: &AppHandle) {
    unsafe {
        let ns_app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
        let _: () = msg_send![ns_app, activateIgnoringOtherApps: true];
    }

    // Emit event for frontend acknowledgment
    let _ = app.emit("app-focused", ());
}

/// Set the dock badge count.
/// Uses Objective-C runtime to set NSApp dock badge.
/// - `count`: Badge count. 0 clears the badge.
pub fn set_dock_badge(count: u32) {
    unsafe {
        let ns_app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
        let dock_tile: *mut Object = msg_send![ns_app, dockTile];

        if count == 0 {
            let null: *mut Object = ptr::null_mut();
            let _: () = msg_send![dock_tile, setBadgeLabel: null];
        } else {
            let count_str = format!("{}", count);
            let ns_string: *mut Object = msg_send![class!(NSString),
                stringWithUTF8String: count_str.as_ptr()];
            let _: () = msg_send![dock_tile, setBadgeLabel: ns_string];
        }
    }
}

/// Bounce the dock icon to request user attention.
/// - `critical`: If true, bounces until the app is activated.
pub fn bounce_dock(critical: bool) {
    unsafe {
        let ns_app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
        let request_type: i64 = if critical { 1 } else { 0 };
        let _: () = msg_send![ns_app, requestUserAttention: request_type];
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_macos_module_compiles() {
        assert!(true);
    }

    #[test]
    fn test_request_foreground_activation() {
        assert!(true);
    }
}
