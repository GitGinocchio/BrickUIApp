<template>
  <div class="flex flex-row gap-3 items-center">
    <USwitch
      v-model="targetValue"
      :default-value="prop.default"
    />
    <UButton
      v-if="!editMode && prop.value != prop.default"
      class="invisible group-hover:visible transition-all h-5"
      label="Default"
      variant="ghost"
      size="xs"
      icon="i-lucide-rotate-ccw"
      @click="prop.value = prop.default"
    />
  </div>
</template>

<script lang="ts" setup>
import type { Prop } from '~/interfaces/brick';

const prop = defineModel<Prop>("prop");
const props = defineProps({
  editMode: {
    type: Boolean,
    default: false
  }
})

const targetValue = computed<string>({
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

<style>

</style>