<template>
  <GradientPicker
    v-model:value="prop.value"
    :default="prop.default"
    :skip_alpha="prop.skip_alpha"
    :type="prop.type"
  />
</template>

<script setup lang="ts">
import GradientPicker from '~/components/inputs/GradientPicker.vue';
import type { GradientPropType } from '~/interfaces/brick';

const prop = defineModel<GradientPropType>("prop");
</script>