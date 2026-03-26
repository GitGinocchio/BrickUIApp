<template>
  <component 
    :is="currentComponent"
    :key="prop.prop_name"
    v-bind="componentProps" 
    v-model:value="modelValue"
  >
    <template v-if="currentComponent === NColorPicker && prop.prop_type === 'Color'" #action>
      <div class="color-picker-actions">
        <n-tooltip trigger="hover" placement="bottom" :delay="500">
          <template #trigger>
            <n-button size="small" @click="onSaveColor">Save</n-button>
          </template>
          Salva nei preferiti
        </n-tooltip>
        <n-tooltip trigger="hover" placement="bottom" :delay="500">
          <template #trigger>
            <n-button size="small" @click="onRemoveColor">Remove</n-button>
          </template>
          Rimuovi dai preferiti
        </n-tooltip>
        <n-tooltip trigger="hover" placement="bottom" :delay="500">
          <template #trigger>
            <n-button size="small" @click="onClearColor">Clear</n-button>
          </template>
          Resetta colore
        </n-tooltip>
      </div>
    </template>
  </component>
</template>

<script setup lang="ts">
import { computed, type PropType } from 'vue';
import {
  NInput, NButton, NInputNumber, NTooltip, NSelect,
  NDynamicTags, NColorPicker, NSwitch, NDatePicker, NTimePicker
} from 'naive-ui';
import type { Prop as BrickPropType } from '#interfaces/brick';
import { colorStringToRGBA } from '#utils/color';
import GradientPicker from '#components/GradientPicker.vue';

const props = defineProps({
  prop: {
    type: Object as PropType<BrickPropType>,
    required: true
  }
});

const emit = defineEmits<{
  (e: 'update:prop', prop: BrickPropType): void
}>();

// 1. Mapping dei componenti
const componentMap: Record<string, any> = {
  'String': NInput, 'Text': NInput, 'Any': NInput,
  'Int': NInputNumber, 'Float': NInputNumber,
  'Bool': NSwitch,
  'Select': NSelect, 'StringSelect': NSelect, 'IntSelect': NSelect, 'FloatSelect': NSelect,
  'Array': NDynamicTags, 'StringArray': NDynamicTags, 'IntArray': NDynamicTags, 'FloatArray': NDynamicTags,
  'Color': NColorPicker,
  'Gradient': GradientPicker,
  'Date': NDatePicker, 'Datetime': NDatePicker, 'Time': NTimePicker
};

const currentComponent = computed(() => componentMap[props.prop.prop_type] || NInput);

// 2. Logica di trasformazione del valore (Getter/Setter)
const modelValue = computed<any>({
  get() {
    const p = props.prop;
    switch (p.prop_type) {
      case 'Bool': return p.value ?? p.default ?? false;
      case 'Select':
        return p.max <= 1 ? (p.value?.[0] ?? p.default?.[0] ?? null) : (p.value ?? p.default ?? []);
      case 'Array':
        return (p.value ?? p.default ?? []).map((v: any) => ({ label: String(v), value: v }));
      case 'Color': return p.value ?? p.default ?? '#00000000';
      case 'Gradient': return p.value ?? p.default ?? [];
      case 'Date': case 'Datetime': case 'Time': return p.value ?? p.default ?? null;
      default: return p.value ?? p.default ?? null;
    }
  },
  set(newValue) {
    const p = props.prop;
    let finalValue;

    switch (p.prop_type) {
      case 'Array':
        finalValue = newValue ? newValue.map((item: any) => (typeof item === 'string' ? item : item.value)) : [];
        if (p.value_type === 'Integer') finalValue = finalValue.map((v: any) => parseInt(v)).filter((v: any) => !isNaN(v));
        break;
      case 'Select':
        finalValue = Array.isArray(newValue) ? newValue : (newValue ? [newValue] : []);
        break;
      case 'Color':
        finalValue = newValue ? colorStringToRGBA(newValue) : "#00000000";
        break;
      default:
        finalValue = newValue;
    }

    p.value = finalValue;
    emit('update:prop', p);
  }
});

