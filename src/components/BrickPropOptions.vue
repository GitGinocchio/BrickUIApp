<template>
  <div class="brick-prop-options">
    <n-form class="brick-prop-items">
      <n-form-item label="Description:">
        <n-input
          v-model:value="prop.description"
          placeholder="Type the prop's description (Markdown supported)"
          type="textarea"
          maxlength="512"
          show-count
          resizable
          :autosize="{ minRows: 2 }"
        />
      </n-form-item>

      <n-form-item label="Type:">
        <n-select 
          :value="prop.prop_type" 
          :options="propTypes"
          :render-label="renderLabel"
          @update:value="(value) => onNewPropTypeSelected(value)"
        />
      </n-form-item>

      <n-form-item 
        label="Value type"
        v-if="prop.prop_type === 'Select' || prop.prop_type === 'Array'"
      >
        <n-select
          v-model:value="prop.value_type"
          :options="valueTypeOptions"
          :render-label="renderValueTypeLabel"
          @update:value="onValueTypeChanged"
        />
      </n-form-item>
      
      <n-form-item v-if="optionsInputField" label="Options:">
        <component :is="optionsInputField" />
      </n-form-item>

      <n-form-item v-if="prop.prop_type == 'Color' || prop.prop_type == 'Gradient'" label="Skip Alpha:">
        <n-switch v-model:value="prop.skip_alpha" @update:value="onSkipAlphaChanged" />
      </n-form-item>

      <n-form-item v-if="defaultInputField" label="Default value:">
        <div class="default-input-wrapper">
          <component :is="defaultInputField">
            <template v-if="prop.prop_type === 'Color'" #action>
              <n-space size="small" style="padding: 4px; flex-flow: row;">
                <n-button size="small" @click="onSaveColor">Save</n-button>
                <n-button size="small" @click="onRemoveColor">Remove</n-button>
                <n-button size="small" @click="onClearColor">Clear</n-button>
              </n-space>
            </template>
          </component>
        </div>
      </n-form-item>

      <n-form-item v-if="minInputField" :label="minLabel">
        <component :is="minInputField" />
      </n-form-item>

      <n-form-item v-if="maxInputField" :label="maxLabel">
        <component :is="maxInputField" />
      </n-form-item>

      <n-form-item v-if="stepInputField" label="Step value:">
        <component :is="stepInputField" />
      </n-form-item>

      <template v-if="prop.prop_type == 'Date' || prop.prop_type == 'Datetime'">
        <n-form-item label="Allow past:">
          <n-switch :value="prop.allow_past" @update:value="v => toggleDateConstraint('past', v)" />
        </n-form-item>
        <n-form-item label="Allow future:">
          <n-switch :value="prop.allow_future" @update:value="v => toggleDateConstraint('future', v)" />
        </n-form-item>
      </template>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { 
  NInput, NButton, NInputNumber, NSelect, NSwitch, 
  NDynamicTags, NColorPicker, NDatePicker, NTimePicker, NSpace, NForm, NFormItem, 
  useDialog
} from 'naive-ui';
import { 
  List, ListTodo, PaintBucket, SwatchBook, ToggleLeft, Type, Text, 
  DecimalsArrowRight, ArrowUp10, Clock, Calendar1, CalendarClock
} from 'lucide-vue-next';

import { createProp, type Prop, type PropTypeValue, propTypeValues, type SelectablePropType } from '#interfaces/brick';
import { colorStringToRGBA } from '#utils/color';
import GradientPicker from '#components/GradientPicker.vue';
import type { DatePickerType } from 'naive-ui/es/date-picker/src/config';

const dialog = useDialog();

// --- Props & Emits ---
const prop = defineModel<Prop>("prop", { required: true });
const props = defineProps({
  allProps: {
    type: Object as PropType<Prop[]>,
    required: true
  },
  showAlertOnTypeChange: {
    default: true
  }
});
const emit = defineEmits<{
  (e: 'update:prop', prop: Prop): void
}>();

watch(() => prop.value, (updated) => emit("update:prop", updated), { deep: true });

// --- Mapping Icone e Tipi ---
const iconsMap: Record<string, any> = {
  Select: ListTodo, Array: List, String: Type, Text: Text,
  Int: ArrowUp10, Float: DecimalsArrowRight, Bool: ToggleLeft,
  Color: PaintBucket, Gradient: SwatchBook, Datetime: CalendarClock, 
  Date: Calendar1, Time: Clock
};

