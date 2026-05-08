<template>
  <ColorPicker
    v-model:color="colorTargetValue"
    v-model:saved="savedTargetValue"
    :swatches="swatchesTargetValue"
    :default="prop.default"
    :skip_alpha="prop.skip_alpha"
    :enable_swatches="!editMode"
    format="hex"
  />
</template>

<script setup lang="ts">
import ColorPicker from '~/components/inputs/ColorPicker.vue';
import type { ColorPropType } from '~/interfaces/brick';

const prop = defineModel<ColorPropType>("prop");

const props = defineProps({
  editMode: {
    type: Boolean,
    default: false
  }
})

const colorTargetValue = computed({
  get: () => props.editMode ? prop.value.default : prop.value.value,
  set: (newValue) => {
    if (props.editMode) {
      prop.value.default = newValue;
    } else {
      prop.value.value = newValue;
    }
  }
});

// In modalita' edit i saved sono gli swatches 
const savedTargetValue = computed({
  get: () => props.editMode ? prop.value.swatches : prop.value.saved,
  set: (newValue) => {
    if (props.editMode) {
      prop.value.swatches = newValue;
    } else {
      prop.value.saved = newValue;
    }
  }
});

// ... e gli swatches invece non servono
const swatchesTargetValue = computed({
  get: () => props.editMode ? null : prop.value.swatches,
  set: (newValue) => {
    if (!props.editMode) {
      prop.value.swatches = newValue;
    }
  }
});
</script>