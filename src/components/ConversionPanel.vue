<template>
  <div style="display:flex;flex-direction:column;flex:1;min-height:0;">
    <FileDropZone v-model="queuedFile" />

    <FormatSelector
      :source-format="sourceFormat"
      :target-format="targetFormat"
      :disabled="status === 'converting'"
      @update:source-format="(v) => sourceFormat = v"
      @update:target-format="(v) => targetFormat = v"
      @swap="swapFormats"
    />

    <button
      class="btn btn-primary"
      :disabled="!canConvert()"
      @click="handleConvert"
    >
      <template v-if="status === 'converting'">
        <span class="spinner" style="width:16px;height:16px;border-width:2px;margin:0"></span>
        {{ t('convert.converting') }}
      </template>
      <template v-else>
        {{ t('convert.btn') }}
      </template>
    </button>

    <ConversionResult
      :result="result"
      :status="status"
      :error-message="errorMessage"
      :duration="duration"
      @close="handleReset"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from '../i18n';
import FileDropZone from './FileDropZone.vue';
import FormatSelector from './FormatSelector.vue';
import ConversionResult from './ConversionResult.vue';
import { VALID_CONVERSIONS } from '../utils/formats';
import type { QueuedFile, FileFormat, ConvertResult, ConversionStatus } from '../types';

const { t } = useI18n();

const queuedFile = ref<QueuedFile | null>(null);
const sourceFormat = ref<FileFormat | null>(null);
const targetFormat = ref<FileFormat | null>(null);
const status = ref<ConversionStatus>('idle');
const result = ref<ConvertResult | null>(null);
const errorMessage = ref('');
const duration = ref('');

// ── Auto-select first valid target when source changes ──
watch(sourceFormat, (src) => {
  if (src) {
    const targets = VALID_CONVERSIONS[src];
    targetFormat.value = targets.length > 0 ? targets[0] : null;
  }
});

// ── File changed: reset state, detect format ──
watch(queuedFile, (file) => {
  if (file) {
    sourceFormat.value = file.detectedFormat;
    status.value = 'idle';
    result.value = null;
    errorMessage.value = '';
    duration.value = '';
  }
});

// ── Keyboard shortcuts ──
function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && canConvert()) {
    e.preventDefault();
    handleConvert();
  }
  if (e.key === 'Escape' && status.value !== 'converting') {
    handleReset();
  }
}
onMounted(() => window.addEventListener('keydown', onKeydown));
onUnmounted(() => window.removeEventListener('keydown', onKeydown));

// ── Swap source/target ──
function swapFormats() {
  if (status.value === 'converting') return;
  const src = sourceFormat.value;
  const tgt = targetFormat.value;
  if (tgt && VALID_CONVERSIONS[tgt]?.includes(src!)) {
    sourceFormat.value = tgt;
    targetFormat.value = src;
  }
}

// ── Convert ──
const canConvert = () =>
  status.value !== 'converting' &&
  queuedFile.value &&
  sourceFormat.value &&
  targetFormat.value;

async function handleConvert() {
  if (!canConvert() || !queuedFile.value || !sourceFormat.value || !targetFormat.value) return;
  status.value = 'converting';
  errorMessage.value = '';
  duration.value = '';
  const start = performance.now();

  try {
    const res = await invoke<ConvertResult>('convert_file', {
      filePath: queuedFile.value.path,
      sourceFormat: sourceFormat.value,
      targetFormat: targetFormat.value,
    });
    result.value = res;
    status.value = 'success';
    duration.value = formatDuration(performance.now() - start);
  } catch (err) {
    errorMessage.value = String(err);
    status.value = 'error';
  }
}

function handleReset() {
  queuedFile.value = null;
  sourceFormat.value = null;
  targetFormat.value = null;
  status.value = 'idle';
  result.value = null;
  errorMessage.value = '';
  duration.value = '';
}

function formatDuration(ms: number): string {
  if (ms < 1000) return `${Math.round(ms)}ms`;
  return `${(ms / 1000).toFixed(1)}s`;
}
</script>
