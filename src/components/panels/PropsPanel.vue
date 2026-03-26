<template>
  <n-collapse :trigger-areas="['main', 'arrow']" v-model:expanded-names="expanded">
    <div class="props-header">
      <div class="display-modes">
        <n-button-group name="layout">
          <n-button
            :type="layout == 'adaptive' ? 'primary' : 'tertiary'"
            @click="layout = 'adaptive'"
            size="small"
          >
            <LayoutDashboard :size="18" />
          </n-button>
          <n-button
            round
            :type="layout == 'list' ? 'primary' : 'tertiary'"
            @click="layout = 'list'"
            size="small"
          >
            <LayoutList :size="18" />
          </n-button>
          <n-button 
            round
            :type="layout == 'grid' ? 'primary' : 'tertiary'"
            @click="layout = 'grid'"
            size="small"
          >
            <LayoutGrid :size="18" />
          </n-button>
        </n-button-group>
      </div>
      <div class="props-actions">
        <n-button circle size="small" @click="onNewProp" v-if="editMode"><Plus :size="20" /></n-button>
        <n-switch size="large" @update:value="onToggleEditMode">
          <template #checked-icon>
            <Pencil :size="14" />
          </template>
          <template #unchecked-icon>
            <Eye :size="14" />
          </template>
        </n-switch>
      </div>
    </div>

    <div v-if="newProp" class="prop">
      <n-collapse-item class="prop-options" :name="newProp.prop_name">
        <template #header>
          <n-input
            v-model:value="newProp.prop_name"
            @click.stop
            @keydown.enter.stop="onSavePropName(newProp)"
            size="small" 
            class="prop-input-name" 
            placeholder="Type name and press enter..."
          />
        </template>
        <template #header-extra>
          <div class="prop-actions">
            <n-button @click.stop="onSavePropName(newProp)" text circle>
              <Save :size="16" />
            </n-button>
            <n-button circle text @click.stop="onRemoveProp(newProp)">
              <Trash2 :size="16" />
            </n-button>
          </div>
        </template>
        <BrickPropOptions 
          class="prop-options-content" 
          :prop="newProp" 
          :all-props="brick.props"
          :show-alert-on-type-change="false"
        />
      </n-collapse-item>
      <div class="prop-value-container">
        <BrickProp :prop="newProp" />
      </div>
      <n-divider />
    </div>
    
    <sortable
      v-if="brick.props.length > 0"
      :options="options"
      :list="brick.props"
      item-key="prop_name"
      tag="div"
      class="container"
      :class="layout"
      @update="onPropsOrderUpdate"
    >
      <template #item="{element, index, key}">
        <div :class="[editMode && !expanded.includes(element.prop_name) ? 'draggable' : '']" class="prop" :key="key">
          <n-collapse-item :class="[editMode ? 'prop-options' : 'prop-description']" :disabled="!editMode && !element.description" :name="element.prop_name">
            <template #header>
              <span v-if="!editingPropNames.has(element.prop_name)" class="prop-name">{{ element.prop_name }}</span>
              <n-input 
                v-else
                @update:value="(value: string) => editingPropNames.set(element.prop_name, value)"
                :default-value="element.prop_name"
                :allow-input="(value) => !value || /^[a-zA-Z]+$/.test(value)"
                @click.stop
                @keydown.enter.stop.prevent="onSavePropName(element)"
                size="small" 
                class="prop-input-name" 
                placeholder="Type a new prop name" 
              />
            </template>
            <template v-if="editMode" #header-extra>
              <div class="prop-actions">
                <n-button circle text v-if="editingPropNames.has(element.prop_name) && element.prop_name != newProp?.prop_name" @click.stop="togglePropNameEditMode(element)"><CircleX :size="16" /></n-button>
                <n-button @click.stop="editingPropNames.has(element.prop_name) ? onSavePropName(element) : togglePropNameEditMode(element)" text circle size="medium">
                  <Pencil v-if="!editingPropNames.has(element.prop_name)" :size="16" />
                  <Save v-else :size="16" />
                </n-button>
                <n-button circle text @click.stop="onDuplicateProp(element)" v-if="element.prop_name != newProp?.prop_name"><Copy :size="16" /></n-button>
                <n-button circle text @click.stop="onRemoveProp(element)"><Trash2 :size="16" /></n-button>
              </div>
            </template>
            <template v-if="!editMode" #arrow>
              <CircleX v-if="element.description && expanded.includes(element.prop_name)" :size="18" />
              <CircleQuestionMark v-else-if="element.description" :size="18" />
              <div v-else></div>
            </template>
            
            <BrickPropOptions 
              class="prop-options-content"
              v-if="editMode" 
              :prop="element" 
              :all-props="brick.props"
              @update:prop="(prop) => onUpdateProp(prop)"
            />
            <div v-else-if="element.description" class="prop-description-content">{{ element.description }}</div>
          </n-collapse-item>
          <span v-if="expanded.includes(element.prop_name) && editMode" class="prop-value">Value:</span>
          <div class="prop-value-container">
            <BrickProp :prop="element" @update:prop="(prop) => onUpdateProp(prop)" />
          </div>
          <n-divider class="prop-separator" v-if="index < brick.props.length - 1" />
        </div>
      </template>
    </sortable>
    <div v-else>
      <p>This brick has no props!</p>
    </div>
  </n-collapse>
