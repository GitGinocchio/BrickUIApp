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
        <div v-for="prop in brick.props" :key="prop.prop_name">
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

  <!-- Delete modal -->
  <n-modal
    v-model:show="showDeleteBrickModal"
    :auto-focus="true"
    type="warning"
    preset="dialog"
    title="Confirm Deletion"
    negative-text="Cancel"
    positive-text="Delete"
    @positive-click="deleteBrick"
  >
    Are you sure you want to delete "{{ brick.name }}"?
  </n-modal>

  <n-modal
    v-model:show="showDeletePropModal"
    :auto-focus="true"
    type="warning"
    preset="dialog"
    title="Confirm Deletion"
    negative-text="Cancel"
    positive-text="Delete"
    @positive-click="deleteProp"
  >
    Are you sure you want to delete "{{ propToDelete.prop_name }}"?
  </n-modal>

  <!-- New/Edit Prop modal -->
  <n-modal
    v-model:show="showPropModal"
    :auto-focus="true"
    preset="dialog"
    :title="propEditMode ? 'Edit Prop' : 'New Prop'"
    :draggable="true"
  >
    <n-form>
      <n-form-item 
        label="Prop name"
        :validation-status="feedback ? 'error' : undefined"
        :show-feedback="feedback ? true : false"
        :feedback="feedback"
      >
        <n-input
          :value="currentProp.prop_name"
          placeholder="Type the prop's name"
          @update:value="onPropNameInput"
        />
      </n-form-item>
      <n-form-item 
        label="Prop Description"
        :show-feedback="false"
        :feedback="null"
      >
        <n-input
          v-model:value="currentProp.description"
          :default-value="currentProp.description"
          @update:value="(value) => currentProp.description = value"
          placeholder="Type the prop's description"
          type="textarea"
        />
      </n-form-item>
      <n-form-item 
        label="Prop type"
      >
        <n-select 
          v-model:value="currentProp.prop_type" 
          :options="propTypes"
          @update:value="onNewPropTypeSelected"
          :render-label="renderLabel"
        />
      </n-form-item>
      
      <n-form-item v-if="optionsInputField" label="Options">
        <component :is="optionsInputField" />
      </n-form-item>
      <n-form-item v-if="defaultInputField" label="Default value">
        <component :is="defaultInputField" />
      </n-form-item>
      <n-form-item v-if="minInputField" :label="['Int', 'Float'].includes(currentProp.prop_type) ? 'Input the minimum allowed number' : 'Input the minimum number of values'">
        <component :is="minInputField" />
      </n-form-item>
      <n-form-item v-if="maxInputField" :label="['Int', 'Float'].includes(currentProp.prop_type) ? 'Input the maximum allowed number' : 'Input the maximum number of values'">
        <component :is="maxInputField" />
      </n-form-item>
    </n-form>
    <template #action>
      <n-space justify="end">
        <n-button @click="showPropModal = !showPropModal">Cancel</n-button>
        <n-button
          type="primary"
          :disabled="propEditMode ? deepEqual(startProp, currentProp) || feedback != null : feedback || !currentProp.prop_name ? true : false"
          @click="onPropUpdated"
        >   
          {{ propEditMode ? 'Save' : 'Create' }}
        </n-button>
      </n-space>
    </template>
  </n-modal>
  
</template>

<script setup lang="ts">
import { 
  NSwitch, NSpace, NTag, NCard,
  NCollapse, NCollapseItem, NButton,
  NDropdown, NModal, NInput, NInputNumber,
  NForm, NFormItem, NSelect,
  NDynamicTags,
  NColorPicker
} from 'naive-ui'
import { 
  Copy, MoreVertical, Pencil, PencilOff, Trash2, ExternalLink, Type, 
  Hash, HashIcon, List, PaintBucket, ListOrdered, ToggleLeft, Asterisk, 
  Text, Cog, Wifi, CirclePlus,
  ListTodo
} from 'lucide-vue-next';
import { Brick, createProp, Prop, PropTypeValue, propTypeValues } from '../interfaces/brick'
import BrickProp from './BrickProp.vue';
import { emitTo } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { computed, ref, h } from 'vue';
import { colorStringToRGBA, deepEqual } from '../utils';
import MarkdownIt from 'markdown-it';

