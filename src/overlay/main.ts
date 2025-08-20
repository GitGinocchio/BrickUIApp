import { NNotificationProvider } from "naive-ui";
import App from "./App.vue";
import * as Vue from "vue";
import { expose } from "./apis";
import { invoke } from "@tauri-apps/api/core";
import { Settings } from "../interfaces/settings";

expose();

const settings = await invoke<Settings>("get_settings", {});

const app = Vue.createApp({
  setup() {
    return () => Vue.h(NNotificationProvider, {
      placement : settings.notifications.position
    }, { default: () => Vue.h(App) });
  }
});

app.mount("#app");
