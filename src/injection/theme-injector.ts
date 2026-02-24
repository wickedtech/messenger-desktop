// theme-injector.ts
// Manages CSS themes for the messenger interface

const themes: Record<string, string> = {
  light: '',
  dark: 'body{background:#1a1a2e!important;color:#e0e0e0!important}',
  darker: 'body{background:#0d0d1a!important;color:#e0e0e0!important}',
  oled: 'body{background:#000!important;color:#e0e0e0!important}'
};

let currentTheme: string = 'light';
let styleElement: HTMLStyleElement | null = null;

function createStyleElement(): void {
  if (!styleElement) {
    styleElement = document.createElement('style');
    styleElement.id = 'messenger-theme';
    document.head.appendChild(styleElement);
  }
}

function applyTheme(themeName: string): void {
  if (!themes[themeName]) {
    console.error(`[theme] Unknown theme: ${themeName}`);
    return;
  }

  currentTheme = themeName;
  createStyleElement();

  if (styleElement) {
    styleElement.textContent = themes[themeName];
    console.log(`[theme] Applied: ${themeName}`);
  }
}

function removeTheme(): void {
  if (styleElement) {
    styleElement.remove();
    styleElement = null;
    currentTheme = 'light';
    console.log('[theme] Removed theme');
  }
}

export function init(): void {
  // Apply initial light theme
  applyTheme('light');

  // Listen for theme changes from Tauri
  try {
    // @ts-ignore
    (window as any).__TAURI__.event.listen('set-theme', (e: any) => {
      if (e.payload === 'remove' || e.payload === 'default') {
        removeTheme();
      } else {
        applyTheme(e.payload);
      }
    });
    console.log('[theme] Event listener initialized');
  } catch (error) {
    console.error('[theme] Failed to initialize event listener:', error);
  }
}
