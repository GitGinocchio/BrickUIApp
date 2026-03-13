<template>
  <n-modal
    v-model:show="show"
    :auto-focus="true"
    preset="dialog"
    :title="editMode ? 'Edit Prop' : 'New Prop'"
    :draggable="true"
    @keyup.enter="onFinished(false)"
  >
    <n-form>
      <n-form-item 
        label="Prop name"
        :validation-status="feedback ? 'error' : undefined"
        :show-feedback="feedback ? true : false"
        :feedback="feedback"
      >
        <n-input
          :value="prop.prop_name"
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
          v-model:value="prop.description"
          :default-value="prop.description"
          @update:value="(value) => prop.description = value"
          placeholder="Type the prop's description"
          type="textarea"
        />
      </n-form-item>
      <n-form-item 
        label="Prop type"
      >
        <n-select 
          v-model:value="prop.prop_type" 
          :options="propTypes"
          @update:value="(value) => {
            onNewPropTypeSelected(value)
            if (prop.prop_type === 'Select' || prop.prop_type === 'Array') prop.value_type = 'String'
          }"
          :render-label="renderLabel"
        />
      </n-form-item>
      <n-form-item 
        label="Value type"
        v-if="prop.prop_type === 'Select' || prop.prop_type === 'Array'"
      >
        <n-select
          v-model:value="prop.value_type"
          :options="[
            { label: 'String', value: 'String', icon: h(Type, { size: 16 }) }, 
            { label: 'Float', value: 'Float', icon: h(DecimalsArrowRight, { size: 16 }) },
            { label: 'Integer', value: 'Integer', icon: h(ArrowUp10, { size: 16 }) },
          ]"
          :render-label="(option: { label: string, value: string, icon: any }) => {
            return h('div', { style: 'display: flex; align-items: center; gap: 6px;' }, [
              option.icon,
              h('span', option.label)
            ]);
          }"
          @update:value="() => { 
            if (prop.prop_type === 'Select') {
              prop.options = []
            } else if (prop.prop_type === 'Array') {
              prop.default = [] 
            }
          }"
        />
      </n-form-item>
      
      <n-form-item v-if="optionsInputField" label="Options">
        <component :is="optionsInputField" />
      </n-form-item>
      <n-form-item v-if="prop.prop_type === 'Gradient' || prop.prop_type === 'Color'" label="Skip Alpha">
        <n-switch v-model:value="prop.skip_alpha" :default-value="false" @update:value="onSkipAlphaChanged"></n-switch>
      </n-form-item>
      <n-form-item v-if="defaultInputField" label="Default value">
        <component :is="defaultInputField">
          <template v-if="prop.prop_type == 'Color'" #action>
            <n-tooltip trigger="hover" placement="bottom" :delay="500">
              <template #trigger>
                <n-button size="small" @click="onSaveColor">Save</n-button>
              </template>
              Clicca salva per creare un campione di colore
            </n-tooltip>
            <n-tooltip trigger="hover" placement="bottom" :delay="500">
              <template #trigger>
                <n-button size="small" @click="onRemoveColor">Remove</n-button>
              </template>
              Clicca rimuovi per eliminare un campione di colore
            </n-tooltip>
            <n-tooltip trigger="hover" placement="bottom" :delay="500">
              <template #trigger>
                <n-button size="small" @click="onClearColor">Clear</n-button>
              </template>
              Clicca pulisci per rimuovere il colore di default
            </n-tooltip>
          </template>
        </component>
      </n-form-item>
      <n-form-item v-if="minInputField" :label="['Int', 'Float'].includes(prop.prop_type) ? 'Minimum allowed number' : 'Minimum number of values'">
        <component :is="minInputField" />
      </n-form-item>
      <n-form-item v-if="maxInputField" :label="['Int', 'Float'].includes(prop.prop_type) ? 'Maximum allowed number' : 'Maximum number of values'">
        <component :is="maxInputField" />
      </n-form-item>
      <n-form-item v-if="stepInputField" label="Step value">
        <component :is="stepInputField" />
      </n-form-item>
      <n-form-item label="Allow past" v-if="prop.prop_type === 'Date' || prop.prop_type === 'Datetime'">
        <n-switch
          :value="prop.allow_past"
          v-on:update:value="(value: boolean) => {
            if (prop.prop_type !== 'Date' && prop.prop_type !== 'Datetime') return;
            
            prop.allow_past = value;

            if (prop.allow_future === prop.allow_past && prop.allow_future === false) {
              prop.allow_future = !prop.allow_past
            }
          }"
        />
      </n-form-item>
      <n-form-item label="Allow future" v-if="prop.prop_type === 'Date' || prop.prop_type === 'Datetime'">
        <n-switch
          :value="prop.allow_future"
          v-on:update:value="(value: boolean) => {
            if (prop.prop_type !== 'Date' && prop.prop_type !== 'Datetime') return;
            
            prop.allow_future = value;

            if (prop.allow_future === prop.allow_past && prop.allow_future === false) {
              prop.allow_past = !prop.allow_future
            }
          }"
        />
      </n-form-item>
    </n-form>
    <template #action>
      <n-space justify="end">
        <n-button
          v-if="editMode"
          type="secondary"
          @click="onFinished(true)"
        >
        Duplicate
        </n-button>
        <n-button @click="show = !show">Cancel</n-button>
        <n-button
          type="primary"
          :disabled="editMode ? deepEqual(initialProp, prop) || feedback != null : feedback || !prop.prop_name ? true : false"
          @click="onFinished(false)"
        >   
          {{ editMode ? 'Save' : 'Create' }}
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { NColorPicker, NDatePicker, NDynamicTags, NInput, NInputNumber, NSelect, NSwitch, NTimePicker, NTooltip } from 'naive-ui';
import { List, ListTodo, PaintBucket, SwatchBook, ToggleLeft, Type, Text, DecimalsArrowRight, ArrowUp10, Clock, Calendar1, CalendarClock } from 'lucide-vue-next';
import { computed, h, ref } from 'vue';

