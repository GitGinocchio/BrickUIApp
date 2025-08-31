<template>
  <div class="brick-prop">
    <n-tooltip :delay="600" :trigger="prop.description ? 'hover' : 'manual'" placement="top-start">
      <template #trigger>
        <label>{{ prop.prop_name }}</label>
      </template>
      <div v-html="renderedDescription" class="brick-prop-description"></div>
    </n-tooltip>
    <div class="prop-input-section">
      <component
        :is="currentComponent"
        v-bind="componentProps"
        v-model:value="modelValue"
        class="prop-input-component"
      >
        <template v-if="currentComponent == NColorPicker && prop.prop_type == 'Color'" #action>
          <n-button size="small" @click="onSaveColor">Save</n-button>
          <n-button size="small" @click="onRemoveColor">Remove</n-button>
          <n-button size="small" @click="onClearColor">Clear</n-button>
        </template>
      </component>
      <template v-if="editMode">
        <n-button circle text @click="emit('edit:prop', prop)"><Wrench :size="16" /></n-button>
        <n-button circle text @click="emit('delete:prop', prop)"><Trash2 :size="16" /></n-button>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NInput, NButton, NInputNumber, NTooltip, NSelect, NDynamicTags, NColorPicker, NSwitch } from 'naive-ui';
import { computed, watch } from 'vue';
//import { debounce } from 'lodash-es'; // puoi anche scrivere una funzione debounce a mano
import type { PropType } from 'vue';
import type { Prop as BrickPropType } from 'interfaces/brick';
import { colorStringToRGBA } from '../utils';
import MarkdownIt from 'markdown-it';
import { Trash2, Wrench } from 'lucide-vue-next';
import GradientPicker from './GradientPicker.vue';

const md = new MarkdownIt();

let { prop, editMode } = defineProps({
  prop: { 
    type: Object as PropType<BrickPropType>, 
    required: true 
  },
  editMode: {
    type: Boolean,
    required: true
  }
});

const emit = defineEmits<{
  (e: 'update:prop', prop: BrickPropType): void
  (e: 'edit:prop', prop: BrickPropType): void
  (e: 'delete:prop', prop: BrickPropType): void
}>();


const renderedDescription = computed(() => md.render(prop.description));

// Mapping dei componenti
const componentMap: Record<string, any> = {
  'String': NInput,
  'Any': NInput,
  'Int': NInputNumber,
  'Float': NInputNumber,
  'Bool' : NSwitch,
  'StringSelect': NSelect,
  'IntSelect': NSelect,
  'FloatSelect': NSelect,
  'Select': NSelect,
  'Array': NDynamicTags,
  'StringArray': NDynamicTags,
  'IntArray': NDynamicTags,
  'FloatArray': NDynamicTags,
  'Color' : NColorPicker,
  'Gradient' : GradientPicker
};

const currentComponent = computed(() => componentMap[prop.prop_type] || NInput);

const componentProps = computed(() => {
  switch (prop.prop_type) {
    case 'Bool':
      return {
        round: true,
        defaultValue: prop.default || false
      };
    case 'String':
    case 'Any':
      return {
        placeholder: prop.default || 'Type a string value...',
        clearable: true
      };
    case 'Int':
    case 'Float':
      return {
        precision: prop.prop_type === 'Int' ? 0 : 2,
        min: prop.min,
        max: prop.max,
        defaultValue: prop.default,
        placeholder: prop.default ? `${prop.default}` : 'Type a number...',
        clearable: true
      };
    

    case 'Select':
    case 'StringSelect':
    case 'IntSelect':
    case 'FloatSelect':
      return {
        options: prop.options.map(option => ({ label: option, value: option })),
        placeholder: String(prop.default) || 'Select a value...',
        multiple: prop.max > 1,
        clearable: true
      };

    case 'Array':
    case 'StringArray':
    case 'IntArray':
    case 'FloatArray':
      return {
        defaultValue: prop.default?.map(v => ({ label: String(v), value: v })) || [],
        round: true,
        min: prop.min,
        max: prop.max
      };
    
    case 'Color':
      const merged = [
        ...(Array.isArray(prop.saved) ? prop.saved : []),
        ...(Array.isArray(prop.swatches) ? prop.swatches : []),
      ]

      return {
        placement: "top-start",
        swatches: merged.length > 0 ? merged : null,
        'show-alpha': !prop.skip_alpha,
        'show-preview': true
      }

    case 'Gradient':
      return {
        defaultValue: prop.value,
        "onUpdate:value" : (stops) => (prop.value = stops)
      }

    default:
      return {};
  }
});

