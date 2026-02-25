// Main entry point for the Tauri app frontend.
// Initializes the app, sets up event listeners, and manages state.

import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { registerShortcuts } from './keyboard-shortcuts';
import { setupNotificationInterceptor } from './notification-interceptor';
import { setupThemeInjector } from './theme-injector';
import { setupUnreadCounter } from './unread-counter';
import { setupPrivacyGuard } from './privacy-guard';
import { loadSettings } from './settings';

// Initialize the app.
async function init() {
    try {
        // Load settings
        await loadSettings();
        
        // Set up event listeners
        setupEventListeners();
        
        // Set up modules
        registerShortcuts();
        setupNotificationInterceptor();
        setupThemeInjector();
        setupUnreadCounter();
        setupPrivacyGuard();
        
        // Log initialization
        console.log('Messenger Desktop initialized');
    } catch (error) {
        console.error('Failed to initialize app:', error);
    }
}

// Set up event listeners.
function setupEventListeners() {
    // Listen for file drops
    listen('file-drop', (event) => {
        console.log('Files dropped:', event.payload);
    });
    
    // Listen for update progress
    listen('update-progress', (event) => {
        console.log('Update progress:', event.payload);
    });
    
    // Listen for account switching
    listen('switch-account', (event) => {
        console.log('Switched to account:', event.payload);
    });
    
    // Listen for spellcheck events
    listen('enable-spellcheck', (event) => {
        console.log('Spellcheck enabled:', event.payload);
    });
    
    listen('set-spellcheck-lang', (event) => {
        console.log('Spellcheck language set:', event.payload);
    });
    
    // Listen for navigate events
    listen('navigate', (event) => {
        const hash = event.payload as string;
        window.location.hash = hash;
        console.log('Navigated to:', hash);
    });
    
    // Listen for global shortcut trigger events
    listen('global-shortcut-trigger', (event) => {
        const action = event.payload as string;
        if (action === 'new_message') {
            console.log('Global shortcut: new_message');
            // TODO: Implement new message shortcut logic
        } else if (action === 'mute') {
            console.log('Global shortcut: mute');
            // TODO: Implement mute shortcut logic
        } else if (action === 'dnd') {
            console.log('Global shortcut: dnd');
            // TODO: Implement Do Not Disturb shortcut logic
        }
    });
    
    // Handle window close
    getCurrentWindow().onCloseRequested(async (event) => {
        console.log('Window close requested');
        // Add cleanup logic here
    });
}

// Run initialization.
init();