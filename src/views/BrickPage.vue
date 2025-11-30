<template>
  <div class="container" v-if="brick">
    <div
      class="thumbnail"
      :style="{ backgroundImage: `url(${backgroundUrl})` }"
    ></div>
    <Header :sections="sections" class="header">
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

        <n-tab-pane name="props" class="properties-container">
          <template #tab>
            <div class="tab-header">
              <Cog :size="16" />
              <span>Props</span>
            </div>
          </template>
          <div class="tab-content">
            <div class="tab-actions">
              <n-button @click="onEditPropMode" text circle size="medium">
                <Pencil v-if="!propsEditMode" :size="16" />
                <PencilOff v-else :size="16" />
              </n-button>
            </div>
            <div
              class="movable"
              v-for="(prop, index) in brick.props"
              :key="prop.prop_name"
            >
              <span v-if="propsEditMode" class="arrows">
                <ChevronUp
                  v-if="index !== 0"
                  @click="onMovePropUp(prop)"
                  :size="16"
                />
                <ChevronDown
                  v-if="index < brick.props.length - 1"
                  @click="onMovePropDown(prop)"
                  :size="16"
                />
              </span>
              <BrickProp
                :prop="prop"
                :edit-mode="propsEditMode"
                @update:prop="onPropValueChanged"
                @edit:prop="onEditProp"
                @delete:prop="onDeleteProp"
              />
            </div>
            <div v-if="brick.props.length == 0">
              <p>This brick has no props!</p>
            </div>
            <n-button
              size="small"
              v-if="propsEditMode"
              class="add-prop"
              @click="onNewProp"
              ><CirclePlus :size="16" />Add</n-button
            >
          </div>
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

    <!-- Confirm Delete Modal -->
    <GenericModal
      v-model:show="deleteModalShow"
      :message="deleteModalMessage"
      :title="deleteModalTitle"
      negative="Cancel"
      positive="Delete"
      type="warning"
      @confirm="deleteModalOnConfirm"
      @decline="deleteModalOnDecline"
    />

    <!-- New/Edit Prop Modal -->
    <PropModal
      v-model:show="propModalShow"
      v-model:prop="targetProp"
      v-model:initialProp="initialProp"
      :editMode="propModalEditMode"
      :props="brick.props"
      @finished="onPropEditFinished"
    />
  </div>
</template>

