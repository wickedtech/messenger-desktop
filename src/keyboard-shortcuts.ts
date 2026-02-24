// Keyboard shortcuts manager for Tauri app.
// Registers global shortcuts and handles key events.

import { register, unregister } from '@tauri-apps/api/globalShortcut';
import { invoke } from '@tauri-apps/api/tauri';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { isRegistered } from '@tauri-apps/api/globalShortcut';

// Register keyboard shortcuts.
export async function registerShortcuts() {
    try {
        // Register global shortcuts
        await register('CommandOrControl+Shift+M', async () => {
            const window = getCurrentWindow();
            if (await window.isMinimized()) {
                await window.unminimize();
            }
            await window.setFocus();
        });
        
        await register('CommandOrControl+Shift+N', async () => {
            await invoke('handle_new_window');
        });
        
        await register('CommandOrControl+Shift+Q', async () => {
            await invoke('quit_app');
        });
        
        // Register in-app shortcuts
        document.addEventListener('keydown', handleKeyDown);
        
        console.log('Keyboard shortcuts registered');
    } catch (error) {
        console.error('Failed to register shortcuts:', error);
    }
}

// Unregister keyboard shortcuts.
export async function unregisterShortcuts() {
    try {
        await unregister('CommandOrControl+Shift+M');
        await unregister('CommandOrControl+Shift+N');
        await unregister('CommandOrControl+Shift+Q');
        
        document.removeEventListener('keydown', handleKeyDown);
        
        console.log('Keyboard shortcuts unregistered');
    } catch (error) {
        console.error('Failed to unregister shortcuts:', error);
    }
}

// Handle key down events.
function handleKeyDown(event: KeyboardEvent) {
    // Example: Ctrl+K to focus search
    if (event.ctrlKey && event.key === 'k') {
        event.preventDefault();
        const searchInput = document.querySelector('input[placeholder="Search"]') as HTMLElement;
        if (searchInput) {
            searchInput.focus();
        }
    }
    
    // Example: Ctrl+Shift+L to toggle dark mode
    if (event.ctrlKey && event.shiftKey && event.key === 'L') {
        event.preventDefault();
        invoke('toggle_dark_mode');
    }
}

// Check if a shortcut is registered.
export async function isShortcutRegistered(shortcut: string): Promise<boolean> {
    return await isRegistered(shortcut);
}