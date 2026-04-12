<template>
  <div class="flex h-screen overflow-hidden bg-default">
    <UDashboardSidebar
      collapsible
      :is-collapsed="collapsed"
      class="transition-all duration-300 border-r border-default flex flex-col justify-between"
      :class="settings.sidebar.position === 'right' ? 'order-1 border-l border-r-0' : 'order-0'"
      :style="{ width: collapsed ? '64px' : '220px' }"
      @mouseenter="handleMouseEnter"
      @mouseleave="handleMouseLeave"
    >
      <template #header>
        <div class="h-12 flex items-center px-4 overflow-hidden">
          <UIcon name="i-lucide-box" class="w-6 h-6 min-w-6 text-primary" />
          <span v-if="!collapsed" class="ml-3 font-bold truncate text-lg">BrickUI</span>
        </div>
      </template>

      <div class="flex-1 px-2 py-2">
        <UNavigationMenu
          :links="mainNavigation"
          orientation="vertical"
          :collapsed="collapsed"
          :ui="{ link: 'px-3 py-2.5' }"
        />
      </div>

      <template #footer>
        <div class="px-2 pb-4">
          <UNavigationMenu
            :links="bottomNavigation"
            orientation="vertical"
            :collapsed="collapsed"
            :ui="{ link: 'px-3 py-2.5' }"
          />
        </div>
      </template>
    </UDashboardSidebar>

    <UMain 
      class="flex-1 overflow-y-auto relative" 
      @scroll="handleScroll"
    >
      <UContainer class="py-6 min-h-full">
        <slot />
      </UContainer>
      
      <SystemTray v-if="settings?.systemtray" class="fixed bottom-4 right-4 z-50" />
    </UMain>
  </div>
</template>

<script setup lang="ts">
import { useAppState } from '~/composables/useAppState';
import { useBrickActions } from "~/composables/useBrickActions";
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Settings } from "~/interfaces/settings";

const { settings, theme, bricks } = useAppState();
const toast = useToast();
const { t, setLocale } = useI18n();
const router = useRouter();
const route = useRoute();

// Setup Actions (adattato per Nuxt UI Toast)
const { openBrick } = useBrickActions(bricks, theme, null, null);

// Gestione Sidebar (Hover logic)
const collapsed = ref(true);
let hoverTimer: ReturnType<typeof setTimeout> | null = null;

function handleMouseEnter() {
  hoverTimer = setTimeout(() => {
    collapsed.value = false;
  }, 250);
}

function handleMouseLeave() {
  if (hoverTimer) clearTimeout(hoverTimer);
  collapsed.value = true;
}

// Gestione Scroll (per effetti CSS dinamici nei Bricks)
function handleScroll(e: Event) {
  const target = e.target as HTMLElement;
  document.documentElement.style.setProperty('--layout-scroll-top', `${target.scrollTop}px`);
}

// Configurazione Link (Nuxt UI v3 usa Iconify i-lucide-*)
const mainNavigation = computed(() => [
  { label: t('bricks'), icon: 'i-lucide-cuboid', to: '/bricks' },
  { label: t('walls'), icon: 'i-lucide-layout-dashboard', to: '/walls' },
  { label: t('marketplace'), icon: 'i-lucide-store', to: '/marketplace' }
]);

const bottomNavigation = computed(() => [
  { 
    label: t('user'), 
    icon: 'i-lucide-circle-user', 
    to: '/get-started',
    active: route.path.startsWith('/user') || route.path === '/get-started'
  },
  { label: t('settings'), icon: 'i-lucide-settings', to: '/settings' }
]);

// Tauri Events & Lifecycle
onMounted(() => {
  setLocale(settings.value.language);
  
  // Ascolta comandi dal Backend Rust
  listen<{ view: string }>("goto", (event) => router.push(event.payload.view));
  listen<[string, string]>("open_brick", (event) => openBrick(event.payload));
});

// Watcher per la sincronizzazione con Rust (BrickUIState)
watch(
  () => JSON.parse(JSON.stringify(settings.value)) as Settings, 
  async (newSettings, oldSettings) => {
    if (!newSettings) return;
    
    // Salva nel file di config tramite Rust
    await invoke("save_settings", { settings: newSettings });
    setLocale(newSettings.language);

    // Gestione Taskbar Windows via API
    if (oldSettings && oldSettings.taskbar.behavior !== newSettings.taskbar.behavior) {
      await invoke(newSettings.taskbar.behavior === 'hide' ? "hide_taskbar" : "show_taskbar");
    }
  }, 
  { deep: true }
);
</script>

<style scoped>
/* Transizione fluida per il ridimensionamento della sidebar */
.transition-all {
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 300ms;
}
</style>