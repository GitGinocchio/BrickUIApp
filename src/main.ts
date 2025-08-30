import { computed, createApp, ref } from "vue";
import { createNaiveUi } from "./naive";
import { router } from './router'

import App from "./App.vue";
import i18n from "./i18n";
import { invoke } from "@tauri-apps/api/core";
import { Settings } from "interfaces/settings";
import { darkTheme, lightTheme } from "naive-ui";

const media = window.matchMedia('(prefers-color-scheme: dark)')
media.addEventListener('change', updateSystemTheme)

function updateSystemTheme() {
  systemIsDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
}

const settings = ref(await invoke<Settings>("get_settings", {}));

const systemIsDark = ref(false)

const theme = computed(() => {
  switch (settings.value.theme) {
    case "light":
      return lightTheme
    case "dark":
      return darkTheme
    case "system":
      updateSystemTheme();
      return systemIsDark.value ? darkTheme : lightTheme
  }
});

const app = createApp(App);
app.provide("settings", settings);
app.provide("theme", theme);

app.use(router)
app.use(createNaiveUi());
app.use(i18n);
app.mount("#app");