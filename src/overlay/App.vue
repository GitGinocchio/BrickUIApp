<template>
  <div id="overlay" :class="{ show: isVisible }"></div>
</template>

<script setup lang="ts"> 
import { onMounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { LogicalPosition, LogicalSize } from '@tauri-apps/api/window';
import { appendScript } from '../assets/utils';
import { applyCsp, buildCsp } from '../csp';
import { invoke } from '@tauri-apps/api/core';

const isVisible = ref(false);

onMounted(async () => {
  const currentWindow = getCurrentWindow();
  await currentWindow.setSize(new LogicalSize(window.outerWidth, window.outerHeight));
  await currentWindow.setIgnoreCursorEvents(true);
  await currentWindow.maximize();

  // Questo rimuove un bordo di 1px sopra (non so come)
  await currentWindow.setPosition(new LogicalPosition(0, 0));

  await currentWindow.show();

  let isClickThroughEnabled = false;

  listen<[number, number]>('global_mouse_moved', async (event) => {
    const [screenX, screenY] = event.payload;
    const element = document.elementFromPoint(screenX, screenY);

    if (element?.tagName === "HTML" && !isClickThroughEnabled) {
      isClickThroughEnabled = true;
      await currentWindow.setIgnoreCursorEvents(true);
      return;
    } else if (element?.tagName === "HTML") return;

    const shouldEnable = !element;
    if (shouldEnable !== isClickThroughEnabled) {
      isClickThroughEnabled = shouldEnable;
      await currentWindow.setIgnoreCursorEvents(shouldEnable);
    }
  });

  await appendScript("../assets/externals.js", "module");

  const csp = await buildCsp();
  await applyCsp(csp);

  await appendScript("https://unpkg.com/vue@3.5.18/dist/vue.global.prod.js");
  await appendScript("https://unpkg.com/vue3-sfc-loader@0.9.5/dist/vue3-sfc-loader.js");
  await appendScript("../assets/loader.js", "module");

  isVisible.value = true;
  
}); 

</script>

<style scoped>
#overlay {
  width: min-content;
  height: 100%;
  margin-top: 3vh;
  overflow: hidden;
  overflow-y: hidden;
  box-sizing: border-box;
  background: transparent;
  color: white;

  opacity: 0;
  transition: opacity 1s ease-in-out;
  pointer-events: none;
}

#overlay.show {
  opacity: 1;
}

.warning {
  width: max-content;
  color: orange;
  pointer-events: none;
}
</style>