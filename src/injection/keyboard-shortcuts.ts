// keyboard-shortcuts.ts
// Handles keyboard shortcuts for messenger actions

function isInputActive(): boolean {
  const active = document.activeElement;
  if (!active) return false;

  const tagName = active.tagName.toLowerCase();
  if (tagName === 'input' || tagName === 'textarea') {
    return true;
  }

  return (active as HTMLElement).getAttribute('contenteditable') === 'true';
}

function handleShortcut(action: string, data?: any): void {
  try {
    // @ts-ignore
    (window as any).__TAURI__.invoke('handle_shortcut', { action, ...data });
  } catch (error) {
    console.error(`[shortcuts] Failed to handle ${action}:`, error);
  }
}

export function init(): void {
  document.addEventListener('keydown', (event) => {
    // Skip if user is typing in an input field
    if (isInputActive()) {
      return;
    }

    const isCtrl = event.ctrlKey || event.metaKey;
    const isShift = event.shiftKey;

    // Ctrl+N - New message
    if (isCtrl && event.key.toLowerCase() === 'n') {
      event.preventDefault();
      handleShortcut('new-message');
      return;
    }

    // Ctrl+Shift+M - Mute
    if (isCtrl && isShift && event.key.toLowerCase() === 'm') {
      event.preventDefault();
      handleShortcut('mute');
      return;
    }

    // Ctrl+1-9 - Switch conversation
    if (isCtrl && event.key >= '1' && event.key <= '9') {
      event.preventDefault();
      const index = parseInt(event.key, 10);
      handleShortcut('switch-conversation', { index });
      return;
    }
  });

  console.log('[shortcuts] Keyboard listener initialized');
}
