type Settings = {
    "auto-start": boolean;
    "minimize-to-tray": boolean;
    language: string;
    "notifications-enabled": boolean;
    "dnd-enabled": boolean;
    "dnd-from": string;
    "dnd-to": string;
    theme: string;
    "text-size": number;
    "custom-css": string;
    "block-typing": boolean;
    "block-receipts": boolean;
    "hide-active": boolean;
    "block-previews": boolean;
    shortcuts: Record<string, string>;
};

// Initialize settings on load
window.addEventListener('DOMContentLoaded', async () => {
    // Load all settings from backend
    const settings: Settings = await window.__TAURI__.invoke('get_all_settings');
    
    // Populate form fields
    (document.getElementById('auto-start') as HTMLInputElement).checked = settings["auto-start"];
    (document.getElementById('minimize-to-tray') as HTMLInputElement).checked = settings["minimize-to-tray"];
    (document.getElementById('language') as HTMLSelectElement).value = settings.language;
    (document.getElementById('notifications-enabled') as HTMLInputElement).checked = settings["notifications-enabled"];
    (document.getElementById('dnd-enabled') as HTMLInputElement).checked = settings["dnd-enabled"];
    (document.getElementById('dnd-from') as HTMLInputElement).value = settings["dnd-from"];
    (document.getElementById('dnd-to') as HTMLInputElement).value = settings["dnd-to"];
    
    // Set theme radio
    const themeRadios = document.getElementsByName('theme') as NodeListOf<HTMLInputElement>;
    themeRadios.forEach(radio => {
        radio.checked = radio.value === settings.theme;
    });
    
    // Set text size slider
    (document.getElementById('text-size') as HTMLInputElement).value = settings["text-size"].toString();
    document.getElementById('text-size-value')!.textContent = `${settings["text-size"]}%`;
    
    // Set custom CSS if theme is custom
    if (settings.theme === 'custom') {
        document.getElementById('custom-css-row')!.classList.add('active');
        (document.getElementById('custom-css') as HTMLTextAreaElement).value = settings["custom-css"];
    }
    
    // Set privacy toggles
    (document.getElementById('block-typing') as HTMLInputElement).checked = settings["block-typing"];
    (document.getElementById('block-receipts') as HTMLInputElement).checked = settings["block-receipts"];
    (document.getElementById('hide-active') as HTMLInputElement).checked = settings["hide-active"];
    (document.getElementById('block-previews') as HTMLInputElement).checked = settings["block-previews"];
    
    // Set shortcuts
    const shortcuts = settings.shortcuts;
    document.getElementById('shortcut-toggle-window')!.textContent = shortcuts["toggle-window"];
    document.getElementById('shortcut-new-message')!.textContent = shortcuts["new-message"];
    document.getElementById('shortcut-mute')!.textContent = shortcuts.mute;
    document.getElementById('shortcut-dnd')!.textContent = shortcuts.dnd;
    document.getElementById('shortcut-fullscreen')!.textContent = shortcuts.fullscreen;
    
    // Set version
    const version = await window.__TAURI__.invoke('get_current_version');
    document.getElementById('version')!.textContent = version;
    
    // Setup sidebar navigation
    const sidebarItems = document.querySelectorAll('.sidebar-item');
    sidebarItems.forEach(item => {
        item.addEventListener('click', () => {
            sidebarItems.forEach(i => i.classList.remove('active'));
            item.classList.add('active');
            
            const sectionId = item.getAttribute('data-section');
            document.querySelectorAll('.section').forEach(section => {
                section.classList.remove('active');
            });
            document.getElementById(sectionId!)!.classList.add('active');
        });
    });
    
    // Auto-save on change for toggles
    document.querySelectorAll('input[type="checkbox"]').forEach(toggle => {
        toggle.addEventListener('change', async () => {
            const key = toggle.id;
            const value = (toggle as HTMLInputElement).checked;
            await window.__TAURI__.invoke('save_setting', { key, value });
        });
    });
    
    // Auto-save for language select
    document.getElementById('language')!.addEventListener('change', async (e) => {
        const key = 'language';
        const value = (e.target as HTMLSelectElement).value;
        await window.__TAURI__.invoke('save_setting', { key, value });
    });
    
    // Auto-save for DND time inputs
    document.querySelectorAll('#dnd-from, #dnd-to').forEach(input => {
        input.addEventListener('change', async (e) => {
            const key = (e.target as HTMLInputElement).id;
            const value = (e.target as HTMLInputElement).value;
            await window.__TAURI__.invoke('save_setting', { key, value });
        });
    });
    
    // Theme radio change handler
    document.querySelectorAll('input[name="theme"]').forEach(radio => {
        radio.addEventListener('change', async (e) => {
            const theme = (e.target as HTMLInputElement).value;
            await window.__TAURI__.invoke('set_theme', { theme_name: theme });
            
            if (theme === 'custom') {
                document.getElementById('custom-css-row')!.classList.add('active');
            } else {
                document.getElementById('custom-css-row')!.classList.remove('active');
            }
        });
    });
    
    // Text size slider handler
    document.getElementById('text-size')!.addEventListener('input', (e) => {
        const value = (e.target as HTMLInputElement).value;
        document.getElementById('text-size-value')!.textContent = `${value}%`;
    });
    
    document.getElementById('text-size')!.addEventListener('change', async (e) => {
        const value = parseInt((e.target as HTMLInputElement).value);
        await window.__TAURI__.invoke('set_zoom', { level: value / 100 });
    });
    
    // Custom CSS handler
    document.getElementById('custom-css')!.addEventListener('change', async (e) => {
        const value = (e.target as HTMLTextAreaElement).value;
        await window.__TAURI__.invoke('save_setting', { key: 'custom-css', value });
    });
    
    // Privacy toggles handler
    document.querySelectorAll('#block-typing, #block-receipts, #hide-active, #block-previews').forEach(toggle => {
        toggle.addEventListener('change', async () => {
            const block_typing = (document.getElementById('block-typing') as HTMLInputElement).checked;
            const block_read_receipts = (document.getElementById('block-receipts') as HTMLInputElement).checked;
            const hide_last_active = (document.getElementById('hide-active') as HTMLInputElement).checked;
            const block_link_previews = (document.getElementById('block-previews') as HTMLInputElement).checked;
            
            await window.__TAURI__.invoke('set_privacy', {
                block_typing,
                block_read_receipts,
                hide_last_active,
                block_link_previews
            });
        });
    });
    
    // Shortcut edit buttons
    document.querySelectorAll('.edit-button').forEach(button => {
        button.addEventListener('click', async (e) => {
            const action = (e.target as HTMLButtonElement).getAttribute('data-action');
            const newKeybinding = prompt(`Enter new keybinding for ${action}:`, '');
            
            if (newKeybinding) {
                await window.__TAURI__.invoke('update_shortcut', { action, keys: newKeybinding });
                // Update UI
                document.getElementById(`shortcut-${action}`)!.textContent = newKeybinding;
            }
        });
    });
    
    // Clear cache button
    document.getElementById('clear-cache')!.addEventListener('click', async () => {
        const confirmClear = confirm('Are you sure you want to clear cookies and cache? This cannot be undone.');
        if (confirmClear) {
            await window.__TAURI__.invoke('clear_webview_data', { data_type: 'all' });
            alert('Cookies and cache cleared successfully.');
        }
    });
    
    // Reset settings button
    document.getElementById('reset-settings')!.addEventListener('click', async () => {
        const confirmReset = confirm('Are you sure you want to reset all settings to default? This cannot be undone.');
        if (confirmReset) {
            await window.__TAURI__.invoke('reset_settings');
            alert('Settings reset successfully. The app will reload.');
            window.location.reload();
        }
    });
    
    // Choose notification sound button
    document.getElementById('choose-sound')!.addEventListener('click', async () => {
        await window.__TAURI__.invoke('choose_notification_sound');
    });
});