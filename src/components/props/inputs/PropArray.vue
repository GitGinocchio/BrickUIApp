<template>
  <Array
    v-model:value="prop.value"
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
</script>