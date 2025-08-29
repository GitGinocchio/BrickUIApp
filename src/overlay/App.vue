<template>
  <div id="overlay" ref="overlay"></div>
</template>

<script setup lang="ts">
import { LogicalPosition, LogicalSize, getCurrentWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';
import { useNotification } from 'naive-ui';
import { handleClickThrough } from './utils/mouseClickThrough';
import { init, toggleBrick, updateBrick } from './loader';
import { invoke } from '@tauri-apps/api/core';
import { Brick } from 'interfaces/brick';

const currentWindow = getCurrentWindow();
const notification = useNotification();
const isVisible = ref(false);
const overlay = ref<HTMLDivElement>(null);
const bricks = ref<Array<Brick>>();

listen<[number, number]>('global_mouse_moved', async (event) => {
  handleClickThrough(event, currentWindow);
});

listen<{ brick: Brick }>('toggle-brick', async (event) => toggleBrick(event.payload.brick));
listen<{ name: string, prop_name: string, prop_value: string }>('update-brick', async (event) => {
  updateBrick(event.payload.name, event.payload.prop_name, event.payload.prop_value);
});

onMounted(async () => {
  try {
    await currentWindow.setSize(new LogicalSize(window.outerWidth, window.outerHeight));
    await currentWindow.setIgnoreCursorEvents(true);
    await currentWindow.maximize();

    await currentWindow.setPosition(new LogicalPosition(0, 0));
    await currentWindow.show();

    bricks.value = await invoke("get_bricks");

    await init(bricks.value);

    notification.success({
      title: "Bricks loaded successfully!",
      description: "All bricks has been loaded successfully",
      keepAliveOnHover : true,
      duration: 3000,
      closable : true
    });

    isVisible.value = true;
  } 
  catch (error) {
    let technicalMessage = "Unexpected error";

    if (error instanceof Error) {
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

  if (overlay.value.children.length) {
    notification.warning({
      title: "No bricks were found",
      description: "You should create a brick first!",
      keepAliveOnHover : true,
      closable: true
    });
  }
});

listen<any>("changed-not-pos", () => {
  notification.destroyAll();
  notification.info({
    title: "Test notification",
    duration: 750
  });
});
</script>

<style scoped>
#root {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  box-sizing: border-box;
  background: transparent;
  border: none;
  color: white;

  opacity: 0;
  transition: opacity 1s ease-in-out;
  pointer-events: none;
}

#root.show {
  opacity: 1;
}

/*
::v-deep(n-notification-container > n-scrollbar > n-scrollbar-container n-scrollbar-content) {
  padding-bottom: 0 !important;
}
*/
</style>