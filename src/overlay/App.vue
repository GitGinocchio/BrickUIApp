<template>
  <div id="overlay" ref="overlay" :class="{ ready: isReady }"></div>
</template>

<script setup lang="ts">
import { LogicalPosition, LogicalSize, getCurrentWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { ComponentPublicInstance, h, inject, onMounted, Ref, ref } from 'vue';
import { NButton, useNotification } from 'naive-ui';
import { handleClickThrough, simulateFakeMousePressed } from '../utils/mouseClickThrough';
import { initLoader, toggleBrick, updateBrick } from '../loader';
import { invoke } from '@tauri-apps/api/core';
import { Brick } from 'interfaces/brick';
import { Settings } from 'interfaces/settings';

const settings = inject("settings") as Ref<Settings>;

const currentWindow = getCurrentWindow();
const notification = useNotification();
const lastNotificationPosition = ref<string>(settings.value.notifications.position);
const lastTaskBarBehavior = ref<string>(settings.value.taskbar.behavior);
const isReady = ref(false);
const overlay = ref<HTMLDivElement>();
const bricks = ref<Array<Brick>>();

listen<[number, number, string]>('global_mouse_pressed', async (event) => simulateFakeMousePressed(event));
listen<[number, number]>('global_mouse_moved', async (event) => handleClickThrough(event, currentWindow));
listen<{ brick: Brick }>('toggle-brick', async (event) => await toggleBrick(event.payload.brick));
listen<{ name: string, prop_name: string, prop_value: string }>('update-brick', async (event) => {
  updateBrick(event.payload.name, event.payload.prop_name, event.payload.prop_value);
});

async function onBrickError(
  error: Error, 
  instance: ComponentPublicInstance | { name: string, author: string }, 
  info: string
) {
  console.log(error, instance, info);
  if (!instance) return;
  if ('type' in instance) return;
  let brickName: string;
  let brickAuthor: string;

  if ('$options' in instance) {
    while ('$options' in instance && !('__brickContext' in instance.$options)) {
      instance = instance.$parent;
    }
    brickName = instance.$options?.__brickContext?.name;
    brickAuthor = instance.$options?.__brickContext?.author;
  }
  else if ('name' in instance && 'author' in instance) {
    brickName = instance.name;
    brickAuthor = instance.author;
  }

  notification.error({
    // @ts-ignore
    title: `Error ${info ? `in ${info}` : ''} — brick "${brickName}"${brickAuthor !== "undefined" ? ` by ${brickAuthor}` : ""}`,
    content: () => {
      return h('code', {
        style: {
          whiteSpace: 'pre-wrap',
          fontSize: '12px',
          color: '#FFFFFF85'
        }
      }, `${error.message}\n${error.cause ?? ''}`);
    },
    action: () => {
      return h(NButton, {
        style: { color: 'white' },
        onClick: async () => {
          // @ts-ignore
          await invoke('open_brick', { brickName: brickName });
        }
      }, {
        default: () => 'Open Brick'
      });
    },
    keepAliveOnHover : true,
    duration: 10000
  });
}

async function onBrickWarn(
  message: string, 
  instance: ComponentPublicInstance | { name: string, author: string }, 
  _trace: string
) {
  let brickName: string;
  let brickAuthor: string;
  if ('$options' in instance) {
    brickName = instance.$options.__brickContext.name;
    brickAuthor = instance.$options.__brickContext.author;
  }
  else if ('name' in instance && 'author' in instance) {
    brickName = instance.name;
    brickAuthor = instance.author;
  }

  notification.error({
    title: `Warn in brick "${brickName}"${brickAuthor !== "undefined" ? ` by ${brickAuthor}` : ""}`,
    content: () => {
      return h('code', {
        style: {
          whiteSpace: 'pre-wrap',
          fontSize: '12px',
          color: '#FFFFFF85'
        }
      }, `${message}`);
    },
    action: () => {
      return h(NButton, {
        style: { color: 'white' },
        onClick: async () => {
          // @ts-ignore
          await invoke('open_brick', { brickName: brickName });
        }
      }, {
        default: () => 'Open Brick'
      });
    },
    keepAliveOnHover : true,
    duration: 10000
  });
}

onMounted(async () => {
  try {
    notification.destroyAll();
    await currentWindow.setIgnoreCursorEvents(true);
    await currentWindow.maximize();
    await currentWindow.setSize(new LogicalSize(window.outerWidth, window.outerHeight));
    await currentWindow.setPosition(new LogicalPosition(0, 0));

    bricks.value = await invoke("get_bricks");

    await initLoader(bricks.value, onBrickError, onBrickWarn);

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
      title: "Error occurred while loading bricks",
      description: technicalMessage,
      closable : true
    });
    isReady.value = false;
  }

  if (overlay.value && overlay.value.children.length === 0) {
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