// Notification interceptor for Tauri app.
// Intercepts and manages browser notifications.

import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

// Setup notification interceptor.
export function setupNotificationInterceptor() {
    // Override Notification API
    const OriginalNotification = window.Notification;
    
    window.Notification = class Notification extends OriginalNotification {
        constructor(title: string, options?: NotificationOptions) {
            super(title, options);
            
            // Intercept notification
            handleNotification(title, options);
        }
    } as any;
    
    // Listen for notification events from Rust backend
    listen('show-notification', (event) => {
        const { title, body, icon } = event.payload as { title: string; body: string; icon?: string };
        showNotification(title, body, icon);
    });
    
    console.log('Notification interceptor set up');
}

// Handle a notification.
async function handleNotification(title: string, options?: NotificationOptions) {
    const permissionGranted = await isPermissionGranted();
    
    if (!permissionGranted) {
        const permission = await requestPermission();
        if (permission !== 'granted') {
            console.warn('Notification permission not granted');
            return;
        }
    }
    
    // Send notification to Tauri backend
    invoke('handle_notification', { title, options });
    
    // Show system notification
    showNotification(title, options?.body, options?.icon);
}

// Show a system notification.
async function showNotification(title: string, body?: string, icon?: string) {
    sendNotification({ title, body, icon });
}

// Request notification permission.
export async function requestNotificationPermission(): Promise<boolean> {
    const permission = await requestPermission();
    return permission === 'granted';
}

// Check if notification permission is granted.
export async function isNotificationPermissionGranted(): Promise<boolean> {
    return await isPermissionGranted();
}