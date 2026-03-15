// src/plugins/setup.client.ts
import naive from "naive-ui";

export default defineNuxtPlugin(async (nuxtApp) => {
  // Accediamo al composable all'interno del contesto di Nuxt
  const { init, systemIsDark } = useAppState();

  // 1. Carichiamo i dati da Rust
  // Usiamo un try/catch perché se Rust fallisce, l'app Nuxt non deve crashare male
  try {
    await init();
  } catch (e) {
    console.error("Errore durante l'invocazione di get_settings da Rust:", e);
  }

  // 2. Listener per il tema (il file è .client.ts, quindi window esiste sicuramente)
  const media = window.matchMedia('(prefers-color-scheme: dark)');
  systemIsDark.value = media.matches; // Impostiamo il valore iniziale corretto

  media.addEventListener('change', (e) => {
    systemIsDark.value = e.matches;
  });

  // 3. Registriamo NaiveUI
  nuxtApp.vueApp.use(naive);
  console.log("Setup finished!");
});