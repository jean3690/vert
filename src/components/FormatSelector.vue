<template>
  <div class="format-section">
    <!-- From -->
    <div class="format-source">
      <div class="format-section-label">{{ t('format.from') }}</div>
      <template v-for="entry in groupedSources" :key="entry.group">
        <div class="format-chips">
          <button
            v-for="fmt in entry.formats"
            :key="fmt"
            class="format-chip"
            :class="{ selected: sourceFormat === fmt, disabled }"
            @click="selectSource(fmt)"
          >
            <span class="dot" :class="FORMAT_CATEGORY[fmt]"></span>
            {{ FORMAT_LABELS[fmt] }}
          </button>
        </div>
      </template>
    </div>

    <div class="format-arrow">
      <button
        v-if="canSwap"
        class="swap-btn"
        :title="t('format.swap')"
        @click="emit('swap')"
      >
        &#8645;
      </button>
      <span class="arrow-icon" aria-hidden="true"></span>
    </div>

    <!-- To -->
    <div class="format-target">
      <div class="format-section-label">{{ t('format.to') }}</div>
      <div class="format-chips">
        <template v-if="!sourceFormat">
          <span class="chip-placeholder">{{ t('format.placeholder') }}</span>
        </template>
        <template v-else-if="targetFormats.length === 0">
          <span class="chip-placeholder">{{ t('format.none') }}</span>
        </template>
        <button
          v-for="fmt in targetFormats"
          :key="fmt"
          class="format-chip"
          :class="{ selected: targetFormat === fmt, disabled }"
          @click="selectTarget(fmt)"
        >
          <span class="dot" :class="FORMAT_CATEGORY[fmt]"></span>
          {{ FORMAT_LABELS[fmt] }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from '../i18n';
import { FORMAT_LABELS, FORMAT_CATEGORY, VALID_CONVERSIONS } from '../utils/formats';
import type { FileFormat } from '../types';

const { t } = useI18n();

const props = defineProps<{
  sourceFormat: FileFormat | null;
  targetFormat: FileFormat | null;
  disabled: boolean;
}>();

const emit = defineEmits<{
  'update:sourceFormat': [value: FileFormat | null];
  'update:targetFormat': [value: FileFormat | null];
  swap: [];
}>();

const sourceFormats: { fmt: FileFormat; group: string }[] = [
  { fmt: 'properties', group: 'Config' },
  { fmt: 'yaml', group: 'Config' },
  { fmt: 'json', group: 'Config' },
  { fmt: 'toml', group: 'Config' },
  { fmt: 'xml', group: 'Config' },
  { fmt: 'ini', group: 'Config' },
  { fmt: 'csv', group: 'Data' },
  { fmt: 'epub', group: 'Document' },
  { fmt: 'docx', group: 'Document' },
  { fmt: 'pdf', group: 'Document' },
  { fmt: 'markdown', group: 'Document' },
  { fmt: 'html', group: 'Document' },
  { fmt: 'txt', group: 'Document' },
  { fmt: 'svg', group: 'Image' },
  { fmt: 'png', group: 'Image' },
  { fmt: 'jpeg', group: 'Image' },
  { fmt: 'gif', group: 'Image' },
  { fmt: 'webp', group: 'Image' },
  { fmt: 'bmp', group: 'Image' },
  { fmt: 'tiff', group: 'Image' },
];

const groupedSources = computed(() => {
  const result: { group: string; formats: FileFormat[] }[] = [];
  const map: Record<string, FileFormat[]> = {};
  for (const { fmt, group } of sourceFormats) {
    if (!map[group]) { map[group] = []; result.push({ group, formats: map[group] }); }
    map[group].push(fmt);
  }
  return result;
});

const targetFormats = computed<FileFormat[]>(() => {
  if (!props.sourceFormat) return [];
  return VALID_CONVERSIONS[props.sourceFormat];
});

const canSwap = computed(() => {
  if (!props.sourceFormat || !props.targetFormat) return false;
  return VALID_CONVERSIONS[props.targetFormat]?.includes(props.sourceFormat);
});

function selectSource(fmt: FileFormat) {
  if (props.disabled) return;
  emit('update:sourceFormat', props.sourceFormat === fmt ? null : fmt);
}

function selectTarget(fmt: FileFormat) {
  if (props.disabled || !props.sourceFormat) return;
  emit('update:targetFormat', props.targetFormat === fmt ? null : fmt);
}
</script>
