import { defineConfig, presetWind } from 'unocss';

export default defineConfig({
  presets: [presetWind()],
  shortcuts: {
    'btn-primary':
      'bg-gradient-to-r from-primary to-[#8b5cf6] text-white w-full mt-5 shadow-[0_2px_8px_rgba(99,102,241,0.25)] rounded-xl py-2.75 px-7 text-sm font-semibold inline-flex items-center justify-center gap-2 border-none cursor-pointer transition-all duration-200 font-inherit hover:translate-y--1px hover:shadow-[0_4px_16px_rgba(99,102,241,0.35)] active:translate-y-0 disabled:opacity-35 disabled:cursor-not-allowed disabled:shadow-none',
    'btn-secondary':
      'bg-[var(--color-surface)] text-[var(--color-text)] border border-[var(--color-border)] rounded-xl py-2.75 px-7 text-sm font-semibold inline-flex items-center justify-center gap-2 cursor-pointer transition-all duration-200 font-inherit hover:bg-[var(--color-border)]',
    'btn-outline':
      'bg-transparent text-primary border-1.5 border-primary rounded-xl py-2.75 px-7 text-sm font-semibold inline-flex items-center justify-center gap-2 cursor-pointer transition-all duration-200 font-inherit hover:bg-primaryLight',
    'chip-placeholder': 'text-sm text-[var(--color-text-muted)] py-2',
  },
});