// 3. Props dinamiche per i componenti Naive UI
const componentProps = computed(() => {
  const p = props.prop;
  const base = { clearable: true, placeholder: 'Select/Type value...' };

  switch (p.prop_type) {
    case 'Bool': return { round: true };
    case 'Int': case 'Float':
      return { 
        ...base, 
        precision: p.prop_type === 'Int' ? 0 : 2, 
        step: p.step || (p.prop_type === 'Float' ? 0.1 : 1), 
        min: p.min, 
        max: p.max 
      };
    case 'Text': return { ...base, type: 'textarea' };
    case 'Select':
      return { ...base, options: p.options?.map(o => ({ label: o, value: o })), multiple: p.max > 1 };
    case 'Color':
      console.log([...(p.saved || []), ...(p.swatches || [])])
      const swatches = [...(p.saved || []), ...(p.swatches || [])];
      return {
        swatches: swatches.length > 0 ? swatches : null, 
        'show-alpha': !p.skip_alpha 
      };
    case 'Gradient':
      return { 
        'show-alpha' : !p.skip_alpha,
        'default': p.default
      };
    case 'Date': case 'Datetime':
      return { 
        ...base, 
        type: p.prop_type === 'Datetime' ? 'datetime' : 'date',
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

              if (!p.allow_past && p.allow_future) {
                return hour < now.getHours();
              }
              if (!p.allow_future && p.allow_past) {
                return hour > now.getHours();
              }
              return false;
            },
            isMinuteDisabled: (minute: number, hour: number) => {
              if (!isToday) return false;

              if (!p.allow_past && p.allow_future && hour === now.getHours()) {
                return minute < now.getMinutes();
              }
              if (!p.allow_future && p.allow_past && hour === now.getHours()) {
                return minute > now.getMinutes();
              }
              return false;
            },
            isSecondDisabled: (second: number, minute: number, hour: number) => {
              if (!isToday) return false;

              if (!p.allow_past && p.allow_future && hour === now.getHours() && minute === now.getMinutes()) {
                return second < now.getSeconds();
              }
              if (!p.allow_future && p.allow_past && hour === now.getHours() && minute === now.getMinutes()) {
                return second > now.getSeconds();
              }
              return false;
            }
          };
        },
        isDateDisabled: (ts: number) => {
          const date = new Date(ts);
          const now = new Date();
          if (p.allow_future) now.setHours(now.getHours() - 24);

          if (p.allow_future && date.getTime() >= now.getTime()) {
            return false;
          }
          if (p.allow_past && date.getTime() <= now.getTime()) {
            return false;
          }

          return true;
        }
      };
    case 'Time':
      return { ...base, type: 'time' };
    default: {};
  }
});

// 4. Azioni specifiche Color Picker
function onClearColor() { modelValue.value = null; }
function onSaveColor() {
  if (props.prop.prop_type !== 'Color') return;
  if (!props.prop.saved) props.prop.saved = [];
  if (!props.prop.saved.includes(props.prop.value)) {
    props.prop.saved.unshift(props.prop.value);
    emit('update:prop', props.prop);
  }
}
function onRemoveColor() {
  if (props.prop.prop_type === 'Color' && props.prop.saved) {
    props.prop.saved = props.prop.saved.filter(v => v !== props.prop.value);
    emit('update:prop', props.prop);
  }
}
</script>

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

<style scoped>
.color-picker-actions {
  display: flex;
  flex-flow: row;
  gap: 8px;
  padding: 4px;
}

/* Fix larghezza per picker specifici */
:deep(.n-input-number),
:deep(.n-date-picker),
:deep(.n-time-picker),
:deep(.n-select) {
  width: 100% !important;
}
</style>