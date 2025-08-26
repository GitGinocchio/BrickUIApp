<template>
  <n-config-provider :theme="theme">
    <n-layout style="height: 100vh" has-sider>
      <n-layout-sider
        width="220"
        :collapsed-width="64"
        :collapsed="collapsed"
        collapse-mode="width"
        @collapse="collapsed = true"
        @expand="collapsed = false"
        @mouseenter="handleMouseEnter"
        @mouseleave="handleMouseLeave"
      > 
        <n-menu
          v-model:value="option"
          :collapsed="collapsed"
          :options="menuOptions"
          @update:value="onMenuSelect"
        />
      </n-layout-sider>

      <n-layout-content :native-scrollbar="false">
        <router-view />
      </n-layout-content>
    </n-layout>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, h, onMounted, watch, provide, computed } from 'vue'
import { useRouter } from 'vue-router'
import { LayoutDashboardIcon, SettingsIcon, StoreIcon } from 'lucide-vue-next'
import {
  NConfigProvider,
  NLayout,
  NLayoutSider,
  NLayoutContent,
  NMenu,
  darkTheme,
  lightTheme
} from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import { Brick } from 'interfaces/brick'
import { Settings } from 'interfaces/settings'
import { useI18n } from 'vue-i18n'
import { emit, emitTo } from '@tauri-apps/api/event'
const { t, locale } = useI18n();

const systemIsDark = ref(false)


function updateSystemTheme() {
  systemIsDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
}

const theme = computed(() => {
  switch (settings.value.theme) {
    case "light":
      return lightTheme
    case "dark":
      return darkTheme
    case "system":
      updateSystemTheme();
      return systemIsDark.value ? darkTheme : lightTheme
  }
});

const router = useRouter();
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

const option = ref('/widgets')
const menuOptions = computed(() => [
  { label: t('bricks-manager'), key: '/widgets',     icon: () => h(LayoutDashboardIcon) },
  { label: t('settings'),       key: '/settings',    icon: () => h(SettingsIcon) },
  { label: t('marketplace'),    key: '/marketplace', icon: () => h(StoreIcon) }
])

function onMenuSelect(key: string) {
  clearTimeout(hoverTimer)
  router.push(key)
}

const settings = ref<Settings>({
  language: "en",
  theme: "light",
  notifications: { position: "top-right" },
  taskbar: { behavior: "windows-default" },
});

provide("settings", settings);

onMounted(async () => {
  const media = window.matchMedia('(prefers-color-scheme: dark)')
  media.addEventListener('change', updateSystemTheme)

  // Inizialmente la lista dei brick e' vuota, in questo modo la carichiamo una volta sola all'interno dell'app
  // in modo anche da poter prendere eventuali errori nel caricamento e mostrarli all'utente
  // in caso di brick formattati male
  await invoke<Brick[]>("load_bricks", {});
  settings.value = await invoke<Settings>("get_settings");
});

watch(settings,async (newSettings) => {
  await invoke("save_settings", { settings: newSettings });
  await emit("settings-update", { ...newSettings });
  settings.value = newSettings;
}, { deep: true });

watch(
  () => settings.value.language,
  (newLang) => {
    locale.value = newLang 
    localStorage.setItem('language', newLang)
  }
)
</script>


<style scoped>
:deep(.n-menu-item .n-menu-item-content){
  padding-left: 18px !important;
  margin-right: 2px;
}

:deep(.n-menu) {
  padding-top: 6px;
}

</style>