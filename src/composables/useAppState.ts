// src/composables/useAppState.ts
import { invoke } from "@tauri-apps/api/core";
import { darkTheme, lightTheme } from "naive-ui";

import type { Settings } from "#interfaces/settings";
import type { Brick } from "#interfaces/brick";
import type { User } from "#interfaces/user";

export const useAppState = () => {
  // Stati reattivi globali (Nuxt li mantiene sincronizzati)
  const settings = useState<Settings>('settings');
  const bricks = useState<Brick[]>('bricks',() => []);
  const user = useState<User | null>('user', () => null);
  const systemIsDark = useState<boolean>('systemIsDark', () => false);

  // Inizializzazione dati da Rust
  const init = async () => {
    settings.value = await invoke<Settings>("get_settings");
    bricks.value = await invoke<Brick[]>("load_bricks");
    systemIsDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches;
  };

  // Logica del Tema
  const theme = computed(() => {
    switch (settings.value?.theme) {
      case "light": return lightTheme;
      case "dark": return darkTheme;
      case "system": return systemIsDark.value ? darkTheme : lightTheme;
      default: return darkTheme;
    }
  });

  return { settings, bricks, user, theme, systemIsDark, init };
};