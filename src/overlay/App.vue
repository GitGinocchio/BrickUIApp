<template>
  <iframe
    id="root"
    ref="sandbox"
    :class="{ show: isVisible }" 
    src="/sandbox/index.html"
    sandbox="allow-scripts">
  </iframe>
</template>

<script setup lang="ts">
import { LogicalPosition, LogicalSize, getCurrentWindow } from '@tauri-apps/api/window';
import { BaseDirectory, appDataDir as getAppDataDir } from '@tauri-apps/api/path';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import { readTextFile } from '@tauri-apps/plugin-fs';
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';
import { useNotification } from 'naive-ui';
import { Brick } from 'interfaces/brick';
import { sendResponseRequest } from './utils';

const notification = useNotification();

const isVisible = ref(false)
const sandbox = ref<HTMLIFrameElement>();
const channel = ref<MessageChannel|null>(null);
const port = ref<MessagePort|null>(null);
const key = ref<string>(crypto.randomUUID());

const appDataDir = ref<string|null>(null);

async function onSandboxLoaded() {
  channel.value = new MessageChannel();
  port.value = channel.value.port1;
  port.value.onmessage = onMessage;
  port.value.onmessageerror = (error) => console.error(error);

  const bricks : Brick[] = await invoke("get_bricks", {});

  sandbox.value.contentWindow.postMessage(
    { type: "init", key: key.value, path: appDataDir.value, bricks: bricks }, 
    "*", 
    [channel.value.port2]
  );
}

async function onMessage(event: MessageEvent) {
  if (event.data.key !== key.value) return;

  switch (event.data.type) {
    case "get-file-content":
      const content = await readTextFile(event.data.payload.path, { baseDir: BaseDirectory.AppData });
      port.value.postMessage({ type: "file-content", key: key.value , payload: { content: content }});
      break;
    case "get-app-data-dir":
      port.value.postMessage({ type: "app-data-dir", key: key.value, payload: { path: appDataDir.value } });
      break;
    case "get-convert-file-src":
      const converted = convertFileSrc(event.data.payload.path);
      port.value.postMessage({ type: "convert-file-src", key: key.value, payload: { path: converted }});
      break;
  }
}

onMounted(async () => {
  try {
    appDataDir.value = await getAppDataDir();

    const currentWindow = getCurrentWindow();
    await currentWindow.setSize(new LogicalSize(window.outerWidth, window.outerHeight));
    await currentWindow.setIgnoreCursorEvents(true);
    await currentWindow.maximize();

    await currentWindow.setPosition(new LogicalPosition(0, 0));
    await currentWindow.show();

    if (sandbox.value) onSandboxLoaded();
    else sandbox.value.addEventListener("load", onSandboxLoaded);

    let isClickThroughEnabled = false;

    const elementsToSkip = [
      "HTML",
      "BODY"
    ]

    listen<[number, number]>('global_mouse_moved', async (event) => {
      const [screenX, screenY] = event.payload;
      let elementTagName = document.elementFromPoint(screenX, screenY)?.tagName;

      if (elementsToSkip.includes(elementTagName)) {
        const payload = await sendResponseRequest(port.value, key.value, "element-from-point", { x: screenX, y: screenY }) as { element: { tagName: string, id: string, classList: Array<string>, rect: DOMRect }};
        elementTagName = payload.element.tagName;
      }

      if (elementsToSkip.includes(elementTagName) && !isClickThroughEnabled) {
        isClickThroughEnabled = true;
        await currentWindow.setIgnoreCursorEvents(true);
        console.log("Enabled click through");
        return;
      } else if (elementsToSkip.includes(elementTagName)) return;

      const shouldEnable = !elementTagName;
      if (shouldEnable !== isClickThroughEnabled) {
        isClickThroughEnabled = shouldEnable;
        await currentWindow.setIgnoreCursorEvents(shouldEnable);
        console.log("Disabled click through", elementTagName);
      }
    });

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

  if (sandbox.value.hasChildNodes()) {
    notification.warning({
      title: "No bricks were found",
      description: "You should create a brick first!",
      keepAliveOnHover : true,
      closable: true
    });
  }
});

listen("toggle-brick", (event) => {
  port.value.postMessage({ type: `toggle-brick`, key: key.value, payload: event.payload });
});

listen("update-brick", (event) => {
  port.value.postMessage({ type: `update-brick`, key: key.value, payload: event.payload });
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