</template>

<script setup lang="ts">
import { NCollapse, NCollapseItem, NDivider, NButton, NInput, NSwitch, NButtonGroup, useNotification, useDialog } from 'naive-ui';
import { Pencil, Trash2, Copy, Save, CircleX, LayoutList, LayoutGrid, LayoutDashboard, Plus, Eye, CircleQuestionMark } from 'lucide-vue-next';
import type { Brick, Prop } from '~/interfaces/brick';
import { type SortableEvent, type SortableOptions } from "sortablejs";
import { Sortable } from 'sortablejs-vue3';
import { invoke } from '@tauri-apps/api/core';
import { emitTo } from '@tauri-apps/api/event';

const newProp = ref<Prop|null>(null);
const editMode = ref<boolean>(false);
const editingPropNames = ref<Map<String, String|null>>(new Map());
const expanded = ref<string[]>([]);
const layout = ref<'grid' | 'list' | 'adaptive'>('adaptive');

const dialog = useDialog()
const notification = useNotification();
const brick = defineModel<Brick>("brick", { required: true });

const options = computed<SortableOptions>(() => {
  return {
    draggable: ".draggable",
    ghostClass: "ghost",
    dragClass: "drag",
    filter: `
      input, 
      textarea, 
      .n-input, 
      .n-button, 
      .n-select, 
      .n-color-picker, 
      .prop-value, 
      .prop-value-container, 
      .prop-options-content`,
    preventOnFilter: false,
    animation: 150,
    group: "props",
    scroll: true,
    fallbackClass: "clone",
    forceFallback: true,
    fallbackOnBody: true,
    bubbleScroll: true,
  };
});

const debouncedSaveBrick = debounce(async () => await invoke("save_brick", { brick: brick.value }), 3000);
const debouncedOverlayUpdateBrick = debounce(async (prop: Prop) => await emitTo("overlay", "update-brick", { name: brick.value.name, prop: prop }), 50);

function onPropsOrderUpdate(event: SortableEvent) {
  console.log(event);
  const { oldIndex, newIndex } = event;

  if (oldIndex !== undefined && newIndex !== undefined && oldIndex !== newIndex) {
    const newList = [...brick.value.props];
    
    const [movedItem] = newList.splice(oldIndex, 1);
    newList.splice(newIndex, 0, movedItem);
    
    brick.value.props = newList;
    
    debouncedSaveBrick();
  }
}

function onToggleEditMode() {
  editingPropNames.value.clear();
  newProp.value = null;
  editMode.value = !editMode.value;
}

function togglePropNameEditMode(prop: Prop) {
  if (editingPropNames.value.has(prop.prop_name)) {
    editingPropNames.value.delete(prop.prop_name)
  } else {
    editingPropNames.value.set(prop.prop_name, prop.prop_name)
  }
}

