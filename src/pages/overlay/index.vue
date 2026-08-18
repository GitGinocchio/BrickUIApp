<template>
  <div id="overlay" ref="overlay" :class="{ ready: isReady }" />
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref, watchEffect, h } from "vue";
import {
  handleClickThrough,
  simulateFakeMouseMoved,
  simulateFakeMousePressed,
} from "#utils/mouseClickThrough";
import { deleteBrick, initLoader, toggleBrick, updateBrickProp } from "~/loader";
import { onBrickError, onBrickWarn } from "#utils/errors";
import type { Brick, Prop } from "#interfaces";
import type { Settings } from "#interfaces";

const { settings, bricks } = useAppState();

definePageMeta({
  layout: 'overlay'
});

const currentWindow = getCurrentWindow();
const toast = useToast();
const { t } = useI18n();

// Create a small compatibility wrapper so functions that expect the naive NotificationApi still work.
const notification = {
  destroyAll: () => toast.clear(),
  success: (opts: any) => toast.add({ title: opts.title, description: opts.description ?? (opts.content ? opts.content() : undefined), color: 'success', duration: opts.duration ?? 3000 }),
  info: (opts: any) => toast.add({ title: opts.title, description: opts.description ?? (opts.content ? opts.content() : undefined), color: 'info', duration: opts.duration ?? 3000 }),
  error: (opts: any) => {
    // If opts.content/action are VNode factories, render them inside description to preserve original layout
    const description = () => h('div', [ opts.content ? opts.content() : null, opts.action ? opts.action() : null ]);
    toast.add({ title: opts.title, description, color: 'error', duration: opts.duration ?? 0 });
  }
};

const isReady = ref(false);
const overlay = ref<HTMLDivElement>();

listen<[number, number, string]>("global_mouse_pressed", async (event) =>
  simulateFakeMousePressed(event)
);

listen<[number, number]>("global_mouse_moved", async (event) =>
  simulateFakeMouseMoved(event.payload[0], event.payload[1])
);

listen<[number, number]>("global_mouse_moved", async (event) =>
  handleClickThrough(event, currentWindow)
);

listen<{ brick: Brick }>(
  "toggle-brick",
  async (event) => await toggleBrick(event.payload.brick)
);
listen<{ brick: Brick }> (
  "delete-brick",
  async (event) => await deleteBrick(event.payload.brick)
);
listen<{ name: string; prop: Prop }>(
  "update-brick",
  async (event) => await updateBrickProp(event.payload.name, event.payload.prop)
);

onMounted(async () => {
  try {
    await currentWindow.hide();
    notification.destroyAll();
    await currentWindow.setIgnoreCursorEvents(true);

    await initLoader(
      bricks.value,
      async (...args) => await onBrickError(toast, ...args),
      async (...args) => await onBrickWarn(toast, ...args)
    );

    notification.success({
          title: t('notifications.bricks_loaded_title'),
          description: t('notifications.bricks_loaded_message'),
      keepAliveOnHover: true,
      duration: 3000,
      closable: true,
    });

    isReady.value = true;

    await currentWindow.show();
  } catch (error) {
    let technicalMessage = t('errors.unexpected');

    if (error instanceof Error) {
      technicalMessage = `${error.message}\n\n${error.stack || ""}`;
    } else if (typeof error === "string") {
      technicalMessage = error;
    } else {
      technicalMessage = JSON.stringify(error, null, 2);
    }

    console.error("Errore in onMounted:", error);

    notification.error({
          title: t('notifications.bricks_load_error_title'),
      description: technicalMessage,
      closable: true,
    });
    isReady.value = false;
  }
});

watch(
  () => JSON.parse(JSON.stringify(settings.value)) as Settings, 
  async (settings, old) => {
    if (!settings) return;

    if (settings.notifications.position !== old.notifications.position) {
      notification.destroyAll();
      notification.info({
              title: t('notifications.test_notification'),
        duration: 750,
      });
    }
  }, 
  { deep: true }
);
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
