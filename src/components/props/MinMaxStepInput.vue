<template>
  <div class="space-y-4">
    <!-- Min Value / Items -->
    <UFormField :label="minLabel">
      <div class="max-w-fit">
        <div class="relative flex items-center">
          <UInputNumber 
            class="w-full" 
            v-model="model.min"
            :max="maxLimitForMin"
            :step="step"
            orientation="vertical"
            @update:model-value="onMinUpdate" 
          />
          <div 
            v-if="model.min != null"
            class="absolute right-8 flex items-center justify-center"
          >
            <UButton 
              icon="i-lucide-circle-x" 
              variant="link" 
              color="primary"
              size="xs"
              @click.stop="model.min = null" 
            />
          </div>
        </div>
      </div>
    </UFormField>

    <!-- Max Value / Items -->
    <UFormField :label="maxLabel">
      <div class="max-w-fit">
        <div class="relative flex items-center">
          <UInputNumber 
            class="w-full"
            :min="minLimitForMax"
            :step="step"
            v-model="model.max"
            orientation="vertical"
            @update:model-value="onMaxUpdate" 
          />
          <div
            v-if="model.max != null"
            class="absolute right-8 flex items-center justify-center"
          >
            <UButton 
              icon="i-lucide-circle-x" 
              variant="link" 
              color="primary"
              size="xs"
              @click.stop="model.max = null" 
            />
          </div>
        </div>
      </div>
    </UFormField>

    <UFormField label="Step value:" v-if="model.prop_type === 'Int' || model.prop_type === 'Float'">
      <div class="max-w-fit">
        <div class="relative flex items-center">
          <UInputNumber 
            class="w-full"
            :min="model.prop_type === 'Int' ? 1 : 0.01"
            :step="model.prop_type === 'Int' ? 1 : 0.01"
            :default-value="step"
            v-model="model.step"
            orientation="vertical"
          />
          <div
            v-if="model.step != null"
            class="absolute right-8 flex items-center justify-center"
          >
            <UButton
              icon="i-lucide-circle-x" 
              variant="link" 
              color="primary"
              size="xs"
              @click.stop="model.step = null" 
            />
          </div>
        </div>
      </div>
    </UFormField>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import type { NumericOrCollectionProp } from '~/interfaces';

const model = defineModel<NumericOrCollectionProp>('prop', { required: true });

const isNumericProp = computed(() =>
  model.value.prop_type === 'Int' || model.value.prop_type === 'Float'
);

// Calcolo dinamico dello Step
const step = computed(() => {
  const p = model.value;

  if (p.prop_type === 'Float') return p.step ?? 0.01;
  if (p.prop_type === 'Int') return p.step ?? 1;

  // Caso Array o Select
  if ('value_type' in p) {
    switch (p.value_type) {
      case 'Float': return 0.1;
      case 'Integer':
      case 'String': 
      default: return 1;
    }
  }

  return 1;
});

// Etichette dinamiche
const minLabel = computed(() => isNumericProp.value ? 'Min value:' : 'Min items:');
const maxLabel = computed(() => isNumericProp.value ? 'Max value:' : 'Max items:');

// Limiti incrociati per gli input numerici
const maxLimitForMin = computed(() => {
  if (model.value.max != null) {
    return Number(model.value.max) - step.value;
  }
  return undefined;
});

const minLimitForMax = computed(() => {
  if (model.value.min != null) {
    return Number(model.value.min) + step.value;
  }
  return undefined;
});

// Handler aggiornamento Max
function onMaxUpdate(max: number | null) {
  if (max == null) return;
  const p = model.value;

  if (p.prop_type === 'Float' || p.prop_type === 'Int') {
    if (typeof p.value === 'number' && p.value > max) {
      p.value = max;
    }
    if (typeof p.default === 'number' && p.default > max) {
      p.default = max;
    }
  } else if (p.prop_type === 'Array' || p.prop_type === 'Select') {
    if (Array.isArray(p.value) && p.value.length > max) {
      p.value = p.value.slice(0, max);
    }
    if (Array.isArray(p.default) && p.default.length > max) {
      p.default = p.default.slice(0, max);
    }
  }
}

// Handler aggiornamento Min
function onMinUpdate(min: number | null) {
  if (min == null) return;
  const p = model.value;

  if (p.prop_type === 'Float' || p.prop_type === 'Int') {
    if (typeof p.value === 'number' && p.value < min) {
      p.value = min;
    }
    if (typeof p.default === 'number' && p.default < min) {
      p.default = min;
    }
  }
}
</script>