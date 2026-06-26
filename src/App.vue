<script setup lang="ts">
import { useI18n } from './i18n';
import ConversionPanel from './components/ConversionPanel.vue';

const { locale, t } = useI18n();
</script>

<template>
  <div class="app">
    <header class="app-header">
      <div class="app-logo">&#128259;</div>
      <div class="app-title-group">
        <span class="app-title">Vert</span>
        <span class="app-subtitle">{{ t('app.subtitle') }}</span>
      </div>
      <button
        class="lang-toggle"
        :title="t('app.lang')"
        @click="locale = locale === 'en' ? 'zh' : 'en'"
      >
        {{ locale === 'en' ? '中' : 'A' }}
      </button>
    </header>
    <main class="app-main">
      <ConversionPanel />
    </main>
  </div>
</template>
