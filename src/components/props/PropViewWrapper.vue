<template>
  <div class="brick-prop-container w-full">
    <component 
      :is="renderComponent"
      v-model:prop="prop"
      v-bind="dynamicProps"
    />
  </div>
</template>

<script setup lang="ts">
import type { Prop as BrickPropType } from '~/interfaces/brick';

const { updateBrickProp } = useBrickActions();

const prop = defineModel<BrickPropType>("prop", { required: true });

const props = defineProps({
  brick_name: {
    type: String,
    required: true
  }
})

const renderComponent = computed(() => {
  switch (prop.value.prop_type) {
    case 'Color': return defineAsyncComponent(() => import('./inputs/PropColorPicker.vue'));
    case 'Gradient': return defineAsyncComponent(() => import('./inputs/PropGradientPicker.vue'));
    case 'Array': return defineAsyncComponent(() => import('./inputs/PropArray.vue'));
    case 'Select': return defineAsyncComponent(() => import('./inputs/PropSelect.vue'));
    case 'Bool': return defineAsyncComponent(() => import('./inputs/PropSwitch.vue'));
    default: return defineAsyncComponent(() => import('./inputs/PropInput.vue'));
  }
});

const dynamicProps = computed(() => {
  return {};
});

watch(() => prop.value.value, (newVal, oldVal) => {
  if (JSON.stringify(newVal) === JSON.stringify(oldVal)) return;
  updateBrickProp(props.brick_name, prop.value);
}, { deep: true })
</script>