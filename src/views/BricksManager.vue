<template>
  <n-space vertical>
    <n-card title="Bricks">
      <n-grid :cols="1" x-gap="16" y-gap="16">
        <n-gi v-for="brick in bricks" :key="brick.name">
          <BrickCard 
            :brick="brick" 
            @open-settings="openSettings" 
            @toggle="onToggle(brick)"
          />
        </n-gi>
      </n-grid>
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import BrickCard from "../components/BrickCard.vue"
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { emitTo } from '@tauri-apps/api/event';
import { Brick } from 'interfaces/brick';

const bricks = ref<Brick[]>([]);

onMounted(async () => {
  bricks.value = await invoke<Brick[]>("get_bricks", {});
});

async function onToggle(brick: Brick) {
  await emitTo("window", "toggle_brick", { name: brick.name });
}

function openSettings(widgetName: string) {
  // Apri overlay per modificare impostazioni
  console.log('Opening settings for', widgetName)
}
</script>
