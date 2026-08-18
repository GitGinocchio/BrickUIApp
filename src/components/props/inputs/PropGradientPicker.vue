<template>
  <GradientPicker
    v-model:value="targetValue"
    :default="prop.default"
    :skip_alpha="prop.skip_alpha"
    :type="prop.type"
  />
</template>

<script setup lang="ts">
import GradientPicker from '~/components/inputs/GradientPicker.vue';
import type { GradientPropType } from '~/interfaces';

const prop = defineModel<GradientPropType>("prop");

const props = defineProps({
  editMode: {
    type: Boolean,
    default: false
  }
})

const targetValue = computed({
  get: () => props.editMode ? prop.value.default : prop.value.value,
  set: (newValue) => {
    if (props.editMode) {
      prop.value.default = newValue;
    } else {
      prop.value.value = newValue;
    }
  }
});
</script>