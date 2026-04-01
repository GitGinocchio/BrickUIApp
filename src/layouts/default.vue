<template>
  <NConfigProvider :theme="theme">
    <NLayout style="height: 100vh" has-sider>
      <NLayoutSider
        class="sider"
        :width="220"
        collapse-mode="width"
        :collapsed-width="64"
        :collapsed="collapsed"
        @mouseenter="handleMouseEnter"
        @mouseleave="handleMouseLeave"
        :style="{ order: settings.sidebar.position === 'right' ? 1 : 0 }"
      > 
        <n-menu :value="activeMenuKey" :collapsed="collapsed" :options="menuOptions" @update:value="onMenuSelect" />
        <n-menu :value="activeMenuKey" :collapsed="collapsed" :options="bottomMenuOptions" @update:value="onMenuSelect" />
      </NLayoutSider>

      <NLayoutContent :native-scrollbar="false" @scroll="(event) => handleScroll(event)">
          <NDialogProvider>
            <NNotificationProvider :theme="theme.Notification" placement="bottom-right">
              <slot />
            </NNotificationProvider>
          </NDialogProvider>
      </NLayoutContent>
      
      <SystemTray v-if="settings?.systemtray" />
    </NLayout>
  </NConfigProvider>
</template>

<script setup lang="ts">
import "#assets/css/default.css";

import { NLayoutSider, NNotificationProvider, NDialogProvider, NLayoutContent, NLayout, NMenu, NConfigProvider, NSpace, NAlert } from 'naive-ui';
import { CircleUser, Cuboid, LayoutDashboard, SettingsIcon, StoreIcon } from 'lucide-vue-next';
import { useAppState } from '~/composables/useAppState';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Settings } from "~/interfaces/settings";
import { createDiscreteApi } from 'naive-ui';
import { useBrickActions } from "~/composables/useBrickActions";
const { settings, theme, bricks } = useAppState();

const { dialog, notification } = createDiscreteApi(['dialog', 'notification'], {
  configProviderProps: {
    theme: theme.value
  },
  notificationProviderProps: {
    themeOverrides: theme.value.Notification,
    placement: 'bottom-right'
  }
});

const { openBrick } = useBrickActions(bricks, theme, dialog, notification);
const { t, setLocale } = useI18n();
const router = useRouter();
const route = useRoute();


const activeMenuKey = computed(() => {
  if (route.path.startsWith('/user')) {
    return '/get-started'
  }
  return route.path
});

const collapsed = ref(true);
let hoverTimer: number | null = null

function handleMouseEnter() {
  // Avvia il timer (es. 500ms prima di aprire)
  hoverTimer = window.setTimeout(() => {
    collapsed.value = false
  }, 250)
}

function handleMouseLeave() {
  // Cancella il timer se l’utente si muove via prima
  if (hoverTimer) {
    clearTimeout(hoverTimer)
    hoverTimer = null
  }
  // richiudi subito al mouseleave
  collapsed.value = true
}

function handleScroll(e: Event) {
  const target = e.target as HTMLElement;
  // Impostiamo la variabile sul documento in modo che sia accessibile ovunque
  document.documentElement.style.setProperty('--layout-scroll-top', `${target.scrollTop}px`);
}

const menuOptions = computed(() => [
  { label: t('bricks'),           key: '/bricks',       icon: () => h(Cuboid) },
  { label: t('walls'),            key: '/walls',        icon: () => h(LayoutDashboard) },
  { label: t('marketplace'),      key: '/marketplace',  icon: () => h(StoreIcon) }
]);

const bottomMenuOptions = computed(() => [
  { label: t('user'),             key: '/get-started',  icon: () => h(CircleUser) },
  { label: t('settings'),         key: '/settings',     icon: () => h(SettingsIcon) },
]);

function onMenuSelect(key: string) {
  clearTimeout(hoverTimer)
  router.push(key)
}

listen<{ view: string }>("goto", (event) => navigateTo(event.payload.view));
listen<[string, string]>("open_brick", (event) => openBrick(event.payload));

onMounted(() => setLocale(settings.value.language));

// Watch per salvare le impostazioni
watch(
  () => JSON.parse(JSON.stringify(settings.value)) as Settings, 
  async (settings, old) => {
    if (!settings) return;
    await invoke("save_settings", { settings: settings });
    setLocale(settings.language);

    if (old.taskbar.behavior != settings.taskbar.behavior) {
      await invoke(settings.taskbar.behavior == 'hide' ? "hide_taskbar" : "show_taskbar");
    }
  }, 
  { deep: true }
);
</script>

<style scoped>
:deep(.n-layout-sider-scroll-container) {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding-bottom: 0.5rem;
}

:deep(.n-menu-item .n-menu-item-content){
  padding-left: 18px !important;
  margin-right: 2px;
}

:deep(.n-menu) {
  padding-top: 6px;
}
</style>