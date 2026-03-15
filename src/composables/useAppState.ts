import { invoke } from "@tauri-apps/api/core";
import { emit, listen, once } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { darkTheme, lightTheme } from "naive-ui";
import type { Brick } from "~/interfaces/brick";
import type { Settings } from "~/interfaces/settings";
import type { User } from "~/interfaces/user";

export const useAppState = () => {
  const settings = useState<Settings | undefined>('settings', () => undefined);
  const bricks = useState<Brick[]>('bricks', () => []);
  const user = useState<User | null | undefined>('user', () => undefined);
  const systemIsDark = useState<boolean>('systemIsDark', () => false);
  
  // Flag cruciale per la sincronizzazione
  const isReady = useState<boolean>('appStateReady', () => false);
  const isInitialized = useState<boolean>('appStateInitialized', () => false);

  const appWindow = getCurrentWindow();
  const isMain = appWindow.label === 'main';

  const theme = computed(() => {
    switch (settings.value?.theme) {
      case "light": return lightTheme;
      case "dark": return darkTheme;
      case "system": return systemIsDark.value ? darkTheme : lightTheme;
      default: return darkTheme;
    }
  });

  const getFullState = () => ({
    settings: settings.value,
    bricks: bricks.value,
    user: user.value,
    systemIsDark: systemIsDark.value
  });

  const init = async () => {
    if (isInitialized.value) return;
    isInitialized.value = true;

    if (isMain) {
      // --- LOGICA MAIN ---
      
      // Carichiamo i dati
      const [s, b] = await Promise.all([
        invoke<Settings>("get_settings"),
        invoke<Brick[]>("load_bricks")
      ]);
      
      settings.value = s;
      bricks.value = b;
      systemIsDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches;

      // DICHIARIAMO CHE I DATI SONO PRONTI
      isReady.value = true;

      // Listener per le richieste delle altre finestre
      await listen("request-full-state", async () => {
        // Se per qualche motivo isReady fosse ancora false, aspettiamo un ciclo
        if (!isReady.value) {
          const unwatch = watch(isReady, (ready) => {
            if (ready) {
              emit("sync-full-state", getFullState());
              unwatch();
            }
          });
        } else {
          await emit("sync-full-state", getFullState());
        }
      });

      // Watcher per modifiche live (solo dopo il caricamento iniziale)
      watch([settings, bricks, user, systemIsDark], () => {
        if (isReady.value) {
          emit("state-changed", getFullState());
        }
      }, { deep: true });

    } else {
      // --- LOGICA ALTRE FINESTRE ---
      
      // Funzione ricorsiva o con retry per chiedere lo stato
      const fetchState = async () => {
        await emit("request-full-state");
      };

      // Ascolta la risposta
      await once<any>("sync-full-state", (event) => {
        settings.value = event.payload.settings;
        bricks.value = event.payload.bricks;
        user.value = event.payload.user;
        systemIsDark.value = event.payload.systemIsDark;
        isReady.value = true;
      });

      await fetchState();

      // Fallback: se la Main era chiusa o non ha risposto, riprova dopo 500ms
      setTimeout(() => {
        if (!isReady.value) fetchState();
      }, 500);

      await listen<any>("state-changed", (event) => {
        settings.value = event.payload.settings;
        bricks.value = event.payload.bricks;
        user.value = event.payload.user;
        systemIsDark.value = event.payload.systemIsDark;
      });
    }
  };

  return { settings, bricks, user, theme, systemIsDark, init, isReady };
};