const md = new MarkdownIt();
const renderedDescription = computed(() => md.render(brick.description));

const { brick } = defineProps<{ brick: Brick }>()
const emit = defineEmits<{
  (e: "changed"): void
  (e: "edit", brick: Brick): void
}>();

const showDeleteBrickModal = ref<boolean>(false);
const showDeletePropModal = ref<boolean>(false);
const propToDelete = ref<Prop|null>(null);

const propsEditMode = ref<boolean>(false);
const descriptionEditMode = ref<boolean>(false);
const brickOptions = ref([
  { label: 'Edit', key: 'edit', icon: () => h(Pencil) },
  { label: 'Delete', key: 'delete', type: 'error', icon: () => h(Trash2) },
  { label: 'Duplicate', key: 'duplicate', icon: () => h(Copy) },
]);

const showPropModal = ref<boolean>(false);
const propEditMode = ref<boolean>(false);
const startProp = ref<Prop|null>(null);
const currentProp = ref<Prop>(null);
const feedback = ref<string|null>(null);

const iconsMap: Record<string, any> = {
  Any: Asterisk,
  Select: ListTodo,
  Array: List,
  
  String: Type,
  StringSelect: ListTodo,
  StringArray: List,

  Int: Hash,
  IntSelect: ListTodo,
  IntArray: ListOrdered,

  Float: Hash,
  FloatSelect: ListTodo,
  FloatArray: ListOrdered,

  Bool: ToggleLeft,
  
  Color: PaintBucket
};

const propTypes = propTypeValues.map((value) => ({
  label: value.replace(/([A-Z])/g, " $1").trim(),
  value,
  icon: () => h(iconsMap[value], { size: 16 })  // 👈 aggiunta icona
}));

function renderLabel(option) {
  return h("div", { style: "display: flex; align-items: center; gap: 6px;" }, [
    option.icon && option.icon(),
    h("span", option.label)
  ]);
}

const defaultInputField = computed(() => {
  switch (currentProp.value.prop_type) {
    case "Any":
    case "String":
      return h(
        NInput,
        {
          value: currentProp.value.default,
          clearable: true,
          "onUpdate:value": (val: string) => (currentProp.value.default = val)
        }
      );
    
    case "Int":
    case "Float":
      return h(
        NInputNumber,
        {
          value: currentProp.value.default,
          min: currentProp.value.min,
          max: currentProp.value.max,
          clearable: true,
          precision: currentProp.value.prop_type == 'Int' ? 0 : 2,
          "onUpdate:value": (val: number) => (currentProp.value.default = val)
        }
      );
    
    case "Bool":
      return h(
        NSwitch,
        {
          value: currentProp.value.default,
          clearable: true,
          "onUpdate:value": (val: boolean) => (currentProp.value.default = val)
        }
      );

    case "Select":
    case "StringSelect":
    case "FloatSelect":
    case "IntSelect":
      return h(
        NSelect,
        {
          value: currentProp.value.default,
          clearable: true,
          options: (currentProp.value.options || []).map(v => ({
            label: String(v),
            value: v
          })),
          "onUpdate:value": (val: any) => (currentProp.value.default = val)
        }
      );

    case "Array":
    case "StringArray":
    case "FloatArray":
    case "IntArray":
      return h(
        NDynamicTags,
        {
          value: currentProp.value.default ? currentProp.value.default.map((value : string|number|any) => String(value)) : [],
          clearable: true,
          "onUpdate:value": (newValue: any[]) => {
            let parsedValue;

            if (currentProp.value.prop_type === 'StringArray') {
              parsedValue = newValue.map(item => typeof item === 'string' ? item : item.value);
            } 
            else if (currentProp.value.prop_type === 'IntArray') {
              parsedValue = newValue
                .map(item => {
                  const val = typeof item === 'string'
                    ? /[a-zA-Z]/.test(item) ? NaN : parseInt(item)
                    : item.value;
                  return isNaN(val) ? null : val;
                })
                .filter(val => val !== null);
            } 
            else if (currentProp.value.prop_type === 'FloatArray') {
              parsedValue = newValue
                .map(item => {
                  const val = typeof item === 'string'
                    ? /[a-zA-Z]/.test(item) ? NaN : parseFloat(item.replace(',', '.'))
                    : item.value;
                  return isNaN(val) ? null : val;
                })
                .filter(val => val !== null);
            } 
            else {
              parsedValue = newValue;
            }
 
            // @ts-ignore
            currentProp.value.default = parsedValue;
          }
        }
      );
    
    case "Color":
      return h(
        NColorPicker, 
        {
          value: currentProp.value.default,
          clearable: true,
          "onUpdate:value": (val: string) => (currentProp.value.default = colorStringToRGBA(val))
        }
      );
    
    default:
      return null
  }
});