const modelValue = computed<any>({
  get() {
    switch (prop.prop_type) {
      case 'Bool':
        return prop.value ?? prop.default ?? false;
      case 'String':
        return prop.value ?? prop.default ?? '';
      case 'Any':
        return prop.value ?? prop.default ?? null;
      case 'Int':
        return prop.value ?? prop.default ?? null;
      case 'Float':
        return prop.value ?? prop.default ?? null;

      case 'Select':
      case 'StringSelect':
      case 'IntSelect':
      case 'FloatSelect':
        return prop.value ?? prop.default ?? [];

      case 'Array':
      case 'StringArray':
      case 'IntArray':
      case 'FloatArray':
        return (prop.value ?? prop.default ?? []).map(v => ({ label: String(v.toLocaleString()), value: v }));

      case 'Color':
        return prop.value ?? prop.default ?? '#00000000';

      case 'Gradient':
        return prop.value ?? prop.default ?? [];

      default:
        return [];
    }
  },
  set(newValue) {
    console.log(newValue);
    let value;
    switch (prop.prop_type) {
      case 'Array':
      case 'StringArray':
        value = newValue
          ? newValue
              .map(item => (typeof item === 'string' ? item : item.value))
          : prop.default ?? [];
        break;
      case 'IntArray':
        value = newValue
          ? newValue
              .map(item => (typeof item === 'string' ? /[a-zA-Z]/.test(item) ? Number.NaN : parseInt(item) : item.value))
              .filter(val => !isNaN(val))
          : prop.default ?? [];
        break;
      case 'FloatArray':
        value = newValue
          ? newValue
              .map(item => (typeof item === 'string' ? /[a-zA-Z]/.test(item) ? null : parseFloat(item) : item.value))
              .filter(val => !isNaN(val))
          : prop.default ?? [];
        break;

      case 'Color':
        value = newValue ? colorStringToRGBA(newValue) : (prop.default ?? "#00000000");
        break;

      case 'Gradient':
        value = newValue;
        break;

      case 'Bool':
        value = newValue;
        break;

      default:
        value =  newValue !== null ? newValue : (prop.default ? prop.default : null);
    }
    prop.value = value;
  }
});

function onClearColor() {
  modelValue.value = null;
}

function onSaveColor() {
  if (prop.prop_type !== 'Color') return;

  // Se il colore è già nei swatches, non fare nulla
  if (prop.swatches && prop.swatches.includes(prop.value)) return;

  // Aggiungi il colore a saved se non presente
  if (prop.saved) {
    if (!prop.saved.includes(prop.value)) {
      prop.saved.unshift(prop.value);
    }
  } else {
    prop.saved = [prop.value];
  }
  emit("update:prop", prop)
}

function onRemoveColor() {
  if (prop.prop_type !== 'Color') return;

  if (prop.saved && prop.saved.includes(prop.value)) {
    prop.saved = prop.saved.filter((value) => value !== prop.value);
  }
  emit("update:prop", prop)
}

// Watch per salvare automaticamente
watch(modelValue, () => {
  emit("update:prop", prop)
});
</script>

<style scoped>
.brick-prop {
  display: flex;
  width: 100%;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
  margin-top: 0.5rem;
}

.brick-prop label {
  font-weight: normal;
  margin-left: 0rem;
}
</style>

<style>
::deep(.n-popover__content) {
  display: flex;
}

.prop-input-section {
  display: flex;
  flex-direction: row;
  gap: 0.5rem;
}

.prop-input-component {
  justify-content: flex-start;
  width: 100%;
}

.brick-prop-description * {
  margin: 0;
}

.brick-prop-description {
  white-space: pre-line; 
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.brick-prop-description p {
  display: flex;
  flex-direction: row;
  gap: 0.25rem;
}

.brick-prop-description ul {
  display: flex;
  flex-direction: column;
  list-style-type: disc;
  padding-left: 1.2em;   /* spazio per i puntini */
  margin: 0;             /* opzionale, per togliere margini */
}

.brick-prop-description li {
  margin-bottom: 0.003em;
}

.brick-prop-description code {
  background-color: #f5f5f5;   /* grigio chiaro */
  padding: 2px 4px;
  border-radius: 3px;
  font-family: monospace;
  font-size: 0.9em;
  color: #c7254e;  /* un rosso scuro, ma puoi scegliere altro */
}
</style>
