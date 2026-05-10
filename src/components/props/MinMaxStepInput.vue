<template>
  <div>
    <UFormField :label="minLabel">
      <div class="max-w-fit">
        <div class="relative flex items-center">
          <UInputNumber 
            class="w-full" 
            v-model="prop.min"
            :max="prop.max != undefined ? prop.max-step : undefined"
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
            :min="prop.min != undefined ? prop.min+step : undefined"
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
    <UFormField label="Step value:" v-if="prop.prop_type == 'Int' || prop.prop_type == 'Float'">
      <div class="max-w-fit">
        <div class="relative flex items-center">
          <UInputNumber 
            class="w-full"
            :min="prop.prop_type == 'Int' ? 1 : 0.01"
            :step="prop.prop_type == 'Int' ? 1 : 0.01"
            :max="prop.min || undefined"
            :default-value="step"
            v-model="prop.step"
            orientation="vertical"
          />
          <div
            v-if="prop.step != undefined"
            class="absolute right-8 flex items-center justify-center"
          >
            <UButton 
              icon="i-lucide-circle-x" 
              variant="link" 
              color="primary"
              size="xs"
              @click.stop="prop.step = undefined" 
            />
          </div>
        </div>
      </div>
    </UFormField>
  </div>
</template>

<script lang="ts" setup>
import type { ArrayPropType, NumericPropType, SelectablePropType } from '~/interfaces/brick';

const prop = defineModel<SelectablePropType<number|string> | ArrayPropType<number|string> | NumericPropType<'Int'|'Float'>>("prop");

const step = computed(() => {
  switch (prop.value.prop_type) {
    case 'Float':
      return prop.value.step || 0.01
    case 'Int':
      return prop.value.step || 1
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

  switch (p.prop_type) {
    case 'Float':
    case 'Int':
      if (p.value > max) {
        p.value = max;
      }
      if (p.default > max) {
        p.default = max;
      }
      break;
    case 'Array':
    case 'Select':
      if (p.value.length > max) {
        p.value = p.value.slice(0, max);
      }
      if (p.default.length > max) {
        p.default = p.default.slice(0, max);
      }
      break;

  }
}

function onMinUpdate(min: number) {
  const p = prop.value;

  switch (p.prop_type) {
    case 'Float':
    case 'Int':
      if (typeof p.value === 'number' && p.value < min) {
        p.value = min;
      }
      
      if (typeof p.default === 'number' && p.default < min) {
        p.default = min;
      }
  }

  // Nel caso di select o array non fa niente perche' andrebbe invalidato il form...
}
</script>