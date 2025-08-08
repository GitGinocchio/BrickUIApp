<template>
  <n-space vertical>
    <n-card title="Widgets">
      <n-grid :cols="1" x-gap="16" y-gap="16">
        <n-gi v-for="widget in widgets" :key="widget.name">
          <widget-card :widget="widget" @open-settings="openSettings" />
        </n-gi>
      </n-grid>
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import { BaseDirectory, readDir } from '@tauri-apps/plugin-fs';
import WidgetCard from '/components/WidgetCard.vue'
import { onMounted, ref } from 'vue';

const widgets = ref([]);

onMounted(async () => {
  widgets.value = await readDir('bricks', { baseDir: BaseDirectory.AppData });
});

function openSettings(widgetName: string) {
  // Apri overlay per modificare impostazioni
  console.log('Opening settings for', widgetName)
}
</script>