<script setup lang="ts">
import { NTabs, NTabPane, NButton, NSwitch, NInput, NTag } from "naive-ui";
import { computed, onMounted, ref, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";
import {
  Blocks,
  Cuboid,
  BadgeCheck,
  Text,
  Pencil,
  PencilOff,
  Cog,
  CirclePlus,
  ChevronDown,
  ChevronUp,
  Wifi,
  Shield,
} from "lucide-vue-next";
import GenericModal from "../components/modals/GenericModal.vue";
import PropModal from "../components/modals/PropModal.vue";
import BrickProp from "../components/BrickProp.vue";
import Header from "../components/Header.vue";
import { Brick, Prop } from "../interfaces/brick";
import { useRouter } from "vue-router";
import MarkdownIt from "markdown-it";
import { emit, emitTo } from "@tauri-apps/api/event";
import { appDataDir, sanitizePath } from "../utils/path";

const { t } = useI18n();
const router = useRouter();
const activeTab = ref<string>("description");
let updateTimeout: ReturnType<typeof setTimeout> | null = null;
let saveTimeout: ReturnType<typeof setTimeout> | null = null;

const props = defineProps({
  name: {
    type: String,
    required: true,
  },
});

const brick = ref<Brick>(null);
const sections = computed(() => {
  return [
    { icon: Blocks, label: t("bricks"), onclick: () => router.push("/bricks") },
    { icon: brick.value?.icon, defaultIcon: Cuboid, label: props.name },
  ];
});

const defaultBanner = new URL(
  "../assets/images/banner-brick-iso.svg",
  import.meta.url
).href;
const backgroundUrl = ref(defaultBanner);

watchEffect(async () => {
  if (!brick.value?.banner) {
    backgroundUrl.value = defaultBanner;
    return;
  }

  // Precarica l'immagine
  const img = new Image();
  img.src = await sanitizePath(brick.value?.banner, { root: `${appDataDir}/bricks/${brick.value.name}` });

  img.onload = () => {
    backgroundUrl.value = img.src;
  };
  img.onerror = () => {
    console.error(`Error loading banner image for brick: ${brick.value?.name}`);
    backgroundUrl.value = defaultBanner;
  };
});

onMounted(async () => {
  brick.value = await invoke("get_brick_by_name", { name: props.name });
});

/* Delete Modal */
const deleteModalTitle = ref<string>("");
const deleteModalMessage = ref<string>("");
const deleteModalShow = ref<boolean>(false);
const deleteModalOnConfirm = ref<() => void | null>();
const deleteModalOnDecline = ref<() => void | null>();
const propToDelete = ref<Prop | null>(null);

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

/* Props */
const propsEditMode = ref<boolean>(false);
const propModalShow = ref<boolean>(false);
const propModalEditMode = ref<boolean>(false);
const targetProp = ref<Prop | null>(null);
const initialProp = ref<Prop | null>(null);

function onNewProp() {
  targetProp.value = { prop_type: "String", prop_name: "", description: null };
  initialProp.value = null;
  propModalEditMode.value = false;
  propModalShow.value = true;
}

function onEditProp(prop: Prop) {
  targetProp.value = JSON.parse(JSON.stringify(prop));
  initialProp.value = JSON.parse(JSON.stringify(prop));
  propModalEditMode.value = true;
  propModalShow.value = true;
}

async function onDeleteProp(prop: Prop) {
  propToDelete.value = prop;

  deleteModalTitle.value = "Confirm deletion";
  deleteModalMessage.value = `Are you sure you want to delete "${prop.prop_name}"?`;
  deleteModalOnConfirm.value = deleteProp;
  deleteModalShow.value = true;
}

async function deleteProp() {
  const index = brick.value.props.findIndex(
    (p) => p.prop_name === propToDelete.value.prop_name
  );
  brick.value.props.splice(index, 1);

  await invoke("save_brick", { brick: brick.value });
  emit("changed");
}

async function onPropValueChanged(prop: Prop) {
  console.log(`Prop update: ${prop}`);
  const index = brick.value.props.findIndex(
    (p) => p.prop_name === prop.prop_name
  );
  if (index !== -1) brick.value.props[index] = prop;
  else brick.value.props.push(prop);

  if (updateTimeout) clearTimeout(updateTimeout);
  updateTimeout = setTimeout(async () => {
    await emitTo("overlay", "update-brick", {
      name: brick.value.name,
      prop: prop,
    });
    updateTimeout = null;
  }, 50);

  // Debounce del salvataggio
  if (saveTimeout) clearTimeout(saveTimeout);
  saveTimeout = setTimeout(async () => {
    await invoke("save_brick", { brick: brick.value });
    saveTimeout = null;
  }, 3000);
}

async function onPropEditFinished(before: Prop, clone?: boolean) {
  if (propModalEditMode.value) {
    const index = brick.value.props.findIndex(
      (p) => p.prop_name === before.prop_name
    );

    if (index !== -1 && clone) {
      targetProp.value.prop_name = `${before.prop_name}-copy`;
      brick.value.props.splice(index + 1, 0, targetProp.value);
    } else if (index !== -1) {
      brick.value.props[index] = targetProp.value;
    } else {
      brick.value.props.push(targetProp.value);
    }
  } else {
    brick.value.props.push(targetProp.value);
  }

  await invoke("save_brick", { brick: brick.value });
  emit("changed");
}

/* Brick actions */
async function onEditPropMode(_event?: Event) {
  propsEditMode.value = !propsEditMode.value;

  if (propsEditMode.value && activeTab.value !== "props") {
    activeTab.value = "props";
  }
}

async function onToggle() {
  await emitTo("overlay", "toggle-brick", { brick: brick.value });
  await invoke("save_brick", { brick: brick.value });
  emit("changed");
}

async function onOpenBrick() {
  await invoke("open_brick", { brickName: brick.value.name });
}

async function onMovePropUp(prop: Prop) {
  const index = brick.value.props.findIndex(
    (p) => p.prop_name === prop.prop_name
  );
  if (index > 0) {
    // scambia con l'elemento precedente
    const tmp = brick.value.props[index - 1];
    brick.value.props[index - 1] = brick.value.props[index];
    brick.value.props[index] = tmp;
  }

  await invoke("save_brick", { brick: brick.value });
}

async function onMovePropDown(prop: Prop) {
  const index = brick.value.props.findIndex(
    (p) => p.prop_name === prop.prop_name
  );
  if (index >= 0 && index < brick.value.props.length - 1) {
    const tmp = brick.value.props[index + 1];
    brick.value.props[index + 1] = brick.value.props[index];
    brick.value.props[index] = tmp;
  }

  await invoke("save_brick", { brick: brick });
}

async function deleteBrick() {
  await invoke("delete_brick", { brick: brick.value });
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

.thumbnail {
  width: 100%;
  height: 35vh;
  background-size: cover;
  background-position: center;
  opacity: 0.3;
  z-index: 0;
}

.header {
  padding-top: 1rem;
  padding-left: 1rem;
  position: absolute;
  display: flex;
  width: calc(100% - 3rem);
  top: 0;
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
  padding: 1rem 0;
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
