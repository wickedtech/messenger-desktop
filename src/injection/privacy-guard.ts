// privacy-guard.ts
// Privacy protections for messenger interface

interface PrivacyConfig {
  blockTyping: boolean;
  blockReadReceipts: boolean;
  hideLastActive: boolean;
  blockLinkPreviews: boolean;
}

const defaultConfig: PrivacyConfig = {
  blockTyping: true,
  blockReadReceipts: true,
  hideLastActive: true,
  blockLinkPreviews: true
};

let privacyConfig: PrivacyConfig = { ...defaultConfig };
let styleElements: HTMLStyleElement[] = [];

function createStyleElement(css: string): HTMLStyleElement {
  const style = document.createElement('style');
  style.textContent = css;
  document.head.appendChild(style);
  styleElements.push(style);
  return style;
}

function applyPrivacyRules(): void {
  // Clear existing styles
  styleElements.forEach(el => el.remove());
  styleElements = [];

  // Block typing/composing URLs in XHR
  if (privacyConfig.blockTyping) {
    const originalOpen = XMLHttpRequest.prototype.open;
    XMLHttpRequest.prototype.open = function(method: string, url: string) {
      if (typeof url === 'string' && (url.includes('typing') || url.includes('composing'))) {
        console.log('[privacy] Blocked typing/composing request:', url);
        return;
      }
      return originalOpen.apply(this, arguments as any);
    };

    // Block typing/composing URLs in fetch
    const originalFetch = window.fetch;
    window.fetch = async function(input: RequestInfo | URL, init?: RequestInit): Promise<Response> {
      const url = typeof input === 'string' ? input : input instanceof Request ? input.url : '';
      if (url && (url.includes('typing') || url.includes('composing'))) {
        console.log('[privacy] Blocked typing/composing fetch:', url);
        throw new Error('Blocked by privacy guard');
      }
      return originalFetch.apply(this, [input, init] as any);
    };

    console.log('[privacy] Typing/composing blocking enabled');
  }

  // Hide read receipts
  if (privacyConfig.blockReadReceipts) {
    createStyleElement('[aria-label*="Seen"]{display:none!important}');
    console.log('[privacy] Read receipts hidden');
  }

  // Hide last active status
  if (privacyConfig.hideLastActive) {
    createStyleElement('[aria-label*="last active"],[data-last-active="true"]{display:none!important}');
    console.log('[privacy] Last active status hidden');
  }

  // Block link previews
  if (privacyConfig.blockLinkPreviews) {
    createStyleElement('[data-preview="true"],.link-preview{display:none!important}');
    console.log('[privacy] Link previews hidden');
  }
}

function updateConfig(newConfig: Partial<PrivacyConfig>): void {
  privacyConfig = { ...privacyConfig, ...newConfig };
  applyPrivacyRules();
}

export function init(): void {
  applyPrivacyRules();

  // Listen for config updates from Tauri
  try {
    // @ts-ignore
    (window as any).__TAURI__.event.listen('update-privacy', (e: any) => {
      updateConfig(e.payload);
    });
    console.log('[privacy] Event listener initialized');
  } catch (error) {
    console.error('[privacy] Failed to initialize event listener:', error);
  }
}
