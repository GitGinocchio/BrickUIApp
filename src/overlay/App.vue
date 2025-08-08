<template>
  <div id="overlay" :class="{ show: isVisible }"></div>
</template>

<script setup lang="ts"> 
import { onMounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { LogicalPosition, LogicalSize } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import { appendScript } from '../assets/utils';
import { applyCsp, buildCsp } from '../csp';

const isVisible = ref(false);

onMounted(async () => {
  const currentWindow = getCurrentWindow();
  const yOffset = 25;

  await currentWindow.setPosition(new LogicalPosition(0, -yOffset));
  await currentWindow.setSize(new LogicalSize(1920, 1080 + yOffset - 10));

  let isClickThroughEnabled = false;

  document.addEventListener('mousemove', () => {
    console.log('mouse moved');
  });

  listen<[number, number]>('global_mouse_move', async (event) => {
    const [screenX, screenY] = event.payload;
    const element = document.elementFromPoint(screenX, screenY + yOffset);

    if (element?.tagName === "HTML" && !isClickThroughEnabled) {
      isClickThroughEnabled = true;
      await invoke("enable_click_through_command");
      await invoke("hide_titlebar_command");
      return;
    } else if (element?.tagName === "HTML") return;

    const shouldEnable = !element;
    if (shouldEnable !== isClickThroughEnabled) {
      isClickThroughEnabled = shouldEnable;
      await invoke(shouldEnable ? "enable_click_through_command" : "disable_click_through_command");
      await invoke("hide_titlebar_command");
    }
  });

  currentWindow.onFocusChanged(async ({ payload: _focused }) => {
    await invoke("hide_titlebar_command");
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