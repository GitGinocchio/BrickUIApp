<template>
  <n-card class="card" :segmented="true" hoverable>
    <template #header>
      <n-space justify="space-between" align="center" class="w-full">
        <div v-if="brick.icon" class="brick-icon">
          <img :src="brick.icon" alt="Brick Icon" />
        </div>
        <div class="brick-title">
          <strong>{{ brick.name }}</strong><template v-if="brick.author"> - {{ brick.author }}</template>
          <n-button text circle @click="onOpenBrick"><ExternalLink :size="18" /></n-button>
          <span class="brick-version">(v {{ brick.version.join('.') }})</span>
          <span class="brick-version" v-if="brick.license">License: {{ brick.license }}</span>
        </div>
        <div class="brick-controls">
          <n-switch v-model:value="brick.enabled" @update:value="onToggle" />
          <n-dropdown :options="brickOptions" :animated="true" @select="handleBrickAction">
            <n-button text circle><MoreVertical/></n-button>
          </n-dropdown>
        </div>
      </n-space>
      <n-space>
        <n-tag round v-for="tag in brick.tags" :key="tag" type="info">{{ tag }}</n-tag>
      </n-space>
    </template>

    <!-- Info generali -->
    <!--
    <div v-if="brick.dependencies.length">
      <strong>Dependencies:</strong> {{ brick.dependencies.join(', ') }}
    </div>
    -->
 
    <n-collapse default-expanded-names="2" accordion>
      <n-collapse-item title="Description" name="1">
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
        <div v-else v-html="renderedDescription" class="description"></div>
      </n-collapse-item>
      <n-collapse-item title="Props" name="2" class="properties-container">
        <template #arrow>
          <Cog #arrow />
        </template>
        <template #header>
          <div class="collapse-item-header">
            <div>Props</div>
            <n-button @click.stop="propsEditMode = !propsEditMode" text circle size="medium">
              <Pencil v-if="!propsEditMode" :size="16"/>
              <PencilOff v-else :size="16"/>
            </n-button>
          </div>
        </template>
        <div class="movable" v-for="prop in brick.props" :key="prop.prop_name">
          <span v-if="propsEditMode" class="arrows cursor-move mr-2">
            <ChevronUp @click="onMovePropUp(prop)" :size="16" />
            <ChevronDown @click="onMovePropDown(prop)" :size="16" />
          </span>
          <BrickProp 
            :prop="prop" 
            :edit-mode="propsEditMode" 
            @update:prop="onPropValueChanged" 
            @edit:prop="onEditProp"
            @delete:prop="onDeleteProp"
          />
        </div>
        <n-button size="small" v-if="propsEditMode" class="add-prop" @click="onNewProp"><CirclePlus :size="16"/>Add</n-button>
      </n-collapse-item>
      <n-collapse-item title="Emits" name="3" class="emits-container">
        <template #arrow>
          <Wifi />
        </template>
        <!--
        <div v-for="prop in brick.props" :key="prop.prop_name">

        </div>
        -->
        <p>Work in progress :P</p>
      </n-collapse-item>
    </n-collapse>
  </n-card>

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
  
</template>

<script setup lang="ts">
import { 
  NSwitch, NSpace, NTag, NCard,
  NCollapse, NCollapseItem, NButton,
  NDropdown, NInput
} from 'naive-ui'
import { 
  Copy, MoreVertical, Pencil, PencilOff, Trash2, ExternalLink,
  Text, Cog, Wifi, CirclePlus,
  ChevronDown, ChevronUp
} from 'lucide-vue-next';
import { Brick, Prop } from '../interfaces/brick'
import ConfirmModal from './modals/ConfirmModal.vue';
import PropModal from './modals/PropModal.vue';
import BrickProp from './BrickProp.vue';
import { emitTo } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { computed, ref, h } from 'vue';
import MarkdownIt from 'markdown-it';

const md = new MarkdownIt();
const renderedDescription = computed(() => md.render(props.brick.description));

const props = defineProps<{ 
  brick: Brick 
}>();

const emit = defineEmits<{
  (e: "changed"): void
  (e: "edit", brick: Brick): void
}>();

// Delete Modal variables
const deleteModalTitle = ref<string>('');
const deleteModalMessage = ref<string>('');
const deleteModalShow = ref<boolean>(false);
const deleteModalOnConfirm = ref<() => void|null>();
const deleteModalOnDecline = ref<() => void|null>();
const propToDelete = ref<Prop|null>(null);

// Add/Edit Modal variables
const propModalShow = ref<boolean>(false);
const propModalEditMode = ref<boolean>(false);
const targetProp = ref<Prop|null>(null);
const initialProp = ref<Prop|null>(null);

const propsEditMode = ref<boolean>(false);
const descriptionEditMode = ref<boolean>(false);
const brickOptions = ref([
  { label: 'Edit', key: 'edit', icon: () => h(Pencil) },
  { label: 'Delete', key: 'delete', type: 'error', icon: () => h(Trash2) },
  { label: 'Duplicate', key: 'duplicate', icon: () => h(Copy) },
]);

