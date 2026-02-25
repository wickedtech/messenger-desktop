// notification-interceptor.ts
// Intercepts window.Notification to redirect notifications to Tauri
// Notification interceptor for Tauri v2 webview injection
// Intercepts window.Notification and routes to Tauri handle_notification command

(function() {
    if ((window as any).__MESSENGER_DESKTOP_PATCHED__) { return; }
    (window as any).__MESSENGER_DESKTOP_PATCHED__ = true;

    const OriginalNotification = (window as any).Notification;
    const tauriCore = (window as any).__TAURI__?.core;

    (window as any).Notification = function(title: string, options: any = {}) {
        if (tauriCore?.invoke) {
            tauriCore.invoke('handle_notification', {
                title: String(title),
                options: {
                    body: options?.body || '',
                    icon: options?.icon || null,
                    tag: options?.tag || null,
                    silent: options?.silent || false,
                }
            }).catch((e: any) => console.warn('[notification] Tauri invoke failed:', e));
        } else if (OriginalNotification) {
            return new OriginalNotification(title, options);
        }
    };

    (window as any).Notification.permission = 'granted';
    (window as any).Notification.requestPermission = () => Promise.resolve('granted' as NotificationPermission);
})();
