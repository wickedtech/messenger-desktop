// index.ts
// Main injection entry point - initializes all injection modules

import { init as initNotifications } from './notification-interceptor';
import { init as initUnread } from './unread-counter';
import { init as initTheme } from './theme-injector';
import { init as initShortcuts } from './keyboard-shortcuts';
import { init as initPrivacy } from './privacy-guard';

document.addEventListener('DOMContentLoaded', () => {
  try { initNotifications(); } catch(e) { console.error('[injection] notifications:', e); }
  try { initUnread(); } catch(e) { console.error('[injection] unread:', e); }
  try { initTheme(); } catch(e) { console.error('[injection] theme:', e); }
  try { initShortcuts(); } catch(e) { console.error('[injection] shortcuts:', e); }
  try { initPrivacy(); } catch(e) { console.error('[injection] privacy:', e); }
});
