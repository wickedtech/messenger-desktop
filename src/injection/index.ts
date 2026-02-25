const PLATFORM_PATTERNS: Record<string, RegExp> = {
  Messenger: /messenger\.com/,
  Instagram: /instagram\.com/,
  Facebook:  /facebook\.com/,
  X:         /x\.com/,
};

function detectPlatform(): string | null {
  const host = window.location.hostname;
  for (const [name, pattern] of Object.entries(PLATFORM_PATTERNS)) {
    if (pattern.test(host)) return name;
  }
  return null;
}

document.addEventListener('DOMContentLoaded', () => {
  const platform = detectPlatform();
  if (!platform) return;
  console.log(`[injection] platform: ${platform}`);
  try { initNotifications(); } catch(e) { console.error('[injection] notifications:', e); }
  try { initUnread(); } catch(e) { console.error('[injection] unread:', e); }
  try { initTheme(); } catch(e) { console.error('[injection] theme:', e); }
  if (platform === 'Messenger' || platform === 'Facebook') {
    try { initShortcuts(); } catch(e) { console.error('[injection] shortcuts:', e); }
  }
  try { initPrivacy(); } catch(e) { console.error('[injection] privacy:', e); }
});
