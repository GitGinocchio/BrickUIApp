<template>
  <div class="container" v-if="brick">
    <div class="thumbnail">
    </div>
    <Header :sections="sections" class="header">
      <template #actions>
        <div style="display: flex; align-items:center; gap: 1rem;">
          <n-switch style="margin: 0;" size="large" v-model:value="brick.enabled" @update:value="onToggle" />
          <n-tag round :bordered="false" type="info">
            v. {{ brick.version.join('.') }}
            <template #icon>
              <BadgeCheck />
            </template>
          </n-tag>
        </div>
      </template>
    </Header>
    <div class="content">
      <n-collapse accordion v-model:expanded-names="expanded">
        <n-collapse-item title="Description" name="description">
          <template #arrow>
            <Text />
          </template>
          <template #header>
            <div class="collapse-item-header">
              <div>Description</div>
              <n-button @click.stop="onEditDescription" text circle size="medium">
                <Pencil v-if="!descriptionEditMode" :size="16"/>
                <PencilOff v-else :size="16"/>
              </n-button>
            </div>
          </template>
          <n-input
            v-if="descriptionEditMode"
            v-model:value="brick.description"
            type="textarea"
            placeholder="Type your brick's description"
          />
          <p v-else-if="renderedDescription.length > 0" v-html="renderedDescription" class="description"></p>
          <p v-else>This brick has no description</p>
        </n-collapse-item>
        <n-collapse-item title="Props" name="props" class="properties-container">
          <template #arrow>
            <Cog #arrow />
          </template>
          <template #header>
            <div class="collapse-item-header">
              <div>Props</div>
              <n-button @click.stop="onEditPropMode" text circle size="medium">
                <Pencil v-if="!propsEditMode" :size="16"/>
                <PencilOff v-else :size="16"/>
              </n-button>
            </div>
          </template>
          <div class="movable" v-for="(prop, index) in brick.props" :key="prop.prop_name">
            <span v-if="propsEditMode" class="arrows">
              <ChevronUp v-if="index !== 0" @click="onMovePropUp(prop)" :size="16" />
              <ChevronDown v-if="index < brick.props.length - 1" @click="onMovePropDown(prop)" :size="16" />
            </span>
            <BrickProp 
              :prop="prop"
              :edit-mode="propsEditMode" 
              @update:prop="onPropValueChanged" 
              @edit:prop="onEditProp"
              @delete:prop="onDeleteProp"
            />
          </div>
          <div v-if="brick.props.length == 0"><p>This brick has no props!</p></div>
          <n-button size="small" v-if="propsEditMode" class="add-prop" @click="onNewProp"><CirclePlus :size="16"/>Add</n-button>
        </n-collapse-item>
        <n-collapse-item title="Emits" name="emits" class="emits-container">
          <template #arrow>
            <Wifi />
          </template>
          <!--
          <div v-for="prop in brick.props" :key="prop.prop_name">

          </div>
          -->
          <p>Work in progress :P</p>
        </n-collapse-item>
        <n-collapse-item title="Permissions" name="permissions" class="emits-container">
          <template #arrow>
            <Shield />
          </template>
          <!--
          <div v-for="prop in brick.props" :key="prop.prop_name">

          </div>
          -->
          <p>Work in progress :P</p>
        </n-collapse-item>
      </n-collapse>
    </div>
    
    <!-- Confirm Delete Modal -->
    <ConfirmModal
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
import { NCollapse, NCollapseItem, NButton, NSwitch, NInput } from 'naive-ui';
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import { Blocks, Cuboid, BadgeCheck, Text, Pencil, PencilOff, Cog, CirclePlus, ChevronDown, ChevronUp, Wifi, Shield } from 'lucide-vue-next';
import ConfirmModal from '../components/modals/ConfirmModal.vue';
import PropModal from '../components/modals/PropModal.vue';
import BrickProp from '../components/BrickProp.vue';
import Header from '../components/Header.vue';
import { Brick, Prop } from '../interfaces/brick';
import { useRouter } from 'vue-router';
import MarkdownIt from 'markdown-it';
import { emit, emitTo } from '@tauri-apps/api/event';

const { t } = useI18n();
const router = useRouter();
const expanded = ref<Array<string>>([]);
let updateTimeout: ReturnType<typeof setTimeout> | null = null;
let saveTimeout: ReturnType<typeof setTimeout> | null = null;

