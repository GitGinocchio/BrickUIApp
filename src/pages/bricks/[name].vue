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
        <div style="display: flex; align-items: center; gap: 1rem">
          <n-switch
            style="margin: 0"
            size="large"
            v-model:value="brick.enabled"
            @update:value="onToggle"
          />
          <n-tag round :bordered="false" type="info">
            v. {{ brick.version.join(".") }}
            <template #icon>
              <BadgeCheck />
            </template>
          </n-tag>
        </div>
      </template>
    </Header>
    <div class="content">
      <n-tabs type="line" v-model:value="activeTab" animated>
        <n-tab-pane name="description">
          <template #tab>
            <div class="tab-header">
              <Text :size="16" />
              <span>Description</span>
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

        <n-tab-pane name="props" class="props-container">
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

        <n-tab-pane name="emits" class="emits-container">
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

        <n-tab-pane name="permissions" class="permissions-container">
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
import { NTabs, NTabPane, NButton, NSwitch, NInput, NTag, NImage } from "naive-ui";
import { invoke } from "@tauri-apps/api/core";
import {
  Blocks,
  Cuboid,
  BadgeCheck,
  Text,
  Pencil,
  PencilOff,
  Cog,
  Wifi,
  Shield,
} from "lucide-vue-next";
import { emit, emitTo } from "@tauri-apps/api/event";
import MarkdownIt from "markdown-it";

import { appDataDir, sanitizePath } from "#utils/path";
import type { Brick } from "#interfaces/brick";
import PropsPanel from "#components/panels/PropsPanel.vue";

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const activeTab = ref<string>("description");

const brickName = computed(() => route.params.name as string);

const brick = ref<Brick>(null);
const sections = computed(() => {
  return [
    { icon: Blocks, label: t("bricks"), onclick: () => router.push("/bricks") },
    { icon: iconUrl.value, defaultIcon: Cuboid, label: brickName.value },
  ];
});

const headerOpacity = ref(0);

function handleScroll(e: Event) {
  const target = e.target as HTMLElement;
  const scrollTop = target.scrollTop;
  
  const maxScroll = 150; 
  headerOpacity.value = Math.min(scrollTop / maxScroll, 1);

  console.log(headerOpacity.value);
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
  
  if (brick.value.banner) {
    bannerUrl.value = await sanitizePath(brick.value.banner, { root: `${appDataDir}/bricks/${brick.value.name ?? brickName.value}` });
  }
  if (brick.value.icon) {
    iconUrl.value = await sanitizePath(brick.value.icon, { root: `${appDataDir}/bricks/${brick.value.name ?? brickName.value}` });
  }
});

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
    emit("changed");
  }
}

async function onToggle() {
  await emitTo("overlay", "toggle-brick", { brick: brick.value });
  await invoke("save_brick", { brick: brick.value });
  emit("changed");
}
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
