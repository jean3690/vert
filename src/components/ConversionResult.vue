<template>
  <!-- Converting -->
  <div v-if="status === 'converting'" class="status-panel converting">
    <div class="spinner"></div>
    <div class="status-title">{{ t('result.converting') }}</div>
    <div class="status-desc">{{ t('result.processing') }}</div>
  </div>

  <!-- Success -->
  <div v-else-if="status === 'success' && result" class="status-panel success">
    <div class="status-icon">&#9989;</div>
    <div class="status-title">{{ t('result.done') }}</div>
    <div class="status-desc">{{ t('result.converted', { duration }) }}</div>
    <div class="status-detail-row">
      <div style="font-weight:600;margin-bottom:4px;color:var(--color-text)">
        {{ extractFileName(result.outputPath) }}
      </div>
      <div style="margin-bottom:2px">{{ formatFileSize(result.outputSize) }}</div>
      <div style="font-size:11px;color:var(--color-text-muted);word-break:break-all">
        {{ result.outputPath }}
      </div>
    </div>
    <div class="status-actions">
      <button class="btn btn-primary" style="width:auto;margin-top:0" @click="handleOpenFile(result.outputPath)">
        {{ t('result.open') }}
      </button>
      <button class="btn btn-secondary" style="width:auto" @click="handleRevealInFolder(result.outputPath)">
        {{ t('result.showInFolder') }}
      </button>
      <button class="btn btn-secondary" style="width:auto" @click="handleCopyPath(result.outputPath)">
        {{ copied ? t('result.copied') : t('result.copyPath') }}
      </button>
      <button class="btn btn-outline" style="width:auto" @click="emit('close')">
        {{ t('result.another') }}
      </button>
    </div>
  </div>

  <!-- Error -->
  <div v-else-if="status === 'error'" class="status-panel error">
    <div class="status-icon">&#9888;&#65039;</div>
    <div class="status-title">{{ t('result.failed') }}</div>
    <div class="status-error-msg">{{ errorMessage }}</div>
    <button class="btn btn-outline" @click="emit('close')">{{ t('result.tryAgain') }}</button>
  </div>

  <!-- Idle -->
  <div v-else class="status-panel idle-state">
    <div class="idle-illustration">
      <div class="big-icon">&#128259;</div>
      <div class="status-desc" style="margin:0">{{ t('result.idle') }}</div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { openPath } from '@tauri-apps/plugin-opener';
import { useI18n } from '../i18n';
import { formatFileSize } from '../utils/formats';
import type { ConvertResult, ConversionStatus } from '../types';

const { t } = useI18n();

defineProps<{
  result: ConvertResult | null;
  status: ConversionStatus;
  errorMessage: string;
  duration: string;
}>();

const emit = defineEmits<{ close: [] }>();

const copied = ref(false);

async function handleOpenFile(path: string) {
  try { await openPath(path); } catch { /* fallback */ }
}

async function handleRevealInFolder(path: string) {
  try {
    const parent = path.replace(/[/\\][^/\\]+$/, '');
    await openPath(parent);
  } catch { /* fallback */ }
}

async function handleCopyPath(path: string) {
  try {
    await navigator.clipboard.writeText(path);
    copied.value = true;
    setTimeout(() => { copied.value = false; }, 2000);
  } catch { /* fallback */ }
}

function extractFileName(path: string): string {
  return path.split(/[/\\]/).pop() || path;
}
</script>
