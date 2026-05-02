<template>
  <div class="brick-prop-container w-full">
    <component 
      :is="renderComponent"
      v-model="modelValue"
      :prop="prop"
      v-bind="dynamicProps"
    />
  </div>
</template>

<script setup lang="ts">
import type { Prop as BrickPropType } from '~/interfaces/brick';

const props = defineProps<{
  prop: BrickPropType
}>();

const emit = defineEmits(['update:prop']);

// Mapping dei wrapper custom
const renderComponent = computed(() => {
  switch (props.prop.prop_type) {
    case 'Color': return defineAsyncComponent(() => import('./PropColorPicker.vue'));
    case 'Gradient': return defineAsyncComponent(() => import('./PropGradientPicker.vue'));
    case 'Array': return defineAsyncComponent(() => import('./PropTagInput.vue'));
    case 'Select': return defineAsyncComponent(() => import('./PropSelect.vue'));
    case 'Bool': return defineAsyncComponent(() => import('@nuxt/ui/components/Switch.vue'));
    default: return defineAsyncComponent(() => import('@nuxt/ui/components/Input.vue'));
  }
});

// Getter/Setter centralizzato (la tua logica originale ottimizzata)
const modelValue = computed<any>({
  get() {
    const p = props.prop;
    switch (p.prop_type) {
      case 'Bool': return p.value ?? p.default ?? false;
      case 'Select':
        return p.max <= 1 ? (p.value?.[0] ?? p.default?.[0] ?? null) : (p.value ?? p.default ?? []);
      case 'Array':
        return (p.value ?? p.default ?? []).map((v: any) => ({ label: String(v), value: v }));
      case 'Color': return p.value ?? p.default ?? '#00000000';
      case 'Gradient': return p.value ?? p.default ?? [];
      case 'Date': case 'Datetime': case 'Time': return p.value ?? p.default ?? null;
      default: return p.value ?? p.default ?? null;
    }
  },
  set(newValue) {
    const p = props.prop;
    let finalValue: any;

    switch (p.prop_type) {
      case 'Array':
        finalValue = newValue ? newValue.map((item: any) => (typeof item === 'string' ? item : item.value)) : [];
        if (p.value_type === 'Integer') finalValue = finalValue.map((v: any) => parseInt(v)).filter((v: any) => !isNaN(v));
        break;
      case 'Select':
        finalValue = Array.isArray(newValue) ? newValue : (newValue ? [newValue] : []);
        break;
      case 'Color':
        finalValue = newValue ? colorStringToRGBA(newValue) : "#00000000";
        break;
      default:
        finalValue = newValue;
    }

    p.value = finalValue;
    emit('update:prop', p);
  }
});

const dynamicProps = computed(() => {
  // Qui passiamo le props specifiche di Nuxt UI (es. type="number" per Int)
  const type = props.prop.prop_type;
  if (type === 'Int' || type === 'Float') return { type: 'number', step: props.prop.step || 1 };
  if (type === 'Text') return { type: 'textarea' };
  return {};
});
</script>