import { NNotificationProvider } from "naive-ui";
import App from "./App.vue";
import * as Vue from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Settings } from "../interfaces/settings";
import { listen } from "@tauri-apps/api/event";

listen<any>("changed-not-pos", (event) => {
  settings.value.notifications.position = event.payload.position;
});

const settings = Vue.ref(await invoke<Settings>("get_settings", {}));

const app = Vue.createApp({
  setup() {
    return () => Vue.h(NNotificationProvider, {
      placement : settings.value.notifications.position
    }, { default: () => Vue.h(App) });
  }
});

app.mount("#app");
