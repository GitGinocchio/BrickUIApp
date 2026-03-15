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
          <n-tooltip trigger="hover" placement="bottom" :delay="500">
            <template #trigger>
              <n-button size="small" @click="onSaveColor">Save</n-button>
            </template>
            Clicca salva per salvare un colore
          </n-tooltip>
          <n-tooltip trigger="hover" placement="bottom" :delay="500">
            <template #trigger>
              <n-button size="small" @click="onRemoveColor">Remove</n-button>
            </template>
            Clicca rimuovi per eliminare un colore dai salvati
          </n-tooltip>
          <n-tooltip trigger="hover" placement="bottom" :delay="500">
            <template #trigger>
              <n-button size="small" @click="onClearColor">Clear</n-button>
            </template>
            Clicca pulisci per rimuovere il colore attuale
          </n-tooltip>
        </template>
      </component>
      <template v-if="editMode">
        <div>
          <n-button circle text @click="emit('edit:prop', prop)"><Wrench :size="16" /></n-button>
          <n-button circle text @click="emit('delete:prop', prop)"><Trash2 :size="16" /></n-button> 
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NInput, NButton, NInputNumber, NTooltip, NSelect, NDynamicTags, NColorPicker, NSwitch, NDatePicker, NTimePicker } from 'naive-ui';
import { Trash2, Wrench } from 'lucide-vue-next';
import MarkdownIt from 'markdown-it';

import type { Prop as BrickPropType } from '#interfaces/brick';
import { colorStringToRGBA } from '#utils/color';
import GradientPicker from '#components/GradientPicker.vue';

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
  'Text' : NInput,
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
  'Gradient' : GradientPicker,
  'Date' : NDatePicker,
  'Datetime' : NDatePicker,
  'Time' : NTimePicker
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
      return {
        placeholder: 'Type a string value...',
        defaultValue: prop.default,
        clearable: true
      };
    case 'Int':
    case 'Float':
      return {
        precision: prop.prop_type === 'Int' ? 0 : 2,
        min: prop.min,
        max: prop.max,
        step: prop.step ? prop.step : (prop.prop_type === 'Float' ? 0.1 : 1),
        defaultValue: prop.default,
        placeholder: prop.default ? `${prop.default}` : 'Type a number...',
        clearable: true
      };
    case 'Text':
      return {
        type: 'textarea',
        placeholder: 'Type a multiline text value...',
        defaultValue: prop.default,
        clearable: true
      };

    case 'Select':
      return {
        defaultValue: prop.default && prop.default.length > 0 ? (prop.max > 1 ? prop.default : prop.default[0]) : null,
        options: prop.options.map(option => ({ label: option, value: option })),
        placeholder: 'Select a value...',
        multiple: prop.max > 1,
        clearable: true
      };

    case 'Array':
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
      };

    case 'Gradient':
      return {
        defaultValue: prop.value,
        "onUpdate:value" : (stops) => (prop.value = stops)
      };

    case 'Date':
    case "Datetime":
    case "Time":
      const allow_future = prop.allow_future;
      const allow_past = prop.allow_past;

      return {
        type: prop.prop_type === 'Datetime' ? 'datetime' : 'date',
        clearable: true,
        defaultValue: prop.value,
        "onUpdate:value": (value) => (prop.value = value),
        isTimeDisabled: (current) => {
          const date = new Date(current);
          const now = new Date();

          const isToday =
            date.getFullYear() === now.getFullYear() &&
            date.getMonth() === now.getMonth() &&
            date.getDate() === now.getDate();

          return {
            isHourDisabled: (hour: number) => {
              if (!isToday) return false;

              if (!allow_past && allow_future) {
                return hour < now.getHours();
              }
              if (!allow_future && allow_past) {
                return hour > now.getHours();
              }
              return false;
            },

            isMinuteDisabled: (minute: number, hour: number) => {
              if (!isToday) return false;

              if (!allow_past && allow_future && hour === now.getHours()) {
                return minute < now.getMinutes();
              }
              if (!allow_future && allow_past && hour === now.getHours()) {
                return minute > now.getMinutes();
              }
              return false;
            },

            isSecondDisabled: (second: number, minute: number, hour: number) => {
              if (!isToday) return false;

              if (!allow_past && allow_future && hour === now.getHours() && minute === now.getMinutes()) {
                return second < now.getSeconds();
              }
              if (!allow_future && allow_past && hour === now.getHours() && minute === now.getMinutes()) {
                return second > now.getSeconds();
              }
              return false;
            }
          };
        },
        isDateDisabled: (ts: number) => {
          const date = new Date(ts);
          const now = new Date();
          if (allow_future) now.setHours(now.getHours() - 24);

          if (allow_future && date.getTime() >= now.getTime()) {
            return false;
          }
          if (allow_past && date.getTime() <= now.getTime()) {
            return false;
          }

          return true;
        }
      };

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
        return prop.value ?? prop.default ?? null;
      case 'Text':
        return prop.value ?? prop.default ?? null;
      case 'Int':
        return prop.value ?? prop.default ?? null;
      case 'Float':
        return prop.value ?? prop.default ?? null;

      case 'Select':
        if (prop.max <= 1) {
          return prop.value[0] ?? prop.default[0] ?? null;
        }
        return prop.value ?? prop.default ?? [];

      case 'Array':
        return (prop.value ?? prop.default ?? []).map(v => ({ label: String(v), value: v }));

      case 'Color':
        return prop.value ?? prop.default ?? '#00000000';

      case 'Gradient':
        return prop.value ?? prop.default ?? [];

      case "Date":
      case "Datetime":
      case "Time":
        return prop.value ?? prop.default ?? null;

      default:
        return [];
    }
  },
  set(newValue) {
    console.log(newValue);
    let value;
    switch (prop.prop_type) {
      case 'Array':
        if (prop.value_type === 'String') {
          value = newValue
            ? newValue
                .map(item => (typeof item === 'string' ? item : item.value))
            : prop.default ?? [];
          break;
        }
        else if (prop.value_type === 'Integer') {
          value = newValue
            ? newValue
                .map(item => (typeof item === 'string' ? /[a-zA-Z]/.test(item) ? NaN : parseInt(item.replace(',', '.')) : item.value))
                .filter(val => !isNaN(val))
            : prop.default ?? [];
          break;
        }
        else if (prop.value_type === 'Float') {
          value = newValue
            ? newValue
                .map(item => (typeof item === 'string' ? (/[a-zA-Z]/.test(item) ? NaN : parseFloat(item.replace(',', '.'))) : item.value))
                .filter(val => !isNaN(val))
            : prop.default ?? [];
          break;
        }
        break;

      case 'Select':
        if (newValue === null || newValue === undefined) {
          value = [];
          break;
        }

        if (!Array.isArray(newValue)) {
          newValue = [newValue];
        }

        if (prop.max && newValue.length > prop.max) {
          newValue = newValue.slice(0, prop.max);
        }

        value = newValue;
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

      case 'Date':
      case 'Datetime':
      case 'Time':
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
  margin-left: 0.75rem;
  margin-right: 0.75rem;
}

.brick-prop label {
  font-weight: normal;
  margin-left: 0rem;
}

::deep(.n-popover__content) {
  display: flex;
}

.n-input-number,
.n-date-picker,
.n-time-picker {
  width: 100%;
}

.prop-input-section {
  display: flex;
  justify-content: space-between;
  width: 100%;
  flex-direction: row;
  gap: 0.5rem;
}

.prop-input-section div {
  display: flex;
  flex-direction: row;
  gap: 0.5rem;
}

.prop-input-component {
  justify-content: flex-start;
}
</style>

<style>
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