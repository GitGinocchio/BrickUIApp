<template>
  <USelectMenu
    v-model:model-value="targetValue"
    :default-value="prop.default"
    :items="prop.options"
    :multiple="prop.max && prop.max  > 1"
    :required="prop.min && prop.min >= 0"
    class="w-full"
    clear
  />
</template>

<script lang="ts" setup>
import type { SelectPropType } from '~/interfaces';

const prop = defineModel<SelectPropType>("prop");
const props = defineProps({
  editMode: {
    type: Boolean,
    default: false
  }
});

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