const optionsInputField = computed(() => {
  switch (currentProp.value.prop_type) {
    case "Select":
    case "FloatSelect":
    case "IntSelect":
    case "StringSelect":
      return h(
        NDynamicTags, 
        {
          value: currentProp.value.options.map((value) => String(value)),
          "onUpdate:value": (newValue: any[]) => {
            let parsedValue;

            if (currentProp.value.prop_type === 'StringSelect') {
              parsedValue = newValue.map(item => typeof item === 'string' ? item : item.value);
            } 
            else if (currentProp.value.prop_type === 'IntSelect') {
              parsedValue = newValue
                .map(item => {
                  const val = typeof item === 'string'
                    ? /[a-zA-Z]/.test(item) ? NaN : parseInt(item)
                    : item.value;
                  return isNaN(val) ? null : val;
                })
                .filter(val => val !== null);
            } 
            else if (currentProp.value.prop_type === 'FloatSelect') {
              parsedValue = newValue
                .map(item => {
                  const val = typeof item === 'string'
                    ? /[a-zA-Z]/.test(item) ? NaN : parseFloat(item.replace(',', '.'))
                    : item.value;
                  return isNaN(val) ? null : val;
                })
                .filter(val => val !== null);
            } 
            else {
              parsedValue = newValue;
            }
            
            // @ts-ignore
            currentProp.value.options = [...new Set(parsedValue)];

            // @ts-ignore
            if (!currentProp.value.options.includes(currentProp.value.default)) {
              currentProp.value.default = null;
            }
          }
        }
      );

    default:
      return null;
  };
});

const minInputField = computed(() => {
  switch (currentProp.value.prop_type) {
    case "Int":
    case "Float":
    case "Array":
    case "StringArray":
    case "IntArray":
    case "FloatArray":
    case "Select":
    case "StringSelect":
    case "IntSelect":
    case "FloatSelect":
      if (currentProp.value.max !== null && currentProp.value.min > currentProp.value.max) {
        currentProp.value.max = currentProp.value.min;
      }
      return h(
        NInputNumber, {
          value: currentProp.value.min,
          defaultValue: currentProp.value.min ? currentProp.value.min : 0,
          placeholder: ['Int', 'Float'].includes(currentProp.value.prop_type) ? "Input the minumum allowed number" : "Input the minumum number of values",
          clearable: true,
          precision: currentProp.value.prop_type === 'Float' ? 2 : 0,
          "onUpdate:value": (val: number) => {
            // @ts-ignore
            currentProp.value.min = val !== null ? val : 0;
            // @ts-ignore
            if (val > currentProp.value.max) {
              // @ts-ignore
              currentProp.value.max = val;
            }
          }
        }
      );
    default:
      return null;
  };
});

