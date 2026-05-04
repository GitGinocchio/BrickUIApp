<template>
  <ColorPicker
    v-model:color="prop.value"
    v-model:saved="prop.saved"
    :swatches="prop.swatches"
    :default="prop.default"
    :skip_alpha="prop.skip_alpha"
    format="hex"
  />
</template>

<script setup lang="ts">
import ColorPicker from '~/components/inputs/ColorPicker.vue';
import type { ColorPropType } from '~/interfaces/brick';

const prop = defineModel<ColorPropType>("prop");
</script>