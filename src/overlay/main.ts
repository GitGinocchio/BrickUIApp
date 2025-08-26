import { NNotificationProvider } from "naive-ui";
import App from "./App.vue";
import * as Vue from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Settings } from "../interfaces/settings";
import { listen } from "@tauri-apps/api/event";

listen<Settings>("settings-update", (event) => {
  settings.value = event.payload;
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
