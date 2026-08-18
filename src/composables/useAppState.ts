import { invoke } from "@tauri-apps/api/core";
import { emit, listen, once } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { Brick, Settings, User } from "~/interfaces";

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
    fetchState();
  }, 500);
}

// Manteniamo una promessa condivisa dell'inizializzazione per evitare race conditions
let initPromise: Promise<void> | null = null;

export const useAppState = () => {
  const settings = useState<Settings | null>('settings', () => null);
  const bricks = useState<Brick[]>('bricks', () => []);
  const user = useState<User | null | undefined>('user', () => undefined);
  const systemIsDark = useState<boolean>('systemIsDark', () => false);
  const isReady = useState<boolean>('appStateReady', () => false);
  const isInitialized = useState<boolean>('appStateInitialized', () => false);
  const isSidebarOpen = useState('isSidebarOpen', () => false);
  const isSidebarHidden = useState('isSidebarHidden', () => false);

  const getFullState = () => ({
    settings: settings.value,
    bricks: bricks.value,
    user: user.value,
    systemIsDark: systemIsDark.value
  });

  const updateAll = (payload: any) => {
    if (!payload) return;
    settings.value = payload.settings ?? null;
    bricks.value = payload.bricks ?? [];
    user.value = payload.user ?? null;
    systemIsDark.value = payload.systemIsDark ?? false;
  };

  const init = () => {
    if (initPromise) return initPromise;

    initPromise = (async () => {
      if (import.meta.client) {
        // Inizializza i media query listener solo sul client ed una sola volta
        const mediaQuery = window.matchMedia('(max-width: 1024px)');
        isSidebarHidden.value = mediaQuery.matches;
        
        mediaQuery.addEventListener('change', (e) => {
          isSidebarHidden.value = e.matches;
        });

        const appWindow = getCurrentWindow();
        const isMain = appWindow.label === 'main';

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
        
        isInitialized.value = true;
      }
    })();

    return initPromise;
  };

  return { 
    settings, 
    bricks, 
    user, 
    systemIsDark, 
    init, 
    isReady, 
    isSidebarOpen, 
    isSidebarHidden 
  };
};