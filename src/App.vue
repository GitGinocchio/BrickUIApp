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
import { ref, h, onMounted, watch, computed, Ref, inject } from 'vue'
import { useRouter } from 'vue-router'
import { LayoutDashboardIcon, SettingsIcon, StoreIcon } from 'lucide-vue-next'
import {
  NConfigProvider,
  NLayout,
  NLayoutSider,
  NLayoutContent,
  NMenu,
  GlobalTheme,
} from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import { Brick } from 'interfaces/brick'
import { Settings } from 'interfaces/settings'
import { useI18n } from 'vue-i18n'
import { emit } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
const { t, locale } = useI18n();

const currentWindow = getCurrentWindow()
const settings = inject("settings") as Ref<Settings>;
const theme = inject("theme") as Ref<GlobalTheme>;

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

onMounted(async () => {
  // Inizialmente la lista dei brick e' vuota, in questo modo la carichiamo una volta sola all'interno dell'app
  // in modo anche da poter prendere eventuali errori nel caricamento e mostrarli all'utente
  // in caso di brick formattati male
  await invoke<Brick[]>("load_bricks", {});

  watch(() => settings.value.taskbar.behavior, async (value) => {
    if (value === 'hide' || value === 'hide-and-fill') {
      await invoke("hide_taskbar", { keepTaskbarSpace : value !== 'hide-and-fill'});
    }
    else {
      await invoke("show_taskbar");
    }
  });

  await currentWindow.hide();
  await currentWindow.show();
});

watch(settings, async (newSettings) => {
  await invoke("save_settings", { settings: newSettings });
  await emit("changed-settings", newSettings);
  locale.value = newSettings.language;
}, { deep: true });
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