import { createProp, Prop, PropTypeValue, propTypeValues } from '../../interfaces/brick';
import { colorStringToRGBA } from '../../utils/color';
import { deepEqual } from '../../utils/misc';
import GradientPicker from '../GradientPicker.vue';
import { DatePickerType } from 'naive-ui/es/date-picker/src/config';


const show = defineModel<boolean>("show");
const prop = defineModel<Prop>("prop");
const initialProp = defineModel<Prop>("initialProp");

const emit = defineEmits<{
  (e: "finished", before: Prop, clone?: boolean): void
}>();

const props = defineProps({
  editMode: {
    type: Boolean,
    default: false,
    required: false
  },
  props: {
    type: Array<Prop>,
    required: true
  }
});

const feedback = ref<string|null>(null);

const iconsMap: Record<string, any> = {
  Select: ListTodo,
  Array: List,
  String: Type,
  Text: Text,
  Int: ArrowUp10,
  Float: DecimalsArrowRight,
  Bool: ToggleLeft,

  Color: PaintBucket,
  Gradient: SwatchBook,
  Datetime: CalendarClock,
  Date: Calendar1,
  Time: Clock
};

const propTypes = propTypeValues.map((value) => ({
  label: value.replace(/([A-Z])/g, " $1").trim(),
  value,
  icon: () => h(iconsMap[value], { size: 16 })
}));

function renderLabel(option) {
  return h("div", { style: "display: flex; align-items: center; gap: 6px;" }, [
    option.icon && option.icon(),
    h("span", option.label)
  ]);
}

function onNewPropTypeSelected(value: PropTypeValue) {
  prop.value = createProp(
    value, 
    prop.value.prop_name, 
    prop.value.description, 
    (prop.value.prop_type === 'Select' || prop.value.prop_type === 'Array' ? prop.value.value_type ?? 'String' : null)
  );
}

function onPropNameInput(value: string) {
  const componentNameRegex = /^[A-Za-z][A-Za-z0-9]*(?:[-_][A-Za-z0-9]+)*$/;
  const other_props = initialProp.value ? props.props.filter((p) => p.prop_name !== initialProp.value.prop_name) : props.props;

  if (!componentNameRegex.test(value)) {
    feedback.value = "Must start with a letter. Only letters, numbers, and underscores are allowed.";
  } else if (other_props.some((p) => p.prop_name === value)) {
    feedback.value = "A prop with this name already exists.";
  } else {
    feedback.value = null;
  }

  prop.value.prop_name = value;
}

function onFinished(clone: boolean) {
  show.value = false; 

  emit('finished', initialProp.value, clone)
}

function onClearColor() {
  prop.value.default = null;
}

function onSaveColor() {
  if (prop.value.prop_type !== 'Color') return;

  // Se il colore è già nei swatches, non fare nulla
  if (prop.value.swatches && prop.value.swatches.includes(prop.value.default)) return;

  if (prop.value.swatches) prop.value.swatches = [prop.value.default, ...(prop.value.swatches || [])];
  else prop.value.swatches = [prop.value.default]
}

function onRemoveColor() {
  if (prop.value.prop_type !== 'Color') return;

  if (prop.value.swatches && prop.value.swatches.includes(prop.value.default)) {
    prop.value.swatches = prop.value.swatches.filter((value) => value !== prop.value.default);
  }
}

//let valueBeforeSkipAlpha = null;

function onSkipAlphaChanged(skip_alpha: boolean) {
  /*
  if (skip_alpha) {
    valueBeforeSkipAlpha = prop.value.default;
  }
  */

  if (prop.value.prop_type === 'Color') {
    //prop.value.default = skip_alpha ? prop.value.default.slice(0, 7) + "FF" : valueBeforeSkipAlpha;
    prop.value.default = prop.value.default?.slice(0, 7) + (skip_alpha ? "FF" : "CC");
  }
  else if (prop.value.prop_type === 'Gradient') {
    /*
    prop.value.default = skip_alpha ? prop.value.default.map((gradient_stop) => {
      return {
        'color': gradient_stop.color.slice(0, 7) + (skip_alpha ? "FF" : "CC"),
        'position': gradient_stop.position
      }
    }) : valueBeforeSkipAlpha;
    */
    prop.value.default = prop.value.default?.map((gradient_stop) => {
      return {
        'color': gradient_stop.color.slice(0, 7) + (skip_alpha ? "FF" : "CC"),
        'position': gradient_stop.position
      }
    });
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
          value: prop.value.default as any,
          clearable: true,
          multiple: prop.value.max > 1,
          options: (prop.value.options || []).map(v => ({
            label: String(v),
            value: v
          })),
          "onUpdate:value": (val: any) => (prop.value.default = val)
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
      const value_type = prop.value.value_type;
      return h(
        NDynamicTags, 
        {
          value: prop.value.options.map((value) => String(value)),
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
            prop.value.options = [...new Set(parsedValue)];

            // @ts-ignore
            if (!prop.value.options.includes(prop.value.default)) {
              prop.value.default = null;
            }
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
.n-input-number,
.n-date-picker,
.n-time-picker {
  width: 100%;
}
</style>