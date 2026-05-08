<template>
  <UFormField :label="minLabel">
    <div class="max-w-fit">
      <div class="relative flex items-center">
        <UInputNumber 
          class="w-full" 
          v-model="prop.min"
          :max="prop.max-step"
          :step="step"
          orientation="vertical"
          @update:model-value="(value) => onMinUpdate(value)" 
        />
        <div 
          v-if="prop.min != undefined"
          class="absolute right-8 flex items-center justify-center"
        >
          <UButton 
            icon="i-lucide-circle-x" 
            variant="link" 
            color="primary"
            size="xs"
            @click.stop="prop.min = undefined" 
          />
        </div>
      </div>
    </div>

  </UFormField>
  <UFormField :label="maxLabel">
    <div class="max-w-fit">
      <div class="relative flex items-center">
        <UInputNumber 
          class="w-full"
          :min="prop.min+step"
          :step="step"
          v-model="prop.max"
          orientation="vertical"
          @update:model-value="(value) => onMaxUpdate(value)" 
        />
        <div
          v-if="prop.max != undefined"
          class="absolute right-8 flex items-center justify-center"
        >
          <UButton 
            icon="i-lucide-circle-x" 
            variant="link" 
            color="primary"
            size="xs"
            @click.stop="prop.max = undefined" 
          />
        </div>
      </div>
    </div>
  </UFormField>
</template>

<script lang="ts" setup>
import type { ArrayPropType, NumericPropType, SelectablePropType } from '~/interfaces/brick';

const prop = defineModel<SelectablePropType<number|string> | ArrayPropType<number|string> | NumericPropType<'Int'|'Float'>>("prop");

const step = computed(() => {
  switch (prop.value.prop_type) {
    case 'Float':
      return 0.1
    case 'Int':
      return 1
    case 'Array':
    case 'Select':
      break;
  }

  switch (prop.value.value_type) {
    case 'Float':
      return 0.1
    case 'Integer':
    case 'String':
      return 1
  }
})

const minLabel = computed(() => 
  ['Int', 'Float'].includes(prop.value.prop_type) ? 'Min value:' : 'Min items:'
);

const maxLabel = computed(() => 
  ['Int', 'Float'].includes(prop.value.prop_type) ? 'Max value:' : 'Max items:'
);

function onMaxUpdate(max: number) {
  const p = prop.value;

  if (Array.isArray(p.value) && p.value.length > max) {
    p.value = p.value.slice(0, max);
    p.default = p.value;
  } else if (typeof p.value === 'number' && p.value > max) {
    p.value = max;
    p.default = p.value;
  }
}

function onMinUpdate(min: number) {
  const p = prop.value;

  if (typeof p.value === 'number' && p.value < min) {
    p.value = min;
    p.default = p.value;
  }

  // Nel caso di select o array non fa niente perche' andrebbe invalidato il form...
}
</script>