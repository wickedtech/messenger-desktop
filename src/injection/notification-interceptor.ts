// notification-interceptor.ts
// Intercepts window.Notification to redirect notifications to Tauri

class NotificationInterceptor {
  private originalNotification: any;
  private enabled: boolean = true;

  constructor() {
    this.originalNotification = window.Notification;
  }

  public intercept(): void {
    // @ts-ignore
    window.Notification = (title: string, options?: NotificationOptions) => {
      if (!this.enabled) {
        return null;
      }

      try {
        // @ts-ignore
        (window as any).__TAURI__.invoke('show_notification', {
          title,
          body: options?.body || '',
          icon: options?.icon || '',
          tag: options?.tag || ''
        });
      } catch (error) {
        console.error('[notification] Failed to show notification:', error);
      }

      return null;
    };

    // @ts-ignore
    window.Notification.requestPermission = this.originalNotification.requestPermission.bind(this.originalNotification);
    // @ts-ignore
    window.Notification.permission = this.originalNotification.permission;

    // Listen for toggle events from Tauri
    // @ts-ignore
    (window as any).__TAURI__.event.listen('toggle-notifications', (e: any) => {
      this.enabled = e.payload;
      console.log('[notification] Notifications', this.enabled ? 'enabled' : 'disabled');
    });
  }
}

// Global instance
let interceptor: NotificationInterceptor | null = null;

export function init(): void {
  if (!interceptor) {
    try {
      interceptor = new NotificationInterceptor();
      interceptor.intercept();
      console.log('[notification] Interceptor initialized');
    } catch (error) {
      console.error('[notification] Failed to initialize interceptor:', error);
    }
  } else {
    console.log('[notification] Interceptor already initialized');
  }
}
