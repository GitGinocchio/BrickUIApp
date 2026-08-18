<template>
  <div class="space-y-4">
    <!-- Min Value / Items -->
    <UFormField :label="minLabel">
      <div class="max-w-fit">
        <div class="relative flex items-center">
          <UInputNumber 
            class="w-full" 
            v-model="model.min"
            :min="isNumericProp(model) ? undefined : 0"
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
            :max="isCollectionProp(model) ? 50 : undefined"
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

    <!-- Step Value -->
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

// Calcolo dinamico dello Step
const step = computed(() => {
  const p = model.value;

  if (p.prop_type === 'Float') return p.step ?? 0.01;
  if (p.prop_type === 'Int') return p.step ?? 1;

  if (isCollectionProp(model.value)) {
    switch (p.value_type) {
      case 'Float': return 0.1;
      case 'Int':
      case 'String': 
      default: return 1;
    }
  }

  return 1;
});

// Etichette dinamiche
const minLabel = computed(() => isNumericProp(model.value) ? 'Min value:' : 'Min items:');
const maxLabel = computed(() => isNumericProp(model.value) ? 'Max value:' : 'Max items:');

// Limiti incrociati per gli input numerici
const maxLimitForMin = computed(() => {
  // Gestione specifica per Collezioni (es. Array/Select con limite max 50)
  if (isCollectionProp(model.value) && model.value.min != null && model.value.min >= 50) {
    return 50;
  }

  if (model.value.max == null) return undefined;

  const maxVal = Number(model.value.max);

  if (isNumericProp(model.value)) {
    // Calcola lo step (0.01 per Float, 1 per Int)
    const step = model.value.step ?? (model.value.prop_type === 'Float' ? 0.01 : 1);
    
    // Arrotonda per evitare problemi di precisione dei float in JS (es. -10.000000000000002)
    return Number((maxVal - step).toFixed(6));
  }

  return maxVal;
});

const minLimitForMax = computed(() => {
  if (model.value.min == null) {
    // Per le collezioni la dimensione minima ha senso sia 0 o 1, 
    // ma per i numeri generici non c'è limite inferiore predefinito
    return isCollectionProp(model.value) ? 1 : undefined;
  }

  const minVal = Number(model.value.min);

  if (isCollectionProp(model.value)) {
    // Se è una collezione, la dimensione max deve essere almeno min o min + 1
    return Math.max(1, minVal);
  }

  if (isNumericProp(model.value)) {
    const step = model.value.step ?? (model.value.prop_type === 'Float' ? 0.01 : 1);
    
    // Arrotonda per evitare floating point drift
    return Number((minVal + step).toFixed(6));
  }

  return minVal;
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