function onNewProp() {
  targetProp.value = { prop_type: "Any", prop_name: "", description: null };
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
  const index = props.brick.props.findIndex(p => p.prop_name === propToDelete.value.prop_name);
  props.brick.props.splice(index, 1);

  await invoke("save_brick", { brick: props.brick });
  emit("changed");
}

async function onPropValueChanged(prop: Prop) {
  console.log(`Prop update: ${prop}`);
  const index = props.brick.props.findIndex(p => p.prop_name === prop.prop_name);
  if (index !== -1) props.brick.props[index] = prop;
  else props.brick.props.push(prop);

  await emitTo("window", "update_prop", { 
    brick_name: props.brick.name, 
    prop_name: prop.prop_name, 
    prop_value: prop.value 
  });

  await invoke("save_brick", { brick: props.brick });
}

async function onPropEditFinished(before: Prop, clone?: boolean) {
  if (propModalEditMode.value) {
    const index = props.brick.props.findIndex(p => p.prop_name === before.prop_name);

    if (index !== -1 && clone) {
      targetProp.value.prop_name = `${before.prop_name}-copy`;
      props.brick.props.splice(index + 1, 0, targetProp.value);
    }
    else if (index !== -1) {
      props.brick.props[index] = targetProp.value;
    }
    else {
      props.brick.props.push(targetProp.value);
    }
  } 
  else {
    props.brick.props.push(targetProp.value);
  }

  await invoke("save_brick", { brick: props.brick });
  emit("changed");
}

/* Brick actions */

async function onEditDescription(_event: Event) {
  descriptionEditMode.value = !descriptionEditMode.value;

  if (!descriptionEditMode.value) {
    await invoke("save_brick", { brick: props.brick });
    emit("changed");
  }
}

async function handleBrickAction(action: string) {
  switch (action) {
    case "delete":
      deleteModalTitle.value = "Confirm deletion";
      deleteModalMessage.value = `Are you sure you want to delete "${props.brick.name}"?`;
      deleteModalOnConfirm.value = deleteBrick;
      deleteModalShow.value = true;
      break;
    case "edit":
      emit("edit", props.brick)
      break;
    case "duplicate":
      await invoke("duplicate_brick", { brick: props.brick});
      emit("changed");
      break;
    default:
      console.error(`azione non riconosciuta: ${action}`);
  }
}

async function onToggle() {
  await emitTo("window", "toggle_brick", { brick: props.brick });
  await invoke("save_brick", { brick: props.brick });
}

async function onOpenBrick() {
  await invoke("open_brick", { brickName: props.brick.name });
}

async function onMovePropUp(prop: Prop) {
  const index = props.brick.props.findIndex(p => p.prop_name === prop.prop_name)
  if (index > 0) {
    // scambia con l'elemento precedente
    const tmp = props.brick.props[index - 1]
    props.brick.props[index - 1] = props.brick.props[index]
    props.brick.props[index] = tmp
  }

  await invoke("save_brick", { brick: props.brick });
}

async function onMovePropDown(prop: Prop) {
  const index = props.brick.props.findIndex(p => p.prop_name === prop.prop_name)
  if (index >= 0 && index < props.brick.props.length - 1) {
    const tmp = props.brick.props[index + 1]
    props.brick.props[index + 1] = props.brick.props[index]
    props.brick.props[index] = tmp
  }

  await invoke("save_brick", { brick: props.brick });
}

async function deleteBrick() {
  await invoke("delete_brick", { brick: props.brick });
  emit("changed");
}
</script>

<style scoped>
::v-deep(.n-card-header) {
  padding-bottom: 0.5rem !important;
}

::v-deep(.n-collapse .n-collapse-item .n-collapse-item__content-wrapper .n-collapse-item__content-inner) {
  padding-top: 0;
}

::v-deep(.n-form-item-feedback-wrapper) {
  min-height: 0;
}

::v-deep(.n-input-number) {
  width: 100%;
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

.properties-container { 
  font-weight: bold;
}

.brick-icon {
  text-align: center;
  margin-bottom: 8px;
}
.brick-icon img {
  max-width: 48px;
  max-height: 48px;
}

.brick-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.brick-version {
  color: #888;
  font-size: 0.9rem;
}

.brick-controls {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 0.5rem;
}

.add-prop {
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin-top: 0.5rem;
  gap: 0.5rem;
  width: 100%;
}

.description {
  font-size: 1rem; 
  font-weight: normal;
  margin-top: 0rem;
  margin-bottom: 0.5rem;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.props-section {
  padding-top: 0.0rem;
}

.movable {
  display: flex;
  flex-direction: row;
}

.arrows {
  cursor: pointer;
  margin-right: 8px;
  font-size: 18px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}
</style>
