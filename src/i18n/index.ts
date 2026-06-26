import { ref, watch } from 'vue';
import { messages } from './messages';

export type Locale = 'en' | 'zh';

const locale = ref<Locale>(loadLocale());

function loadLocale(): Locale {
  try {
    const stored = localStorage.getItem('vert-locale');
    if (stored === 'en' || stored === 'zh') return stored;
  } catch { /* localStorage unavailable */ }
  return 'en';
}

export function useI18n() {
  watch(locale, (val) => {
    try { localStorage.setItem('vert-locale', val); } catch { /* */ }
  });

  function t(key: string, params?: Record<string, string>): string {
    const msg = messages[locale.value]?.[key]
      ?? messages.en?.[key]
      ?? key;

    if (!params) return msg;

    return msg.replace(/\{(\w+)\}/g, (_, k) => params[k] ?? `{${k}}`);
  }

  return { locale, t };
}