function onSavePropName(prop: Prop) {
  const isCreating = prop.prop_name === newProp.value?.prop_name;
  
  const newPropName = isCreating 
    ? newProp.value?.prop_name.trim()
    : editingPropNames.value.get(prop.prop_name)?.toString().trim();

  const namesAlreadyTaken = brick.value.props.map((p) => p.prop_name);

  // Validazione
  if (!newPropName || !/^[a-zA-Z]+$/.test(newPropName)) {
    notification.error({
      title: 'Invalid Prop Name',
      description: 'Prop names must contain only letters and cannot be empty.'
    });
    return;
  }

  if (newPropName === prop.prop_name && !isCreating) {
    togglePropNameEditMode(prop);
    return;
  }

  if (namesAlreadyTaken.includes(newPropName)) {
    notification.error({
      title: 'Invalid Prop Name',
      description: 'This name is already taken.'
    });
    return;
  }

  if (isCreating) {
    brick.value.props = [newProp.value, ...brick.value.props];
    newProp.value = null;
  } else {
    if (expanded.value.includes(prop.prop_name)) {
      expanded.value = [...expanded.value.filter((v) => v !== prop.prop_name), newPropName];
    }
    prop.prop_name = newPropName;
    togglePropNameEditMode(prop);
  }

  debouncedSaveBrick();
}

async function onNewProp() {
  newProp.value = {
    prop_type: 'String',
    prop_name: ''
  };
}

function onDuplicateProp(prop: Prop) {
  if (brick.value.props.findIndex((other) => other.prop_name == prop.prop_name + 'Copy') != -1) {
    notification.error({
      title: `Could not duplicate this prop`,
      description: `Prop '${prop.prop_name}Copy' already exists`
    })
    return;
  }

  const currentIndex = brick.value.props.findIndex((other) => other.prop_name == prop.prop_name);

  const newProp: Prop = {
    ...prop,
    prop_name: prop.prop_name + 'Copy'
  };

  brick.value.props = [
    ...brick.value.props.slice(0, currentIndex + 1), // Prendi tutto fino a currentIndex incluso
    newProp,                                         // Inserisci il nuovo elemento
    ...brick.value.props.slice(currentIndex + 1)     // Prendi il resto
  ];

  debouncedSaveBrick();
}

function onRemoveProp(prop: Prop) {
  dialog.warning({
    title: 'Confirm deletion',
    content: `Are you sure you want to remove prop "${prop.prop_name}"?\nThe action is irreversible.`,
    positiveText: 'Delete',
    negativeText: 'Cancel',
    onPositiveClick: () => {
      if (prop.prop_name == newProp.value?.prop_name) {
        newProp.value = null;
        return;
      }

      brick.value.props = brick.value.props.filter(
        (other) => other.prop_name !== prop.prop_name
      );

      debouncedSaveBrick();
    },
  })
}

function onUpdateProp(prop: Prop) {
  const currentIndex = brick.value.props.findIndex((other) => other.prop_name == prop.prop_name);

  brick.value.props = [
    ...brick.value.props.slice(0, currentIndex),
    prop,
    ...brick.value.props.slice(currentIndex + 1)
  ];

  debouncedOverlayUpdateBrick(prop);
  debouncedSaveBrick();
}

function warnIfUnsavedChanges(onPositiveClick: () => void) {
  if (!newProp.value) return;
  dialog.warning({
    title: 'Unsaved Changes',
    content: 'You have a new prop in progress. Do you want to discard it and leave?',
    positiveText: 'Discard and Leave',
    negativeText: 'Stay Here',
    onPositiveClick: onPositiveClick,
  });
}

onBeforeRouteLeave((_to, _from, next) => {
  if (newProp.value) {
    warnIfUnsavedChanges(() => next())
  } else {
    next(); // Nessuna modifica, vai pure
  }
});
</script>

<style scoped>
.prop {
  display: flex;
  flex-direction: column;
  margin-top: 0.15rem;
  padding-top: 0.15rem;
  gap: 0.5rem;
  cursor: default;
}

.draggable {
  cursor: grab;
}

.draggable:active {
  cursor: grabbing;
}

.props-header {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-left: 0.25rem;
  margin-right: 0.25rem;
  margin-bottom: 1rem;
}

.props-actions {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 0.50rem;
}

.display-modes {
  display: flex;
  gap: 0.25rem;
}

