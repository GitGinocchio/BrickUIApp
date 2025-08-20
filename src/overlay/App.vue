<template>
  <div id="overlay" ref="overlay" :class="{ show: isVisible }"></div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { LogicalPosition, LogicalSize } from '@tauri-apps/api/window';
import { appendScript } from '@public/js/utils.js';
//import { applyCsp, buildCsp } from '../csp';

import { useNotification } from 'naive-ui';

const notification = useNotification();

const overlay = ref<HTMLElement>();
const isVisible = ref(false);

onMounted(async () => {
  try {
    const currentWindow = getCurrentWindow();
    await currentWindow.setSize(new LogicalSize(window.outerWidth, window.outerHeight));
    await currentWindow.setIgnoreCursorEvents(true);
    await currentWindow.maximize();

    await currentWindow.setPosition(new LogicalPosition(0, 0));
    await currentWindow.show();

    let isClickThroughEnabled = false;

    listen<[number, number]>('global_mouse_moved', async (event) => {
      const [screenX, screenY] = event.payload;
      const element = document.elementFromPoint(screenX, screenY);


      if (element?.tagName === "HTML" && !isClickThroughEnabled) {
        isClickThroughEnabled = true;
        await currentWindow.setIgnoreCursorEvents(true);
        console.log("Enabled click through");
        return;
      } else if (element?.tagName === "HTML") return;

      const shouldEnable = !element;
      if (shouldEnable !== isClickThroughEnabled) {
        isClickThroughEnabled = shouldEnable;
        await currentWindow.setIgnoreCursorEvents(shouldEnable);
        console.log("Disabled click through");
      }
    });

    const externals_url = new URL("../public/js/externals.js", import.meta.url).href;
    console.log('Loading script from URL:', externals_url);
    await appendScript(externals_url, "module");

    //const csp = await buildCsp();
    //await applyCsp(csp);

    /*
      Error occurred while loading bricks:
      Failed to load script: http://tauri.localhost/assets/externals-DgCDLygv.js

      Error: Failed to load script: http://tauri.localhost/assets/externals-DgCDLygv.js
          at w.onerror (http://tauri.localhost/assets/overlay/index-B6FlGL-Y.js:1:13041)
    */

    await appendScript("https://unpkg.com/vue@3.5.18/dist/vue.global.prod.js");
    await appendScript("https://unpkg.com/vue3-sfc-loader@0.9.5/dist/vue3-sfc-loader.js");

    const loader_url = new URL("../public/js/loader.js", import.meta.url).href;
    await appendScript(loader_url, "module");

    notification.success({
      title: "Bricks loaded successfully!",
      description: "All bricks has been loaded successfully",
      keepAliveOnHover : true,
      duration: 3000,
      closable : true
    });

    isVisible.value = true;
  } catch (error) {
    let technicalMessage = "Unexpected error";

    if (error instanceof Error) {
      // mostra messaggio + stack
      technicalMessage = `${error.message}\n\n${error.stack || ""}`;
    } else if (typeof error === "string") {
      technicalMessage = error;
    } else {
      technicalMessage = JSON.stringify(error, null, 2);
    }

    console.error("Errore in onMounted:", error);

    notification.error({
      title: "Something went wrong while loading bricks",
      description: `Error occurred while loading bricks:\n${technicalMessage}`,
      closable : true
    });
    isVisible.value = false;
  }

  if (overlay.value.hasChildNodes()) {
    notification.warning({
      title: "No bricks were found",
      description: "You should create a brick first!",
      keepAliveOnHover : true,
      closable: true
    });
  }
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

</style>