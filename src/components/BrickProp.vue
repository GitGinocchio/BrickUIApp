<template>
  <n-tooltip :trigger="prop.description ? 'hover' : 'manual'" placement="top-start">
    <template #trigger>
      <div class="brick-prop">
        <label>{{ capitalize(prop.prop_name) }}</label>
        <component
          :is="currentComponent"
          v-bind="componentProps"
          v-model:value="modelValue"
        /> 
      </div>
    </template>
    <div v-html="renderedDescription" class="brick-prop-description"></div>
  </n-tooltip>
</template>

<script setup lang="ts">
import { NInput, NInputNumber, NTooltip, NSelect, NDynamicTags, NColorPicker } from 'naive-ui';
import { computed, watch } from 'vue';
//import { debounce } from 'lodash-es'; // puoi anche scrivere una funzione debounce a mano
import type { PropType } from 'vue';
import type { Prop as BrickPropType } from 'interfaces/brick';
import { capitalize, colorStringToRGBA, RGBAToHex } from '../utils';
import MarkdownIt from 'markdown-it';

const md = new MarkdownIt();

let { prop } = defineProps({
  prop: { 
    type: Object as PropType<BrickPropType>, 
    required: true 
  }
});

const emit = defineEmits<{
  (e: 'update:prop', prop: BrickPropType): void
}>();


const renderedDescription = computed(() => md.render(prop.description));

// Mapping dei componenti
const componentMap: Record<string, any> = {
  'String': NInput,
  'Any': NInput,
  'Int': NInputNumber,
  'Float': NInputNumber,
  'StringSelect': NSelect,
  'IntSelect': NSelect,
  'FloatSelect': NSelect,
  'Select': NSelect,
  'Array': NDynamicTags,
  'StringArray': NDynamicTags,
  'IntArray': NDynamicTags,
  'FloatArray': NDynamicTags,
  'Color' : NColorPicker
};

const currentComponent = computed(() => componentMap[prop.prop_type] || NInput);

const componentProps = computed(() => {
  switch (prop.prop_type) {
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
        placeholder: prop.default || 'Select a value...',
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
      console.log('default:', prop.default)
      return {
        swatches: prop.swatches ? prop.swatches.map((value) => RGBAToHex(value)) : null,
        actions: ['clear'],
        'show-alpha': !prop.skip_alpha
      }

    default:
      return {};
  }
});

const modelValue = computed<any>({
  get() {
    switch (prop.prop_type) {
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
        return RGBAToHex(prop.value ?? prop.default ?? [0,0,0,0]);

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
        value = newValue ? colorStringToRGBA(newValue) : prop.default ?? "#00000000";
        break;

      default:
        value =  newValue ? newValue : (prop.default ? prop.default : null);
    }
    prop.value = value;
  }
});

// Watch per salvare automaticamente
watch(modelValue, () => {
  emit("update:prop", prop)
});
</script>

<style scoped>
.brick-prop {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.brick-prop label {
  margin-left: 0rem;
}
</style>

<style>
::deep(.n-popover__content) {
  display: flex;
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
