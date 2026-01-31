<template>
  <n-config-provider :theme="theme">
    <n-layout style="height: 100vh" has-sider>
      <n-layout-sider
        class="sider"
        width="220"
        :collapsed-width="64"
        :collapsed="collapsed"
        collapse-mode="width"
        @collapse="collapsed = true"
        @expand="collapsed = false"
        @mouseenter="handleMouseEnter"
        @mouseleave="handleMouseLeave"
        :style="{ order: settings.sidebar.position === 'right' ? 1 : 0 }"
      > 
        <n-menu
          :value="activeMenuKey"
          :collapsed="collapsed"
          :options="menuOptions"
          @update:value="onMenuSelect"
        />
        <n-menu
          :value="activeMenuKey"
          :collapsed="collapsed"
          :options="bottomMenuOptions"
          @update:value="onMenuSelect"
        />
      </n-layout-sider>

      <n-layout-content :native-scrollbar="false">
        <router-view />
      </n-layout-content>
    </n-layout>
    <SystemTray :v-if="settings.systemtray" />
    
    <GenericModal
      :message="addBrickModalMessage"
      title="Import brick"
      type="warning"
      v-model:show="showAddBrickModal"
    >
      <template #extra>
        <n-alert title="Warning" type="warning">
          Careful when importing bricks from untrusted sources
        </n-alert>
        <n-alert title="Info" type="info">
          You can verify the brick by yourself by changing the file extension
          from <code>.brick</code> to <code>.zip</code> and look to the code inside
        </n-alert>
      </template>
      <template #actions>
        <n-button type="warning" :secondary="true" @click="showAddBrickModal = false">Cancel</n-button>
        <n-button type="warning" @click="onOpenBrickConfirm">Import</n-button>
      </template>
    </GenericModal>

    <GenericModal
      title="Brick already imported"
      type="error"
      message="There is already a brick with this name"
      v-model:show="showAlreadyImportedModal"
    >
      <template #actions>
        <n-button type="error" @click="showAlreadyImportedModal = false">Ok</n-button>
      </template>
    </GenericModal>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, h, onMounted, watch, computed, Ref, inject } from 'vue'
import SystemTray from './components/SystemTray.vue';
import { useRoute, useRouter } from 'vue-router'
import { SettingsIcon, StoreIcon, CircleUser, Cuboid, LayoutDashboard } from 'lucide-vue-next'
import {
  NConfigProvider,
  NLayout,
  NAlert,
  NButton,
  NLayoutSider,
  NLayoutContent,
  NMenu,
  GlobalTheme,
} from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import { Brick } from 'interfaces/brick'
import { Settings } from 'interfaces/settings'
import { useI18n } from 'vue-i18n'
import { emit, listen } from '@tauri-apps/api/event'
import GenericModal from './components/modals/GenericModal.vue';
const { t, locale } = useI18n();

const settings = inject("settings") as Ref<Settings>;
const theme = inject("theme") as Ref<GlobalTheme>;
const bricks = inject("bricks") as Ref<Brick[]>;

const router = useRouter();
const collapsed = ref(true);
let hoverTimer: number | null = null

const route = useRoute();
const activeMenuKey = computed(() => {
  if (route.path.startsWith('/user')) {
    return '/get-started'
  }
  return route.path
});

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

onMounted(async () => {
  // Inizialmente la lista dei brick e' vuota, in questo modo la carichiamo una volta sola all'interno dell'app
  // in modo anche da poter prendere eventuali errori nel caricamento e mostrarli all'utente
  // in caso di brick formattati male
  bricks.value = await invoke<Brick[]>("load_bricks", {});

  locale.value = settings.value.language;

  watch(() => settings.value.taskbar.behavior, async (value) => {
    await invoke(value === 'hide' ? "hide_taskbar" : "show_taskbar");
  });
});

const showAddBrickModal = ref<boolean>(false);
const showAlreadyImportedModal = ref<boolean>(false);
const addBrickModalMessage = ref<string>("");
const brickToImport = ref<[string, string]>(null);
listen<[string, string]>("open_brick", (event) => {
  const brick = event.payload;

  if (bricks.value.some((list_brick) => list_brick.name === brick[1])) {
    showAlreadyImportedModal.value = true;
    return;
  }

  addBrickModalMessage.value = `Are you sure you want to import brick "${brick[1]}"`
  showAddBrickModal.value = true;
  brickToImport.value = brick;
});

listen<{ "view": string }>("goto", (event) => {
  router.push(event.payload.view);
});

async function onOpenBrickConfirm() {
  showAddBrickModal.value = false;
  await invoke("unpack_brick", { brickPath: brickToImport.value[0], brickName: brickToImport.value[1]});
  bricks.value = await invoke<Brick[]>("load_bricks", {});
  router.push(`/brick/${brickToImport.value[1]}`)
}

watch(settings, async (newSettings) => {
  await invoke("save_settings", { settings: newSettings });
  await emit("changed-settings", newSettings);
  locale.value = newSettings.language;
}, { deep: true });
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