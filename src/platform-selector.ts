import { invoke } from '@tauri-apps/api/core';

export async function selectPlatform(name: string): Promise<void> {
  try {
    await invoke('select_platform', { platformName: name });
    document.getElementById('selector')?.remove();
  } catch (err) {
    const el = document.getElementById('error-msg');
    if (el) {
      el.textContent = `Failed to load ${name}: ${err}`;
      el.style.display = 'block';
    }
  }
}

export async function showLastPlatform(): Promise<void> {
  try {
    const last: string | null = await invoke('get_last_platform');
    if (last) {
      const el = document.getElementById('last-platform');
      if (el) el.textContent = `Last used: ${last}`;
    }
  } catch (_) {}
}

document.addEventListener('DOMContentLoaded', () => {
  showLastPlatform();
  document.querySelectorAll('[data-platform]').forEach(card => {
    card.addEventListener('click', () => {
      selectPlatform((card as HTMLElement).dataset.platform!);
    });
  });
});