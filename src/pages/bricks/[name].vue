<template>
  <div class="container" v-if="brick" @scroll="handleScroll">
    <n-image
      class="banner"
      :class="{ loaded: isBannerLoaded }"
      object-fit="cover"
      width="100%"
      :show-toolbar="false"
      :preview-disabled="true"
      :src="bannerUrl"
      @load="onBannerLoad"
      lazy
    >
      <template #placeholder>
        <img class="banner" loading="lazy" @load="onBannerLoad" :src="defaultBannerUrl" />
      </template>
    </n-image>

    <Header :sections="sections" class="header" :style="{ '--scroll-opacity': headerOpacity }">
      <template #actions>
        <div class="actions">
          <n-switch
            style="margin: 0"
            size="large"
            v-model:value="brick.enabled"
            @update:value="toggleBrick(brick)"
          />
          <n-dropdown :options="options" trigger="click" placement="bottom-end" @select="onBrickActionSelected">
            <n-button circle secondary size="small">
              <template #icon>
                <EllipsisVertical />
              </template>
            </n-button>
          </n-dropdown>
        </div>
      </template>
    </Header>
    <div class="content">
      <n-tabs type="line" v-model:value="activeTab" animated>
        <n-tab-pane name="info">
          <template #tab>
            <div class="tab-header">
              <Info :size="16" />
              <span>Info</span>
            </div>
          </template>
          <div class="tab-content">
            <div class="tab-actions">
              <n-button @click="onEditDescription" text circle size="medium">
                <Pencil v-if="!descriptionEditMode" :size="16" />
                <PencilOff v-else :size="16" />
              </n-button>
            </div>
            <n-input
              v-if="descriptionEditMode"
              v-model:value="brick.description"
              type="textarea"
              placeholder="Type your brick's description"
            />
            <p
              v-else-if="renderedDescription.length > 0"
              v-html="renderedDescription"
              class="description"
            ></p>
            <p v-else>This brick has no description</p>
          </div>
        </n-tab-pane>

        <n-tab-pane name="props">
          <template #tab>
            <div class="tab-header">
              <Cog :size="16" />
              <span>Props</span>
            </div>
          </template>
          <template #suffix>
          </template>
          <PropsPanel v-model:brick="brick" />
        </n-tab-pane>

        <n-tab-pane name="emits">
          <template #tab>
            <div class="tab-header">
              <Wifi :size="16" />
              <span>Emits</span>
            </div>
          </template>
          <div class="tab-content">
            <p>Work in progress :P</p>
          </div>
        </n-tab-pane>

        <n-tab-pane name="permissions">
          <template #tab>
            <div class="tab-header">
              <Shield :size="16" />
              <span>Permissions</span>
            </div>
          </template>
          <div class="tab-content">
            <p>Work in progress :P</p>
          </div>
        </n-tab-pane>
      </n-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NTabs, NTabPane, NButton, NSwitch, NInput, NImage, NDropdown, type DropdownOption } from "naive-ui";
import { invoke } from "@tauri-apps/api/core";
import {
  Blocks,
  Cuboid,
  BadgeCheck,
  EllipsisVertical,
  Pencil,
  PencilOff,
  Cog,
  Wifi,
  Shield,
  Info,
  Trash2,
  Copy,
} from "lucide-vue-next";
import { emitTo } from "@tauri-apps/api/event";
import MarkdownIt from "markdown-it";

import { appDataDir, sanitizePath } from "#utils/path";
import type { Brick } from "#interfaces/brick";
import PropsPanel from "#components/panels/PropsPanel.vue";
import { useBrickActions } from "~/composables/useBrickActions";

const { t } = useI18n();
const { bricks, theme } = useAppState();
const { renameBrick, deleteBrick, duplicateBrick,  toggleBrick } = useBrickActions(bricks, theme);
const route = useRoute();
const router = useRouter();
const activeTab = ref<string>("info");

const brickName = computed(() => route.params.name as string);

const brick = ref<Brick>(null);
const sections = computed(() => {
  return [
    { icon: Blocks, label: t("bricks"), onclick: () => router.push("/bricks") },
    { icon: iconUrl.value, defaultIcon: Cuboid, label: brickName.value },
  ];
});

const options = computed<DropdownOption[]>(() => [
  {
    label: 'Rename',
    key: 'rename',
    icon: () => h(Pencil)
  },
  {
    label: 'Duplicate',
    key: 'duplicate',
    icon: () => h(Copy)
  },
  {
    label: () => h('div', { style: { 'color' : 'red'} }, { default: () => 'Delete' }),
    icon: () => h(Trash2, { color: 'red' }), 
    key: 'delete'
  },
  {
    type: 'divider',
    key: 'd1'
  },
  {
    key: 'version-info',
    icon: () => h(BadgeCheck, { style: { color: '#2080f0', opacity: 1 } }),
    label: () => h(
      'span', 
      { 
        style: { 
          color: '#2080f0', 
          opacity: 1,
          fontWeight: '500'
        } 
      }, 
      `v. ${brick.value?.version.join(".")}`
    ),
    disabled: true
  },
]);

