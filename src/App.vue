<template>
  <n-config-provider>
    <n-layout style="height: 100vh" has-sider>
      <n-layout-sider
        width="220"
        :collapsed-width="64"
        :collapsed="collapsed"
        collapse-mode="width"
        @collapse="collapsed = true"
        @expand="collapsed = false"
        @mouseenter="collapsed = false"
        @mouseleave="collapsed = true"
      >
        <n-menu
          v-model:value="option"
          :collapsed="collapsed"
          :options="menuOptions"
          @update:value="onMenuSelect"
        />
      </n-layout-sider>

      <n-layout-content content-style="padding: 16px;">
        <router-view />
      </n-layout-content>
    </n-layout>
  </n-config-provider>
</template>

<style scoped>

:deep(.n-menu-item .n-menu-item-content){
  padding-left: 18px !important;
  margin-right: 2px;
}

</style>

<script setup lang="ts">
import { ref, h, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { LayoutDashboardIcon, SettingsIcon, StoreIcon } from 'lucide-vue-next'
import {
  NConfigProvider,
  NLayout,
  NLayoutSider,
  NLayoutContent,
  NMenu
} from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import { Brick } from 'interfaces/brick'

const router = useRouter();
const collapsed = ref(true);

const option = ref('/widgets')
const menuOptions = [
  { label: 'Bricks Manager', key: '/widgets',      icon: () => h(LayoutDashboardIcon) },
  { label: 'Settings',       key: '/settings',     icon: () => h(SettingsIcon)},
  { label: 'Marketplace',    key: '/marketplace',  icon: () => h(StoreIcon)}
]

function onMenuSelect(key: string) {
  router.push(key)
}

onMounted(async () => {
  // Inizialmente la lista dei brick e' vuota, in questo modo la carichiamo una volta sola all'interno dell'app
  // in modo anche da poter prendere eventuali errori nel caricamento e mostrarli all'utente
  // in caso di brick formattati male
  await invoke<Brick[]>("load_bricks", {});
});
</script>
