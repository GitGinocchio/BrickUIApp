// src/plugins/global-events.client.ts

export default defineNuxtPlugin(() => {
  // Verifichiamo se siamo in produzione (come nel tuo file originale)
  // In Nuxt si usa import.meta.dev (false in produzione)
  if (!import.meta.dev) {
    window.addEventListener('contextmenu', (e: MouseEvent) => {
      const target = e.target as HTMLElement;

      // Blocca solo su elementi “generici” come div o span
      const allowedTags = ['IMG', 'A', 'VIDEO', 'CANVAS', 'TEXTAREA', 'INPUT'];
      if (!allowedTags.includes(target.tagName)) {
        e.preventDefault();
      }
    });
  }

  // Puoi aggiungere qui altri listener globali se ne avrai bisogno
  console.log("BrickUI: Global protection events active");
});