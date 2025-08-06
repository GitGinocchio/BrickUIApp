<script setup lang="ts">
import { ref, onMounted, computed } from "vue"
import { getCurrentWindow } from "@tauri-apps/api/window"

// Importa le pagine
import Settings from "./pages/Settings.vue";
import Overlay from "./pages/Overlay.vue";

const windowLabel = computed(() => {
  const current = getCurrentWindow();
  console.log(`Window label: ${current.label}`);
  return current.label;
});
</script>

<template>
  <div v-if="windowLabel">
    <component :is="windowLabel === 'overlay' ? Overlay : Settings" />
  </div>
  <div v-else>
    Loading...
  </div>
</template>

<style scoped>
/* Nessuno stile specifico */
</style>
