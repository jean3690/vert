<template>
  <div
    class="drop-zone"
    :class="{ 'drag-over': dragOver, 'has-file': model }"
    @dragover.prevent="dragOver = true"
    @dragleave.prevent="dragOver = false"
    @drop.prevent="dragOver = false"
    @click="!model && browseFile()"
  >
    <div class="drop-zone-inner">
      <!-- Empty -->
      <template v-if="!model">
        <div class="drop-zone-icon">&#128229;</div>
        <div class="drop-zone-title">{{ t('drop.title') }}</div>
        <!-- eslint-disable-next-line vue/no-v-html -->
        <div
          class="drop-zone-hint"
          v-html="t('drop.hint', { link: `<span class='link'>${t('drop.browse')}</span>` })"
        ></div>
      </template>

      <!-- File loaded -->
      <template v-else>
        <div class="file-row">
          <div class="file-row-icon">&#128196;</div>
          <div class="file-row-details">
            <div class="file-row-name" :title="model.name">{{ model.name }}</div>
            <div class="file-row-meta">
              <template v-if="model.detectedFormat">
                <span class="format-badge" :class="FORMAT_CATEGORY[model.detectedFormat as FileFormat]">
                  {{ FORMAT_LABELS[model.detectedFormat as FileFormat] }}
                </span>
              </template>
              <template v-else>{{ t('drop.unknown') }}</template>
            </div>
          </div>
          <button class="file-row-clear" @click.stop="clearFile">&times;</button>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from '../i18n';
import { detectFormat, FORMAT_CATEGORY, FORMAT_LABELS } from '../utils/formats';
import type { QueuedFile, FileFormat } from '../types';

const { t } = useI18n();

const model = defineModel<QueuedFile | null>();

const dragOver = ref(false);
const unlisten: { value: (() => void) | null } = { value: null };

onMounted(async () => {
  try {
    const fn = await getCurrentWindow().onDragDropEvent((event) => {
      if (event.payload.type === 'over') {
        dragOver.value = true;
      } else if (event.payload.type === 'leave') {
        dragOver.value = false;
      } else if (event.payload.type === 'drop') {
        dragOver.value = false;
        if (event.payload.paths.length > 0) {
          handleFilePath(event.payload.paths[0]);
        }
      }
    });
    unlisten.value = fn;
  } catch { /* drag-drop unavailable in browser dev mode */ }
});

onUnmounted(() => unlisten.value?.());

async function handleFilePath(filePath: string) {
  const fileName = filePath.split(/[/\\]/).pop() || filePath;
  const detectedFormat = detectFormat(fileName);
  model.value = { path: filePath, name: fileName, detectedFormat };
}

async function browseFile() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'All Supported',
        extensions: ['properties','yaml','yml','json','toml','xml','csv','docx','md','markdown','html','htm'],
      }],
    });
    if (selected) handleFilePath(selected as string);
  } catch { /* user cancelled */ }
}

function clearFile() { model.value = null; }
</script>

