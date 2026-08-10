<template>
  <UInput 
    v-model="targetValue"
    :default-value="prop.default"
    type="text"
  >
    <template v-if="targetValue?.length" #trailing>
      <UButton
        color="neutral"
        variant="link"
        size="sm"
        class="hidden hover:flex"
        icon="i-lucide-circle-x"
        aria-label="Clear input"
        @click="targetValue = (editMode ? '' : prop.default)"
      />
    </template>
  </UInput>
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