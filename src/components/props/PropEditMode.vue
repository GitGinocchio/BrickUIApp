<template>
  <div class="flex flex-col w-full space-y-6">
    <UFormField label="Description:">
      <UTextarea
        v-model="prop.description"
        placeholder="Type the prop's description (Markdown supported)"
        class="w-full"
        autoresize
        :maxlength="512"
        help="Markdown supported"
      />
    </UFormField>

    <UFormField label="Type:">
      <USelectMenu
        v-model="propTypeOption"
        class="w-full"
        :icon="(propTypeOption as any).icon"
        :items="propTypeOptions"
        @update:model-value="onNewPropTypeSelected"
      />
    </UFormField>

    <UFormField
      v-if="prop.prop_type == 'Select' || prop.prop_type == 'Array'"
      label="Value type"
    >
      <USelectMenu
        v-model="prop.value_type"
        :options="valueTypeOptions"
        @update:model-value="onValueTypeChanged"
      >
        <template #item="{ item }">
          <UIcon :name="(item as any).icon" class="w-4 h-4" />
          <span>{{ (item as any).label }}</span>
        </template>
      </USelectMenu>
    </UFormField>

    <UFormField v-if="prop.prop_type === 'Select'" label="Options:">
      <Array v-model:value="prop.options" />
    </UFormField>

    <div class="grid grid-cols-3 gap-4">
      <UFormField v-if="shouldShowDefaultInput" label="Default value:">
        <component :is="renderDefaultComponent" class="w-full" :prop="prop" :editMode="true" />
      </UFormField>
  
      <PropMinMaxInput 
        v-if="prop.prop_type == 'Array' 
           || prop.prop_type == 'Select'
           || prop.prop_type == 'Int'
           || prop.prop_type == 'Float'"
        v-model:prop="prop"
      />
    </div>

    <div v-if="['Date', 'Datetime'].includes(prop.prop_type)" class="flex gap-4">
      <UFormField label="Allow past:">
        <USwitch v-model="(prop as DatePropType<any>).allow_past" @update:model-value="v => toggleDateConstraint('past', v)" />
      </UFormField>
      <UFormField label="Allow future:">
        <USwitch v-model="(prop as DatePropType<any>).allow_future" @update:model-value="v => toggleDateConstraint('future', v)" />
      </UFormField>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type Prop, createProp, type PropTypeValue, propTypeValues, type DatePropType } from '#interfaces/brick';
import type { SelectMenuItem } from '@nuxt/ui';
import Array from '../inputs/Array.vue';
import PropMinMaxInput from './PropMinMaxInput.vue';

// --- Props & Emits ---
const prop = defineModel<Prop>("prop", { required: true });
const props = defineProps<{
  allProps: Prop[],
  showAlertOnTypeChange?: boolean
}>();

const renderDefaultComponent = computed(() => {
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

// --- Mapping Icone ---
const iconsMap: Record<PropTypeValue, string> = {
  Select: 'i-lucide-list-todo',
  Array: 'i-lucide-list',
  String: 'i-lucide-type',
  Text: 'i-lucide-text',
  Int: 'i-lucide-arrow-up-1-0',
  Float: 'i-lucide-decimals-arrow-right',
  Bool: 'i-lucide-toggle-left',
  Color: 'i-lucide-paint-bucket',
  Gradient: 'i-lucide-swatch-book',
  Datetime: 'i-lucide-calendar-clock',
  Date: 'i-lucide-calendar-1',
  Time: 'i-lucide-clock',
  Null: 'i-lucide-circle-slash',
  Deprecated: 'i-lucide-triangle-alert',
  Unknown: 'i-lucide-circle-question-mark'
};

// --- Computed ---
function asOption(v: PropTypeValue): SelectMenuItem {
  return {
    label: v.replace(/([A-Z])/g, " $1").trim(),
    type: 'item',
    value: v,
    icon: iconsMap[v] || 'i-lucide-circle'
  }
}
const propTypeOption = computed(() => asOption(prop.value.prop_type));
const propTypeOptions: SelectMenuItem[] = propTypeValues
  .filter((v) => !['Deprecated', 'Unknown'].includes(v))
  .map((v) => asOption(v));

const valueTypeOptions = [
  { label: 'String', value: 'String', icon: 'i-lucide-type' },
  { label: 'Float', value: 'Float', icon: 'i-lucide-decimals-arrow-right' },
  { label: 'Integer', value: 'Integer', icon: 'i-lucide-arrow-up-1-0' },
];

const shouldShowDefaultInput = computed(() => prop.value.prop_type !== 'Null');

// --- Methods ---
function onNewPropTypeSelected(newPropType: { label: string, value: PropTypeValue, icon: string }) {
  if (prop.value.prop_type === newPropType.value) return;

  const update = () => {
    const current = prop.value;
    const subType = (newPropType.value === 'Select' || newPropType.value === 'Array') ? 'String' : null;
    prop.value = createProp(newPropType.value, current.prop_name, current.description, subType);
  };

  if (props.showAlertOnTypeChange === false) {
    update();
    return;
  }

  // TODO: Sostituire con UModal se preferisci una UI coerente
  if (confirm(`Are you sure you want to change "${prop.value.prop_name}" to ${newPropType.value}? All settings for this prop will be lost.`)) {
    update();
  }
}

function onValueTypeChanged() {
  if (prop.value.prop_type === 'Select') prop.value.options = [];
  else if (prop.value.prop_type === 'Array') prop.value.default = [];
}

// Vincoli Date
function toggleDateConstraint(type: 'past' | 'future', value: boolean) {
  if (prop.value.prop_type !== 'Date' && prop.value.prop_type !== 'Datetime') return;
  if (type === 'past') {
    prop.value.allow_past = value;
    if (!value && !prop.value.allow_future) prop.value.allow_future = true;
  } else {
    prop.value.allow_future = value;
    if (!value && !prop.value.allow_past) prop.value.allow_past = true;
  }
}
</script>

<style scoped>
/* Layout Grid per simulare l'allineamento di NaiveUI */
.grid-form-item {
  display: grid;
  grid-template-columns: 12rem 1fr;
  align-items: center;
  gap: 1.5rem;
}

@media (max-width: 768px) {
  .grid-form-item {
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }
}

/* Stili specifici per rendere il selettore colore più simile a un input standard */
input[type="color"]::-webkit-color-swatch-wrapper {
  padding: 0;
}
input[type="color"]::-webkit-color-swatch {
  border: none;
  border-radius: 4px;
}
</style>