async function onBrickActionSelected(option: string) {
  switch (option) {
    case 'rename':
      await renameBrick(brick.value, (newName: string) => router.push(`/bricks/${newName}`));
      break;
    case 'duplicate':
      duplicateBrick(brick.value, () => router.push(`/bricks/${brick.value.name}Copy`));
      break;
    case 'delete':
      await deleteBrick(brick.value, () => router.push("/bricks"));
      break;
    default:
      console.warn(`azione brick non riconosciuta: ${option}`);
  }
}

/* Description */

const md = new MarkdownIt();
const renderedDescription = computed(() => md.render(brick.value.description));
const descriptionEditMode = ref<boolean>(false);

async function onEditDescription(_event?: Event) {
  descriptionEditMode.value = !descriptionEditMode.value;

  if (descriptionEditMode.value && activeTab.value !== "description") {
    activeTab.value = "description";
  }

  if (!descriptionEditMode.value) {
    await invoke("save_brick", { brick: brick.value });
    await emitTo("main", "update_bricks");
  }
}

const headerOpacity = ref(0);
function handleScroll(e: Event) {
  const target = e.target as HTMLElement;
  const scrollTop = target.scrollTop;
  
  const maxScroll = 150; 
  headerOpacity.value = Math.min(scrollTop / maxScroll, 1);
}

const isBannerLoaded = ref<boolean>(false);
const bannerUrl = ref<string | null>(null);
const defaultBannerUrl = new URL("../../assets/images/banner-brick-iso.svg", import.meta.url).href;
const iconUrl = ref<string | null>(null);

function onBannerLoad() {
  setTimeout(() => { isBannerLoaded.value = true; }, 15);
}

onMounted(async () => {
  brick.value = await invoke("get_brick_by_name", { name: brickName.value });

  await nextTick();
  
  if (brick.value?.banner) {
    bannerUrl.value = await sanitizePath(brick.value.banner, { root: `${appDataDir}/bricks/${brick.value.name ?? brickName.value}` });
  }
  if (brick.value?.icon) {
    iconUrl.value = await sanitizePath(brick.value.icon, { root: `${appDataDir}/bricks/${brick.value.name ?? brickName.value}` });
  }
});
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  overflow-y: hidden;
  gap: 1rem;
}

:deep(.n-switch) {
  margin: 0;
}

.banner {
  width: 100%;
  height: 35vh;
  object-fit: cover;
  object-position: center;
  opacity: 0.5;
  z-index: 0;

  filter: blur(var(--blur, 20px));
  transition: filter 0.5s ease;
}

.banner.loaded {
  --blur: 0px;
}

.header {
  position: fixed;
  top: 0;
  width: calc(100% - 7rem);
  z-index: 1000 !important;
  margin: 0 !important;

  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;

  padding-left: 1.5rem;
  padding-right: 1.5rem;

  padding-top: clamp(1rem, 2rem - (var(--layout-scroll-top, 0px) / 10), 2rem);
  padding-bottom: clamp(1rem, 2rem - (var(--layout-scroll-top, 0px) / 10), 2rem);

  background-color: rgba(24, 24, 28, clamp(0, var(--layout-scroll-top, 0px) / 150, 0.9));
  backdrop-filter: blur(clamp(0px, var(--layout-scroll-top, 0px) / 15, 12px));
  -webkit-backdrop-filter: blur(clamp(0px, var(--layout-scroll-top, 0px) / 15, 12px));

  transition: 
    background-color 0.1s linear, 
    backdrop-filter 0.1s linear,
    padding 0.1s ease-out;

  border-bottom: 1px solid rgba(255, 255, 255, clamp(0, var(--layout-scroll-top, 0px) / 300, 0.1));
}

.header * {
  margin: 0;
}

.header .actions {
  display: flex; 
  align-items: center; 
  gap: 0.5rem
}

.header,
.content {
  z-index: 1;
  margin-bottom: 1rem;
}

.content {
  margin-left: 1rem;
  margin-right: 1rem;
}

:deep(.n-card-header) {
  padding-bottom: 0.5rem !important;
}

:deep(.n-form-item-feedback-wrapper) {
  min-height: 0;
}

.tab-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.tab-content {
  position: relative;
}

.tab-actions {
  position: absolute;
  top: 0;
  right: 0;
  z-index: 10;
}

.movable {
  display: flex;
  flex-direction: row;
}

.arrows {
  cursor: pointer;
  font-size: 18px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.add-prop {
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin-top: 0.5rem;
  gap: 0.5rem;
  width: 100%;
}
</style>