const maxInputField = computed(() => {
  switch (currentProp.value.prop_type) {
    case "Int":
    case "Float":
    case "Array":
    case "StringArray":
    case "IntArray":
    case "FloatArray":
    case "Select":
    case "StringSelect":
    case "IntSelect":
    case "FloatSelect":
      return h(
        NInputNumber, {
          value: currentProp.value.max,
          defaultValue: currentProp.value.max ? currentProp.value.max : null,
          placeholder: ['Int', 'Float'].includes(currentProp.value.prop_type) ? "Input the minumum allowed number" : "Input the maximum number of values",
          precision: currentProp.value.prop_type === 'Float' ? 2 : 0,
          clearable: true,
          min: currentProp.value.min ? currentProp.value.min : 1,
          "onUpdate:value": (val: number) => {
            console.log('max-value:', val);
            // @ts-ignore
            currentProp.value.max = val;
          }
        }
      );
    default:
      return null;
  };
});

function onNewProp() {
  currentProp.value = { prop_type: "Any", prop_name: "", description: null };
  startProp.value = null;
  propEditMode.value = false;
  showPropModal.value = true;
  feedback.value = null;
}

function onEditProp(prop: Prop) {
  currentProp.value = JSON.parse(JSON.stringify(prop));
  startProp.value = JSON.parse(JSON.stringify(prop));
  propEditMode.value = true;
  showPropModal.value = true;
  feedback.value = null;
}

async function onDeleteProp(prop: Prop) {
  propToDelete.value = prop;
  showDeletePropModal.value = true;
}

async function deleteProp() {
  const index = brick.props.findIndex(p => p.prop_name === propToDelete.value.prop_name);
  brick.props.splice(index, 1);

  await invoke("save_brick", { brick: brick });
  emit("changed");
}

function onNewPropTypeSelected(value: PropTypeValue) {
  currentProp.value = createProp(value, currentProp.value.prop_name, currentProp.value.description);
}

function onPropNameInput(value: string) {
  const componentNameRegex = /^[A-Za-z][A-Za-z0-9]*(?:[-_][A-Za-z0-9]+)*$/;
  const other_props = startProp.value ? brick.props.filter((p) => p.prop_name !== startProp.value.prop_name) : brick.props;

  if (!componentNameRegex.test(value)) {
    feedback.value = "Must start with a letter. Only letters, numbers, and underscores are allowed.";
  } else if (other_props.some((p) => p.prop_name === value)) {
    feedback.value = "A prop with this name already exists.";
  } else {
    feedback.value = null;
  }

  currentProp.value.prop_name = value;
}

async function onPropValueChanged(prop: Prop) {
  console.log(`Prop update: ${prop}`);
  const index = brick.props.findIndex(p => p.prop_name === prop.prop_name);
  if (index !== -1) brick.props[index] = prop;
  else brick.props.push(prop);

  await emitTo("window", "update_prop", { 
    brick_name: brick.name, 
    prop_name: prop.prop_name, 
    prop_value: prop.value 
  });

  await invoke("save_brick", { brick: brick });
}

async function onPropUpdated() {
  showPropModal.value = false;

  if (propEditMode.value) {
    const index = brick.props.findIndex(p => p.prop_name === startProp.value.prop_name);
    if (index !== -1) brick.props[index] = currentProp.value;
    else brick.props.push(currentProp.value);
  } 
  else {
    brick.props.push(currentProp.value);
  }

  await invoke("save_brick", { brick: brick });
  emit("changed");
}

/* Brick actions */

async function onEditDescription(_event: Event) {
  descriptionEditMode.value = !descriptionEditMode.value;

  if (!descriptionEditMode.value) {
    await invoke("save_brick", { brick: brick });
    emit("changed");
  }
}

async function handleBrickAction(action: string) {
  switch (action) {
    case "delete":
      showDeleteBrickModal.value = true;
      break;
    case "edit":
      emit("edit", brick)
      break;
    case "duplicate":
      await invoke("duplicate_brick", { brick: brick});
      emit("changed");
      break;
    default:
      console.error(`azione non riconosciuta: ${action}`);
  }
}

async function onToggle() {
  await emitTo("window", "toggle_brick", { brick: brick });
  await invoke("save_brick", { brick: brick });
}

async function onOpenBrick() {
  await invoke("open_brick", { brickName: brick.name });
}

async function deleteBrick() {
  showDeleteBrickModal.value = false;
  await invoke("delete_brick", { brick: brick });
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
</style>
