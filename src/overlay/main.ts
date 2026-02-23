import { darkTheme, lightTheme } from "naive-ui";
import App from "./App.vue";
import { ref, createApp, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Settings } from "../interfaces/settings";
import { listen } from "@tauri-apps/api/event";
import { Brick } from "interfaces/brick";
//import { catchBrickError } from "../utils/errors";

const media = window.matchMedia('(prefers-color-scheme: dark)')
media.addEventListener('change', updateSystemTheme)

function updateSystemTheme() {
  systemIsDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
}

const settings = ref(await invoke<Settings>("get_settings"));
const bricks = ref(await invoke<Brick[]>("get_bricks"));

listen<Settings>("changed-settings", (event) => {
  settings.value = event.payload;
});

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
app.provide("bricks", bricks);

app.mount("#app");