const props = defineProps({
  name: {
    type: String,
    required: true
  }
});
const brick = ref<Brick>(null);
const sections = computed(() => {
  return [
    { icon: Blocks, label: t('bricks'), onclick: () => router.push('/bricks') },
    { icon: brick.value?.icon, defaultIcon: Cuboid, label: props.name }
  ]
});

onMounted(async () => {
  brick.value = await invoke("get_brick_by_name", { name: props.name });
});

/* Delete Modal */
const deleteModalTitle = ref<string>('');
const deleteModalMessage = ref<string>('');
const deleteModalShow = ref<boolean>(false);
const deleteModalOnConfirm = ref<() => void|null>();
const deleteModalOnDecline = ref<() => void|null>();
const propToDelete = ref<Prop|null>(null);

/* Description */

const md = new MarkdownIt();
const renderedDescription = computed(() => md.render(brick.value.description));
const descriptionEditMode = ref<boolean>(false);
async function onEditDescription(_event: Event) {
  descriptionEditMode.value = !descriptionEditMode.value;

  if (descriptionEditMode.value && !expanded.value.includes("description")) {
    expanded.value.length = 0;
    expanded.value.push("description");
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
const targetProp = ref<Prop|null>(null);
const initialProp = ref<Prop|null>(null);

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
  const index = brick.value.props.findIndex(p => p.prop_name === propToDelete.value.prop_name);
  brick.value.props.splice(index, 1);

  await invoke("save_brick", { brick: brick.value });
  emit("changed");
}

async function onPropValueChanged(prop: Prop) {
  console.log(`Prop update: ${prop}`);
  const index = brick.value.props.findIndex(p => p.prop_name === prop.prop_name);
  if (index !== -1) brick.value.props[index] = prop;
  else brick.value.props.push(prop);

  if (updateTimeout) clearTimeout(updateTimeout);
  updateTimeout = setTimeout(async () => {
    await emitTo("overlay", "update-brick", { 
      name: brick.value.name, 
      prop_name: prop.prop_name, 
      prop_value: prop.value 
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
    const index = brick.value.props.findIndex(p => p.prop_name === before.prop_name);

    if (index !== -1 && clone) {
      targetProp.value.prop_name = `${before.prop_name}-copy`;
      brick.value.props.splice(index + 1, 0, targetProp.value);
    }
    else if (index !== -1) {
      brick.value.props[index] = targetProp.value;
    }
    else {
      brick.value.props.push(targetProp.value);
    }
  } 
  else {
    brick.value.props.push(targetProp.value);
  }

  await invoke("save_brick", { brick: brick.value });
  emit("changed");
}

/* Brick actions */
async function onEditPropMode(_event: Event) {
  propsEditMode.value = !propsEditMode.value
  
  if (propsEditMode.value && !expanded.value.includes("props")) {
    expanded.value.length = 0;
    expanded.value.push("props");
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
  const index = brick.value.props.findIndex(p => p.prop_name === prop.prop_name)
  if (index > 0) {
    // scambia con l'elemento precedente
    const tmp = brick.value.props[index - 1]
    brick.value.props[index - 1] = brick.value.props[index]
    brick.value.props[index] = tmp
  }

  await invoke("save_brick", { brick: brick.value });
}

async function onMovePropDown(prop: Prop) {
  const index = brick.value.props.findIndex(p => p.prop_name === prop.prop_name)
  if (index >= 0 && index < brick.value.props.length - 1) {
    const tmp = brick.value.props[index + 1]
    brick.value.props[index + 1] = brick.value.props[index]
    brick.value.props[index] = tmp
  }

  await invoke("save_brick", { brick: brick.value });
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
  background-image: url('https://picsum.photos/1920/1080');
  background-size: cover;
  background-position: center;
  opacity: 0.2;
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

:deep(.n-collapse .n-collapse-item .n-collapse-item__content-wrapper .n-collapse-item__content-inner) {
  padding-top: 0;
}

:deep(.n-form-item-feedback-wrapper) {
  min-height: 0;
}

:deep(.n-input-number) {
  width: 100%;
}

:deep(
  .n-collapse 
  .n-collapse-item.n-collapse-item--active 
  .n-collapse-item__header.n-collapse-item__header--active 
  .n-collapse-item-arrow) {
  transform: none !important;
}

.collapse-item-header {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  width: 100%;
}

.collapse-item-header div {
  gap: 0.25rem;
  display: flex;
  flex-direction: row;
  justify-content: center;
}

.movable {
  display: flex;
  flex-direction: row;
}

.arrows {
  cursor: pointer;
  /* margin-right: 8px; */
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