const propTypes = propTypeValues.map((value) => ({
  label: value.replace(/([A-Z])/g, " $1").trim(),
  value,
  icon: () => h(iconsMap[value], { size: 16 })
}));

const valueTypeOptions = [
  { label: 'String', value: 'String', icon: h(Type, { size: 16 }) }, 
  { label: 'Float', value: 'Float', icon: h(DecimalsArrowRight, { size: 16 }) },
  { label: 'Integer', value: 'Integer', icon: h(ArrowUp10, { size: 16 }) },
];

// --- Helper Functions ---
const renderLabel = (option: any) => h("div", [option.icon(), h("span", option.label)]);
const renderValueTypeLabel = (option: any) => h('div', [option.icon, h('span', option.label)]);

const minLabel = computed(() => ['Int', 'Float'].includes(prop.value.prop_type) ? 'Minimum allowed number:' : 'Minimum number of values:');
const maxLabel = computed(() => ['Int', 'Float'].includes(prop.value.prop_type) ? 'Maximum allowed number:' : 'Maximum number of values:');

function onNewPropTypeSelected(newPropType: PropTypeValue) {
  const onPositiveClick = () => {
    const current = prop.value;
    const subType = (newPropType === 'Select' || newPropType === 'Array') ? 'String' : null;
    prop.value = createProp(newPropType, current.prop_name, current.description, subType);
  }

  if (!props.showAlertOnTypeChange) {
    onPositiveClick();
    return;
  }

  dialog.warning({
    title: 'Confirm new prop type',
    content: () => [
      h("p", {},`Are you sure you want to change "${prop.value.prop_name}" type from ${prop.value.prop_type} to ${newPropType}?`),
      h("p", {}, "You will lose all the prop's settings. The action is irreversible.")
    ],
    positiveText: 'Change',
    negativeText: 'Cancel',
    onPositiveClick: onPositiveClick
  });
}

function onValueTypeChanged() {
  if (prop.value.prop_type === 'Select') prop.value.options = [];
  else if (prop.value.prop_type === 'Array') prop.value.default = [];
}

function toggleDateConstraint(type: 'past' | 'future', value: boolean) {
  if (prop.value.prop_type !== 'Date' && prop.value.prop_type !== 'Datetime') return;
  if (type === 'past') {
    prop.value.allow_past = value;
    if (!value && !prop.value.allow_future) prop.value.allow_future = true;
  } else {
    prop.value.allow_future = value;
    if (!value && !prop.value.allow_past) prop.value.allow_past = true;
  }
}

// --- Gestione Colori ---
function onSaveColor() {
  if (prop.value.prop_type !== 'Color' || !prop.value.default) return;
  if (!prop.value.swatches) prop.value.swatches = [];
  if (!prop.value.swatches.includes(prop.value.default)) {
    prop.value.swatches = [prop.value.default, ...prop.value.swatches];
  }
}
function onRemoveColor() {
  if (prop.value.prop_type === 'Color' && prop.value.swatches) {
    prop.value.swatches = prop.value.swatches.filter(v => v !== prop.value.default);
  }
}
function onClearColor() { prop.value.default = null; }

function onSkipAlphaChanged(skip: boolean) {
  if (prop.value.prop_type === 'Color' && prop.value.default) {
    prop.value.default = prop.value.default.slice(0, 7) + (skip ? "FF" : "CC");
  }
}

