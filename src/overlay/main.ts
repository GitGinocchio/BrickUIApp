import { darkTheme, lightTheme, NNotificationProvider } from "naive-ui";
import App from "./App.vue";
import { provide, h, ref, createApp, computed, watchEffect} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Settings } from "../interfaces/settings";
import { listen } from "@tauri-apps/api/event";

const media = window.matchMedia('(prefers-color-scheme: dark)')
media.addEventListener('change', updateSystemTheme)

function updateSystemTheme() {
  systemIsDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
}

const settings = ref(await invoke<Settings>("get_settings", {}));

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

const app = createApp({
  setup() {
    provide("settings", settings);
    provide("theme", theme);

    // Prop reattiva tramite ref locale
    const currentNotificationTheme = ref(theme.value.Notification);

    // Aggiorna automaticamente il tema delle notifiche quando cambia il tema globale
    watchEffect(() => {
      currentNotificationTheme.value = theme.value.Notification;
    });

    return () => h(NNotificationProvider, {
      theme: currentNotificationTheme.value,
      placement : settings.value.notifications.position
    }, { default: () => h(App) });
  }
});

app.mount("#app");