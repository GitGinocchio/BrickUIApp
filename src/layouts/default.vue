<template>
  <div 
    class="flex flex-1 h-screen overflow-auto bg-default"
    :class="settings.sidebar.position === 'right' ? 'flex-row-reverse' : 'flex-row'"
  >
    <USidebar
      v-model:open="isSidebarOpen"
      collapsible="icon"
      variant="sidebar"
      :side="settings.sidebar.position"
      :rail="false"
      @mouseenter="handleMouseEnter"
      @mouseleave="handleMouseLeave"
    >
      <template #header>
        <div class="flex items-center gap-3 px-1 overflow-hidden h-6">
          <UIcon name="i-lucide-cuboid" class="size-6 text-primary shrink-0" />
          <span 
            v-if="isSidebarOpen" 
            class="font-bold truncate text-lg transition-opacity duration-200"
          >
            BrickUI
          </span>
        </div>
      </template>

      <template #default="{ state }">
        <UNavigationMenu
          :key="state"
          :items="getTopNavigation()"
          orientation="vertical"
          :ui="{ link: 'p-1.5 overflow-hidden', list: 'flex flex-col gap-1' }"
        />
      </template>

      <template #footer="{ state }">
        <UNavigationMenu
          :key="state"
          :items="getBottomNavigation()"
          :collapsed="!isSidebarOpen"
          orientation="vertical"
          class="w-full"
          :ui="{ link: 'p-1.5 overflow-hidden', list: 'flex flex-col w-full gap-1' }"
        />
      </template>
    </USidebar>

    <div class="flex-1 flex flex-col min-w-0">
      <div class="flex-1 flex flex-col min-w-0 h-full">
        <UMain class="flex-1 overflow-hidden relative flex flex-col">
          <slot />
          <ImportBrickModal ref="importBrickModal" />
          <NewBrickModal ref="newBrickModal" />
          <RenameBrickModal ref="renameBrickModal" />
          <GlobalConfirmModal />
        </UMain>
      </div>

      <SystemTray v-if="settings?.systemtray" />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui';
import { useAppState } from '~/composables/useAppState';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Settings, Brick } from "~/interfaces";

const ImportBrickModal = defineAsyncComponent(() => import('~/components/modals/ImportBrickModal.vue'));
const RenameBrickModal = defineAsyncComponent(() => import('~/components/modals/RenameBrickModal.vue'));
const NewBrickModal = defineAsyncComponent(() => import('~/components/modals/NewBrickModal.vue'));
const GlobalConfirmModal = defineAsyncComponent(() => import('~/components/modals/GlobalConfirmModal.vue'));

const importBrickModal = useTemplateRef('importBrickModal');
const renameBrickModal = useTemplateRef('renameBrickModal');
const newBrickModal = useTemplateRef('newBrickModal');

const { settings, isSidebarOpen, isSidebarHidden } = useAppState();
const { t, setLocale } = useI18n();
const router = useRouter();
const route = useRoute();

let hoverTimer: ReturnType<typeof setTimeout> | null = null;

// Logica Hover mantenuta
function handleMouseEnter() {
  hoverTimer = setTimeout(() => { isSidebarOpen.value = true; }, 150);
}

function handleMouseLeave() {
  if (hoverTimer) {
    clearTimeout(hoverTimer);
    hoverTimer = null;
  }
  isSidebarOpen.value = false;
}

// Navigazione dinamica per gestire lo stato 'expanded' vs 'collapsed'
function getTopNavigation() {
  return [
    { 
      label: t('bricks.title'), 
      icon: 'i-lucide-blocks', 
      to: '/bricks', 
      active: route.path === '/bricks',
      onSelect: () => {
        if (isSidebarHidden.value) isSidebarOpen.value = false
      }
    },
    { 
      label: t('walls.title'), 
      icon: 'i-lucide-brick-wall', 
      to: '/walls', 
      active: route.path === '/walls',
      onSelect: () => {
        if (isSidebarHidden.value) isSidebarOpen.value = false
      }
    },
    { 
      label: t('marketplace.title'), 
      icon: 'i-lucide-shopping-basket', 
      to: '/marketplace', 
      active: route.path === '/marketplace',
      onSelect: () => {
        if (isSidebarHidden.value) isSidebarOpen.value = false
      }
    }
  ] as NavigationMenuItem[];
}

function getBottomNavigation() {
  return [
    { 
      label: t('user.title'), 
      icon: 'i-lucide-circle-user', 
      to: '/get-started', 
      active: route.path === '/get-started', 
      onSelect: () => {
        if (isSidebarHidden.value) isSidebarOpen.value = false
      }
    },
    { 
      label: t('settings.title'), 
      icon: 'i-lucide-settings', 
      to: '/settings', 
      active: route.path === '/settings',
      onSelect: () => {
        if (isSidebarHidden.value) isSidebarOpen.value = false
      }
    },
  ] as NavigationMenuItem[];
}

// Lifecycle e Tauri Listeners (Logica originale intatta)
onMounted(async () => {
  console.log(settings.value);
  setLocale(settings.value.language);
  
  listen<{ view: string }>("goto", (event) => router.push(event.payload.view));
  
  listen<[string, string]>("import_brick", (event) => importBrickModal.value?.open(event.payload));
  listen<{ brick: Brick, redirect: boolean }>("rename_brick", (event) => { renameBrickModal.value?.open(event.payload.brick, event.payload.redirect) });
  listen("new_brick", () => newBrickModal.value?.open());

  // Intercept delete_brick events from main and show global confirm modal before deleting
  listen<Brick>("delete_brick", async (event) => {
    const payload = event.payload as Brick;
    const { confirm } = useConfirmModal();
    const { deleteBrick } = useBrickActions();
    const ok = await confirm({
      title: t('modals.delete_brick_title', 'Delete brick'),
      message: t('modals.delete_brick_message', `Are you sure you want to delete '${payload.name}'?`),
      confirmLabel: t('actions.delete', 'Delete'),
      cancelLabel: t('actions.cancel', 'Cancel'),
      color: 'error'
    });

    if (ok) deleteBrick(payload);
  });
});

// Watcher Settings (Logica originale intatta)
watch(
  () => JSON.parse(JSON.stringify(settings.value)) as Settings, 
  async (newSettings, oldSettings) => {
    if (!newSettings) return;
    await invoke("save_settings", { settings: newSettings });
    setLocale(newSettings.language);
    if (oldSettings && oldSettings.taskbar.behavior !== newSettings.taskbar.behavior) {
      await invoke(newSettings.taskbar.behavior === 'hide' ? "hide_taskbar" : "show_taskbar");
    }
  }, 
  { deep: true }
);
</script>