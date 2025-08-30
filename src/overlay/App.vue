<template>
  <div id="overlay" ref="overlay" :class="{ ready: isReady }"></div>
</template>

<script setup lang="ts">
import { LogicalPosition, LogicalSize, getCurrentWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { inject, onMounted, Ref, ref } from 'vue';
import { useNotification } from 'naive-ui';
import { handleClickThrough } from './utils/mouseClickThrough';
import { initLoader, toggleBrick, updateBrick } from './loader';
import { initTrayIcon } from './tray';
import { invoke } from '@tauri-apps/api/core';
import { Brick } from 'interfaces/brick';
import { Settings } from 'interfaces/settings';

const settings = inject("settings") as Ref<Settings>;

const currentWindow = getCurrentWindow();
const notification = useNotification();
const lastNotificationPosition = ref<string>(settings.value.notifications.position);
const lastTaskBarBehavior = ref<string>(settings.value.taskbar.behavior);
const isReady = ref(false);
const overlay = ref<HTMLDivElement>(null);
const bricks = ref<Array<Brick>>();

listen<[number, number]>('global_mouse_moved', async (event) => handleClickThrough(event, currentWindow));
listen<{ brick: Brick }>('toggle-brick', async (event) => toggleBrick(event.payload.brick));
listen<{ name: string, prop_name: string, prop_value: string }>('update-brick', async (event) => {
  updateBrick(event.payload.name, event.payload.prop_name, event.payload.prop_value);
});

onMounted(async () => {
  try {
    await currentWindow.setIgnoreCursorEvents(true);
    await currentWindow.maximize();
    await currentWindow.setSize(new LogicalSize(window.outerWidth, window.outerHeight));
    await currentWindow.setPosition(new LogicalPosition(0, 0));

    bricks.value = await invoke("get_bricks");

    await initLoader(bricks.value);
    await initTrayIcon(bricks.value);

    notification.success({
      title: "Bricks loaded successfully!",
      description: "All bricks has been loaded successfully",
      keepAliveOnHover : true,
      duration: 3000,
      closable : true
    });

    isReady.value = true;

    await currentWindow.hide();
    await currentWindow.show();
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
    isReady.value = false;
  }

  if (overlay.value.children.length === 0) {
    notification.warning({
      title: "No bricks were found",
      description: "You should create a brick first!",
      keepAliveOnHover : true,
      closable: true
    });
  }
});

listen<Settings>("changed-settings",async (event) => {
  if (event.payload.notifications.position !== lastNotificationPosition.value) {
    notification.destroyAll();
    notification.info({
      title: "Test notification",
      duration: 750
    });
    lastNotificationPosition.value = event.payload.notifications.position;
  };

  if (event.payload.taskbar.behavior !== lastTaskBarBehavior.value) {
    lastTaskBarBehavior.value = event.payload.taskbar.behavior;
    await currentWindow.maximize();
    await currentWindow.setSize(new LogicalSize(window.outerWidth, window.outerHeight));
    await currentWindow.setPosition(new LogicalPosition(0, 0));
  }
});
</script>

<style scoped>
#overlay {
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

#overlay.ready {
  opacity: 1;
}

/*
::v-deep(n-notification-container > n-scrollbar > n-scrollbar-container n-scrollbar-content) {
  padding-bottom: 0 !important;
}
*/
</style>