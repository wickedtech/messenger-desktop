// unread-counter.ts
// Monitors document.title for unread message counts

let lastUnreadCount: number = -1;

function parseUnreadCount(title: string): number | null {
  // Pattern 1: "(12) Messenger" or similar
  const parenPattern = /\((\d+)\)/;
  const parenMatch = title.match(parenPattern);
  if (parenMatch) {
    return parseInt(parenMatch[1], 10);
  }

  // Pattern 2: "Messenger · 12" or similar
  const dotPattern = /·\s*(\d+)/;
  const dotMatch = title.match(dotPattern);
  if (dotMatch) {
    return parseInt(dotMatch[1], 10);
  }

  return null;
}

function updateUnreadCount(count: number): void {
  if (count === lastUnreadCount) {
    return;
  }

  lastUnreadCount = count;

  try {
    // @ts-ignore
    (window as any).__TAURI__.invoke('update_unread_count', { count });
    console.log(`[unread] Unread count updated: ${count}`);
  } catch (error) {
    console.error('[unread] Failed to update count:', error);
  }
}

export function init(): void {
  // Use MutationObserver to watch for title changes
  const titleObserver = new MutationObserver((mutations) => {
    for (const mutation of mutations) {
      if (mutation.type === 'childList' && mutation.target instanceof HTMLTitleElement) {
        const count = parseUnreadCount(mutation.target.textContent || '');
        if (count !== null) {
          updateUnreadCount(count);
        }
      }
    }
  });

  const titleElement = document.querySelector('title');
  if (titleElement) {
    titleObserver.observe(titleElement, { childList: true, subtree: true });
    console.log('[unread] MutationObserver initialized');
  }

  // Poll as fallback (every 2 seconds)
  setInterval(() => {
    const title = document.title;
    const count = parseUnreadCount(title);
    if (count !== null) {
      updateUnreadCount(count);
    }
  }, 2000);

  // Initial check
  const initialCount = parseUnreadCount(document.title);
  if (initialCount !== null) {
    updateUnreadCount(initialCount);
  }
}
