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
import type { Prop } from '~/interfaces/brick';

const { updateBrickProp } = useBrickActions();

const prop = defineModel<Prop>("prop", { required: true });

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
    case 'Int': return defineAsyncComponent(() => import('./inputs/PropInt.vue'));
    case 'Float': return defineAsyncComponent(() => import('./inputs/PropFloat.vue'));
    default: return defineAsyncComponent(() => import('./inputs/PropString.vue'));
  }
});

const dynamicProps = computed(() => {
  return {};
});

const emit = defineEmits<{
  (e: 'save'): void
}>()

watch(() => prop.value.value, () => {
  updateBrickProp(props.brick_name, prop.value);
  emit("save");
}, { deep: true })
</script>