.prop-name {
  height: 1.75rem;
  font-size: 1rem;
  font-weight: 650;
  color: var(--n-text-color)
}

.prop-input-name {
  width: 35vw;
}

.prop-value {
  cursor: auto;
  display: none;
  font-size: 0.95rem;
}

.prop-options {
  padding-top: 0;
}

.prop-description {
  padding-top: 0;
}

:deep(.prop-description .n-collapse-item-arrow) {
  transform: none !important;
}

.prop-actions {
  flex-direction: row;
  gap: 0.5rem;
  display: none;
  margin-right: 0.25rem;
}

.prop:hover .prop-actions {
  display: flex;
}

.prop-description-content {
  margin-left: 0.5rem;
}

.prop-options-content {
  position: relative; /* Necessario per posizionare il pseudo-elemento */
  width: inherit;
  padding-top: 0.25rem;
  margin-left: 0.5rem;
  padding-left: 0.5rem;
  margin-right: 1rem;
  border-left: none; /* Rimuoviamo il bordo originale */
  cursor: auto;
}

.prop-options-content::before {
  content: "";
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 1px;
  /* Creiamo l'effetto tratteggiato con un gradiente ripetuto */
  background-image: linear-gradient(
    to bottom, 
    rgba(255, 255, 255, 0.15) 0%, 
    rgba(255, 255, 255, 0.15) 50%, 
    transparent 50%
  );
  background-size: 1px 8px; /* Altezza del trattino (4px trattino, 4px spazio) */
  
  /* Maschera per lo sfumato finale */
  mask-image: linear-gradient(to bottom, black 70%, transparent 100%);
  -webkit-mask-image: linear-gradient(to bottom, black 70%, transparent 100%);
}

:deep(.n-divider:not(.n-divider--vertical)) {
  margin-bottom: 0.25rem;
  margin-top: 0.25rem;
}

:deep(.n-collapse-item__content-inner) {
  padding-top: 0.25rem !important;
}

:deep(.n-collapse-item.n-collapse-item--disabled .n-collapse-item__header) {
  cursor: default !important;
}

:deep(.draggable .n-collapse-item__header-main) {
  cursor: grab !important;
}

:deep(.draggable .n-collapse-item__header-main:active) {
  cursor: grabbing !important;
}

.ghost {
  opacity: 0.5;
  border: 1px dashed #cccccccc;
  cursor: grabbing !important;
}

.clone {
  opacity: 0.1;
  cursor: grabbing !important;
  color: white;
  z-index: 100;
}

:deep(.n-collapse-item-arrow) {
  display: none;
}

:deep(.n-collapse-item__header-main) {
  display: flex;
  align-items: center;
}

.clone > .prop-separator {
  opacity: 0;
}

.drag {
  cursor: grabbing !important;
  opacity: 0.1;
}

@media (max-width: 800px) {
  .prop-value {
    display: flex;
  }
}

.container > * {
  transition: all 0.3s ease-in-out;
}

.container.grid {
  display: grid;
  gap: 0.5rem;
  grid-template-columns: repeat(auto-fill, minmax(30rem, 1fr));
}

.container.list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

/* Forza lo stesso layout verticale se .grid è attivo */
:deep(.container.grid .n-form-item.n-form-item--top-labelled, 
.container.adaptive .n-form-item.n-form-item--top-labelled) {
  grid-template-columns: 100%;
  grid-template-areas: "label" "blank" "feedback";
}

@media (max-width: 999px) {
  .container.adaptive {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
}

@media (min-width: 1000px) {
  .container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(30rem, 1fr));
    gap: 0.5rem
  }
}
</style>

<style>
.n-color-picker-swatches {
  display: flex;
  align-items: flex-end;
  min-height: 22px;
}
/* Vedere se mantenere o migliorare questo */
.n-color-picker-swatch {
  transition: box-shadow 0.2s ease, transform 0.2s ease;
}

.n-color-picker-swatch:focus {
  outline: none !important;
  
  /* Un bordo nero solido ma più spesso */
  border: 2px solid black !important;
  
  /* Un'ombra netta e scura per staccarlo decisamente dal fondo */
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  
  border-radius: 4px;
  
  /* Solleviamo lo swatch */
  transform: translateY(-2px);
}
</style>