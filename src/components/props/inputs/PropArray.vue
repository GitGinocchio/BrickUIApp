<template>
  <Array
    v-model:value="targetValue"
    :default="prop.default"
    :value_type="prop.value_type"
    :min_value="prop.min_value"
    :max_value="prop.max_value"
    :min="prop.min"
    :max="prop.max"
  />
</template>

<script setup lang="ts">
import Array from '~/components/inputs/Array.vue';
import type { ArrayPropType } from '~/interfaces/brick';

const prop = defineModel<ArrayPropType<string|number>>("prop", { required: true });

const props = defineProps({
  defaultMode: {
    type: Boolean,
    default: false
  }
})

const targetValue = computed({
  get: () => props.defaultMode ? prop.value.default : prop.value.value,
  set: (newValue) => {
    if (props.defaultMode) {
      prop.value.default = newValue;
    } else {
      prop.value.value = newValue;
    }
  }
});
</script>