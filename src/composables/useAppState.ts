import { invoke } from "@tauri-apps/api/core";
import { emit, listen, once } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { darkTheme, lightTheme } from "naive-ui";
import type { Brick } from "~/interfaces/brick";
import type { Settings } from "~/interfaces/settings";
import type { User } from "~/interfaces/user";

/**
 * Recupera i dati iniziali dal backend Rust (BrickUI Core)
 */
async function fetchInitialData() {
  const [settings, bricks] = await Promise.all([
    invoke<Settings>("get_settings"),
    invoke<Brick[]>("load_bricks")
  ]);
  return { settings, bricks };
}

/**
 * Gestisce la logica di sincronizzazione per la finestra Main
 */
async function setupMainSync(
  isReady: Ref<boolean>, 
  getState: () => any
) {
  // Risponde alle richieste di stato delle altre finestre (es. Overlay, Wallpaper)
  await listen("request-full-state", async () => {
    if (!isReady.value) {
      const unwatch = watch(isReady, (ready) => {
        if (ready) {
          emit("sync-full-state", getState());
          unwatch();
        }
      });
    } else {
      await emit("sync-full-state", getState());
    }
  });

  // Notifica i cambiamenti a tutte le finestre
  watch(getState, (newState) => {
    if (isReady.value) {
      emit("state-changed", newState);
    }
  }, { deep: true });
}

/**
 * Gestisce la logica di sincronizzazione per le finestre secondarie
 */
async function setupSecondarySync(
  updateState: (payload: any) => void,
  setReady: (val: boolean) => void
) {
  const fetchState = () => emit("request-full-state");

  await once<any>("sync-full-state", (event) => {
    updateState(event.payload);
    setReady(true);
  });

  await listen<any>("state-changed", (event) => {
    updateState(event.payload);
  });

  await fetchState();
  
  setTimeout(() => {
    if (!setReady) fetchState();
  }, 500);
}

export const useAppState = () => {
  const settings = useState<Settings | undefined>('settings', () => undefined);
  const bricks = useState<Brick[]>('bricks', () => []);
  const user = useState<User | null | undefined>('user', () => undefined);
  const systemIsDark = useState<boolean>('systemIsDark', () => false);
  const isReady = useState<boolean>('appStateReady', () => false);
  const isInitialized = useState<boolean>('appStateInitialized', () => false);

  const appWindow = getCurrentWindow();
  const isMain = appWindow.label === 'main';

  const getFullState = () => ({
    settings: settings.value,
    bricks: bricks.value,
    user: user.value,
    systemIsDark: systemIsDark.value
  });

  const updateAll = (payload: any) => {
    settings.value = payload.settings;
    bricks.value = payload.bricks;
    user.value = payload.user;
    systemIsDark.value = payload.systemIsDark;
  };

  const init = async () => {
    if (isInitialized.value) return;
    isInitialized.value = true;

    if (isMain) {
      const data = await fetchInitialData();
      settings.value = data.settings;
      bricks.value = data.bricks;
      systemIsDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches;
      
      isReady.value = true;
      await setupMainSync(isReady, getFullState);
    } else {
      await setupSecondarySync(updateAll, (val) => isReady.value = val);
    }
  };

  const theme = computed(() => {
    switch (settings.value?.theme) {
      case "light": return lightTheme;
      case "dark": return darkTheme;
      case "system": return systemIsDark.value ? darkTheme : lightTheme;
      default: return darkTheme;
    }
  });

  return { settings, bricks, user, theme, systemIsDark, init, isReady };
};