const defaultInputField = computed(() => {
  switch (prop.value.prop_type) {
    case "String":
      return h(
        NInput,
        {
          value: prop.value.default,
          placeholder: "Input a string value",
          clearable: true,
          "onUpdate:value": (val: string) => (prop.value.default = val)
        }
      );
    case "Text":
      return h(
        NInput,
        {
          value: prop.value.default,
          placeholder: "Input a multiline string value",
          type: 'textarea',
          clearable: true,
          "onUpdate:value": (val: string) => (prop.value.default = val)
        }
      );
    
    case "Int":
    case "Float":
      return h(
        NInputNumber,
        {
          value: prop.value.default,
          min: prop.value.min,
          max: prop.value.max,
          clearable: true,
          precision: prop.value.prop_type == 'Int' ? 0 : 2,
          step: prop.value.step ? prop.value.step : (prop.value.prop_type === 'Float' ? 0.1 : 1),
          "onUpdate:value": (val: number) => (prop.value.default = val)
        }
      );
    
    case "Bool":
      return h(
        NSwitch,
        {
          value: prop.value.default,
          clearable: true,
          "onUpdate:value": (val: boolean) => (prop.value.default = val)
        }
      );

    case "Select":
      return h(
        NSelect,
        {
          value: prop.value.default && prop.value.default.length > 0 ? (prop.value.max > 1 ? prop.value.default : prop.value.default[0]) : null,
          clearable: true,
          multiple: prop.value.max > 1,
          options: (prop.value.options || []).map(v => ({
            label: String(v),
            value: v
          })),
          "onUpdate:value": (val: any) => {
            if (val === null || val === undefined) {
              prop.value.default = [];
            } else if (Array.isArray(val)) {
              prop.value.default = val;
            } else {
              prop.value.default = [val];
            }
          }
        }
      );

    case "Array":
      const value_type = prop.value.value_type;
      return h(
        NDynamicTags,
        {
          value: prop.value.default ? prop.value.default.map((value : string|number|any) => String(value)) : [],
          clearable: true,
          "onUpdate:value": (newValue: any[]) => {
            let parsedValue;

            if (value_type === 'String') {
              parsedValue = newValue.map(item => typeof item === 'string' ? item : item.value);
            } 
            else if (value_type === 'Integer') {
              parsedValue = newValue
                .map(item => {
                  const val = typeof item === 'string'
                    ? /[a-zA-Z]/.test(item) ? NaN : parseInt(item)
                    : item.value;
                  return isNaN(val) ? null : val;
                })
                .filter(val => val !== null);
            } 
            else if (value_type === 'Float') {
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
            prop.value.default = parsedValue;
          }
        }
      );
    
    case "Color":
      return h(
        NColorPicker, 
        {
          value: prop.value.default ?? (prop.value.skip_alpha ? "#000000FF" : "#00000000"),
          swatches: prop.value.swatches.length > 0 ? prop.value.swatches : null,
          'showAlpha' : !prop.value.skip_alpha,
          clearable: true,
          "onUpdate:value": (val: string) => (prop.value.default = val !== null ? colorStringToRGBA(val) : val)
        }
      );
    
    case "Gradient":
      return h(
        GradientPicker,
        {
          "skip_alpha" : prop.value.skip_alpha,
          "value": prop.value.default ?? [{ color: prop.value.skip_alpha ? "#000000FF" : "#00000000", position: 50 }],
          "onUpdate:value" : (stops) => (prop.value.default = stops)
        }
      );

    case "Datetime":
    case "Date":
      const allow_future = prop.value.allow_future;
      const allow_past = prop.value.allow_past;

      let type: DatePickerType;
      switch (prop.value.prop_type) {
        case "Date":
          type = "date";
          break;
        case "Datetime":
          type = "datetime";
          break;
      }

      return h(
        NDatePicker,
        {
          clearable: true,
          type: type,
          value: prop.value.default ? prop.value.default : null,
          onUpdateValue: (value: number) => {
            prop.value.default = value
          },
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
        }
      );
    case "Time":
      return h(
        NTimePicker,
        {
          clearable: true,
          value: prop.value.default ? prop.value.default : null,
          onUpdateValue: (value: number) => {
            prop.value.default = value
          },
        }
      );

    default:
      return null
  }
});

const optionsInputField = computed(() => {
  switch (prop.value.prop_type) {
    case "Select":
      const p = prop.value as SelectablePropType<any>;
      const value_type = p.value_type;
      return h(
        NDynamicTags, 
        {
          value: p.options.map((value) => String(value)),
          "onUpdate:value": (newValue: any[]) => {
            let parsedValue;

            if (value_type === 'String') {
              parsedValue = newValue.map(item => typeof item === 'string' ? item : item.value);
            } 
            else if (value_type === 'Integer') {
              parsedValue = newValue
                .map(item => {
                  const val = typeof item === 'string'
                    ? /[a-zA-Z]/.test(item) ? NaN : parseInt(item)
                    : item.value;
                  return isNaN(val) ? null : val;
                })
                .filter(val => val !== null);
            } 
            else if (value_type === 'Float') {
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
            
            p.options = [...new Set(parsedValue)];

            p.default = p.options.includes(p.default) ? p.default : null;
          },
          type : 'info'
        }
      );
    default:
      return null;
  };
});

const minInputField = computed(() => {
  switch (prop.value.prop_type) {
    case "Int":
    case "Float":
    case "Array":
    case "Select":
      if (prop.value.max !== null && prop.value.min > prop.value.max) {
        prop.value.max = prop.value.min;
      }
      return h(
        NInputNumber, {
          value: prop.value.min,
          defaultValue: prop.value.min ? prop.value.min : 0,
          placeholder: ['Int', 'Float'].includes(prop.value.prop_type) ? "Input the minumum allowed number" : "Input the minumum number of values",
          clearable: true,
          precision: prop.value.prop_type === 'Float' ? 2 : 0,
          step: prop.value.prop_type === 'Float' ? (prop.value.step ? prop.value.step : 0.1) : 1,
          "onUpdate:value": (val: number) => {
            // @ts-ignore
            prop.value.min = val !== null ? val : 0;
            // @ts-ignore
            if (val > prop.value.max) {
              // @ts-ignore
              prop.value.max = val;
            }
          }
        }
      );
    default:
      return null;
  };
});

const maxInputField = computed(() => {
  switch (prop.value.prop_type) {
    case "Int":
    case "Float":
    case "Array":
    case "Select":
      return h(
        NInputNumber, {
          value: prop.value.max,
          defaultValue: prop.value.max ? prop.value.max : null,
          placeholder: ['Int', 'Float'].includes(prop.value.prop_type) ? "Input the minumum allowed number" : "Input the maximum number of values",
          precision: prop.value.prop_type === 'Float' ? 2 : 0,
          clearable: true,
          min: prop.value.min ? prop.value.min : (prop.value.prop_type != 'Select' ? 0.01 : 1),
          step: prop.value.prop_type === 'Float' ? (prop.value.step ? prop.value.step : 0.1) : 1,
          "onUpdate:value": (val: number) => {
            console.log('max-value:', val);
            // @ts-ignore
            prop.value.max = val;
          }
        }
      );
    default:
      return null;
  };
});

const stepInputField = computed(() => {
  switch (prop.value.prop_type) {
    case "Int":
    case "Float":
      return h(
        NInputNumber,
        {
          value: prop.value.step,
          defaultValue: prop.value.step ? prop.value.step : (prop.value.prop_type === "Float" ? 0.1 : 1.0),
          // @ts-ignore
          "onUpdate:value": (val: number) => prop.value.step = val,
          placeholder: "Input the step value",
          precision: prop.value.prop_type === 'Float' ? 2 : 0,
          clearable: true,
          step: prop.value.prop_type === 'Float' ? 0.01 : 1,
          min: prop.value.prop_type === 'Float' ? 0.01 : 1,
        }
      )
    default:
      return null;
  }
});
</script>

<style scoped>
.brick-prop-options {
  width: 100%;
}

.brick-prop-items {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.default-input-wrapper {
  width: 100%;
}
:deep(.n-input-number), :deep(.n-date-picker), :deep(.n-time-picker) {
  width: 100%;
}

:deep(.n-base-selection-input__content > div), :deep(.n-base-select-option__content > div) {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

:deep(.n-form-item.n-form-item--top-labelled) {
  /* Definiamo due colonne: la prima per la label, la seconda per l'input */
  grid-template-areas:
    "label blank"
    "label feedback"; /* Il feedback (errore) solitamente sta sotto l'input */
  
  /* Larghezza: es. 120px per la label e il resto all'input */
  grid-template-columns: 12rem 1fr; 
  
  /* Allineamento verticale al centro */
  align-items: center;
  
  /* Reset delle righe se necessario */
  grid-template-rows: auto auto;
  
  column-gap: 1rem; /* Spazio tra label e input */
}

@media (max-width: 800px) {
  :deep(.n-form-item.n-form-item--top-labelled) {
    grid-template-columns: 100% !important;
    grid-template-areas: "label" "blank" "feedback" !important;
  }
}

:deep(.n-form-item-label) {
  justify-content: flex-end;
  text-align: left;
  padding-right: 12px;
}
</style>