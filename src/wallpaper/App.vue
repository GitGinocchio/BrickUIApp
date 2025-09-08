<template>
  <NeuralBg></NeuralBg>
</template>

<script setup lang="ts">
import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';
import { onMounted } from 'vue';
import NeuralBg from './NeuralBg.vue';
import { listen } from '@tauri-apps/api/event';

function simulateFakeMouseEvent(x: number, y: number) {
  const event = new MouseEvent('mousemove', {
    clientX: x,
    clientY: y,
    bubbles: true,
    cancelable: true
  });

  document.dispatchEvent(event);

  const event2 = new MouseEvent('pointermove', {
    clientX: x,
    clientY: y,
    bubbles: true,
    cancelable: true
  });
  document.dispatchEvent(event2);
}

onMounted(async () => {
  const window = getCurrentWindow();
  await window.setIgnoreCursorEvents(true);
  await window.show();

  listen<[number, number]>('global_mouse_moved', async (event) => {
    simulateFakeMouseEvent(event.payload[0],event.payload[1]);
  });
});

</script>