// src/plugins/setup.client.ts
import naive from "naive-ui";

export default defineNuxtPlugin(async (nuxtApp) => {
  // Accediamo al composable all'interno del contesto di Nuxt
  const { init } = useAppState();

  // 1. Carichiamo i dati da Rust
  // Usiamo un try/catch perché se Rust fallisce, l'app Nuxt non deve crashare male
  try {
    await init();
  } catch (e) {
    console.error("Errore durante l'invocazione di get_settings da Rust:", e);
  }

  // 3. Registriamo NaiveUI
  nuxtApp.vueApp.use(naive);
  console.log("Setup finished!");
});