<template>
  <n-config-provider>
    <n-layout style="height: 100vh" has-sider>
      <n-layout-sider
        width="220"
        :collapsed-width="64"
        :collapsed="collapsed"
        show-trigger
        collapse-mode="width"
        @collapse="collapsed = true"
        @expand="collapsed = false"
      >
        <n-menu
          v-model:value="menu"
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
import { ref, h } from 'vue'
import { useRouter } from 'vue-router'
import { LayoutDashboardIcon, SettingsIcon, StoreIcon } from 'lucide-vue-next'
import {
  NConfigProvider,
  NLayout,
  NLayoutSider,
  NLayoutContent,
  NMenu
} from 'naive-ui'

const router = useRouter()
const menu = ref('/widgets')
const collapsed = ref(false)

const menuOptions = [
  { label: 'Widget Manager', key: '/widgets',      icon: () => h(LayoutDashboardIcon) },
  { label: 'Settings',       key: '/settings',     icon: () => h(SettingsIcon)},
  { label: 'Marketplace',    key: '/marketplace',  icon: () => h(StoreIcon)}
]

function onMenuSelect(key: string) {
  router.push(key)
}
</script>
