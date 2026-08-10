<template>
  <div class="max-w-fit">
    <div class="relative flex items-center">
      <UInputNumber 
        v-model="targetValue"
        :step="prop.step || 0.1"
        :min="prop.min"
        :max="prop.max"
        class="w-full"
        orientation="vertical"
      />
      <div 
        v-if="targetValue != (props.editMode ? undefined : prop.default)"
        class="absolute right-8 flex items-center justify-center"
      >
        <UButton 
          icon="i-lucide-circle-x" 
          variant="link" 
          color="primary"
          size="xs"
          @click.stop="targetValue = (props.editMode ? undefined : prop.default)" 
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { NumericPropType } from '~/interfaces/brick';

const prop = defineModel<NumericPropType<'Int' | 'Float'>>("prop");

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