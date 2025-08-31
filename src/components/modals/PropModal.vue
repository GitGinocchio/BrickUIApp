<template>
  <n-modal
    v-model:show="show"
    :auto-focus="true"
    preset="dialog"
    :title="editMode ? 'Edit Prop' : 'New Prop'"
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
          @update:value="onNewPropTypeSelected"
          :render-label="renderLabel"
        />
      </n-form-item>
      
      <n-form-item v-if="optionsInputField" label="Options">
        <component :is="optionsInputField" />
      </n-form-item>
      <n-form-item v-if="prop.prop_type === 'Gradient' || prop.prop_type === 'Color'" label="Skip Alpha">
        <n-switch v-model:value="prop.skip_alpha" :default-value="false" @update:value="onSkipAlphaChanged"></n-switch>
      </n-form-item>
      <n-form-item v-if="defaultInputField" label="Default value">
        <component :is="defaultInputField" />
      </n-form-item>
      <n-form-item v-if="minInputField" :label="['Int', 'Float'].includes(prop.prop_type) ? 'Input the minimum allowed number' : 'Input the minimum number of values'">
        <component :is="minInputField" />
      </n-form-item>
      <n-form-item v-if="maxInputField" :label="['Int', 'Float'].includes(prop.prop_type) ? 'Input the maximum allowed number' : 'Input the maximum number of values'">
        <component :is="maxInputField" />
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
import { NColorPicker, NDynamicTags, NInput, NInputNumber, NSelect, NSwitch } from 'naive-ui';
import { Asterisk, Hash, List, ListOrdered, ListTodo, PaintBucket, SwatchBook, ToggleLeft, Type } from 'lucide-vue-next';
import { computed, h, ref } from 'vue';

import { createProp, Prop, PropTypeValue, propTypeValues } from '../../interfaces/brick';
import { colorStringToRGBA, deepEqual } from '../../utils';
import GradientPicker from '../GradientPicker.vue';


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
  
  Color: PaintBucket,
  Gradient: SwatchBook
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
  prop.value = createProp(value, prop.value.prop_name, prop.value.description);
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

//let valueBeforeSkipAlpha = null;

function onSkipAlphaChanged(skip_alpha: boolean) {
  /*
  if (skip_alpha) {
    valueBeforeSkipAlpha = prop.value.default;
  }
  */

  if (prop.value.prop_type === 'Color') {
    //prop.value.default = skip_alpha ? prop.value.default.slice(0, 7) + "FF" : valueBeforeSkipAlpha;
    prop.value.default = prop.value.default.slice(0, 7) + (skip_alpha ? "FF" : "CC");
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
    prop.value.default = prop.value.default.map((gradient_stop) => {
      return {
        'color': gradient_stop.color.slice(0, 7) + (skip_alpha ? "FF" : "CC"),
        'position': gradient_stop.position
      }
    });
  }
}

const defaultInputField = computed(() => {
  switch (prop.value.prop_type) {
    case "Any":
    case "String":
      return h(
        NInput,
        {
          value: prop.value.default,
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
    case "StringSelect":
    case "FloatSelect":
    case "IntSelect":
      return h(
        NSelect,
        {
          value: prop.value.default,
          clearable: true,
          options: (prop.value.options || []).map(v => ({
            label: String(v),
            value: v
          })),
          "onUpdate:value": (val: any) => (prop.value.default = val)
        }
      );

    case "Array":
    case "StringArray":
    case "FloatArray":
    case "IntArray":
      return h(
        NDynamicTags,
        {
          value: prop.value.default ? prop.value.default.map((value : string|number|any) => String(value)) : [],
          clearable: true,
          "onUpdate:value": (newValue: any[]) => {
            let parsedValue;

            if (prop.value.prop_type === 'StringArray') {
              parsedValue = newValue.map(item => typeof item === 'string' ? item : item.value);
            } 
            else if (prop.value.prop_type === 'IntArray') {
              parsedValue = newValue
                .map(item => {
                  const val = typeof item === 'string'
                    ? /[a-zA-Z]/.test(item) ? NaN : parseInt(item)
                    : item.value;
                  return isNaN(val) ? null : val;
                })
                .filter(val => val !== null);
            } 
            else if (prop.value.prop_type === 'FloatArray') {
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
          'showAlpha' : !prop.value.skip_alpha,
          clearable: true,
          actions: ['clear'],
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
      )

    default:
      return null
  }
});

const optionsInputField = computed(() => {
  switch (prop.value.prop_type) {
    case "Select":
    case "FloatSelect":
    case "IntSelect":
    case "StringSelect":
      return h(
        NDynamicTags, 
        {
          value: prop.value.options.map((value) => String(value)),
          "onUpdate:value": (newValue: any[]) => {
            let parsedValue;

            if (prop.value.prop_type === 'StringSelect') {
              parsedValue = newValue.map(item => typeof item === 'string' ? item : item.value);
            } 
            else if (prop.value.prop_type === 'IntSelect') {
              parsedValue = newValue
                .map(item => {
                  const val = typeof item === 'string'
                    ? /[a-zA-Z]/.test(item) ? NaN : parseInt(item)
                    : item.value;
                  return isNaN(val) ? null : val;
                })
                .filter(val => val !== null);
            } 
            else if (prop.value.prop_type === 'FloatSelect') {
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
          }
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
    case "StringArray":
    case "IntArray":
    case "FloatArray":
    case "Select":
    case "StringSelect":
    case "IntSelect":
    case "FloatSelect":
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
    case "StringArray":
    case "IntArray":
    case "FloatArray":
    case "Select":
    case "StringSelect":
    case "IntSelect":
    case "FloatSelect":
      return h(
        NInputNumber, {
          value: prop.value.max,
          defaultValue: prop.value.max ? prop.value.max : null,
          placeholder: ['Int', 'Float'].includes(prop.value.prop_type) ? "Input the minumum allowed number" : "Input the maximum number of values",
          precision: prop.value.prop_type === 'Float' ? 2 : 0,
          clearable: true,
          min: prop.value.min ? prop.value.min : 1,
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
</script>