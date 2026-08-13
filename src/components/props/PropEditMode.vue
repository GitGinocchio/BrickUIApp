<template>
  <div class="flex flex-col w-full space-y-6">
    <UFormField :label="t('placeholders.prop_description')">
      <UTextarea
        v-if="prop.prop_type !== 'Unknown' && prop.prop_type !== 'Deprecated'"
        v-model="prop.description"
        :placeholder="t('placeholders.prop_description')"
        class="w-full"
        autoresize
        :maxlength="512"
        :help="t('hints.markdown_supported')"
      />
    </UFormField>

    <UFormField :label="t('placeholders.prop_type', 'Type')">
      <USelectMenu
        :model-value="propTypeOption"
        class="w-full"
        :icon="(propTypeOption as any).icon"
        :items="propTypeOptions"
        @update:model-value="onNewPropTypeSelected"
      />
    </UFormField>

    <UFormField
      v-if="prop.prop_type == 'Select' || prop.prop_type == 'Array'"
      :label="t('props.value_type')"
    >
      <USelectMenu
        :model-value="valueTypeOption"
        class="w-full"
        :icon="(valueTypeOption as any).icon"
        :items="valueTypeOptions"
        @update:model-value="(value) => onValueTypeChanged(value)"
      >
        <template #item="{ item }">
          <UIcon :name="(item as any).icon" class="w-4 h-4" />
          <span>{{ (item as any).label }}</span>
        </template>
      </USelectMenu>
    </UFormField>

    <UFormField v-if="prop.prop_type === 'Select'" :label="t('props.options')">
      <Array class="w-full" 
        v-model:value="prop.options" 
        :value_type="prop.value_type" 
        :min="prop.min"
        :min_value="prop.min_value"
        :max_value="prop.max_value"
        :default="prop.default"
        :show_reset="false"
      />
    </UFormField>

    <MinMaxStepInput
      class="flex flex-row w-full gap-4" 
      v-if="isCollectionProp(prop) || isNumericProp(prop)"
      v-model:prop="prop"
    />

    <UFormField v-if="prop.prop_type === 'Gradient' || prop.prop_type == 'Color'" :label="t('props.skip_alpha')">
      <USwitch
        v-model="prop.skip_alpha"
        :default-value="false"
      />
    </UFormField>

    <UFormField v-if="shouldShowDefaultInput" :label="t('props.default_value')">
      <component :is="renderDefaultComponent" class="w-full" :prop="(prop as any)" :editMode="true" />
    </UFormField>

    <div v-if="['Date', 'Datetime'].includes(prop.prop_type)" class="flex gap-4">
      <UFormField
        :label="t('props.allow_past')"
      >
        <USwitch v-model="(prop as DatePropType).allow_past" @update:model-value="v => toggleDateConstraint('past', v)" />
      </UFormField>
      <UFormField label="Allow future:">
        <USwitch v-model="(prop as DatePropType).allow_future" @update:model-value="v => toggleDateConstraint('future', v)" />
      </UFormField>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type AllPropsType, type CollectionValueTypes, type DatePropType, type ValidPropType } from '#interfaces'
import type { SelectMenuItem } from '@nuxt/ui';
import Array from '../inputs/Array.vue';
import MinMaxStepInput from './inputs/MinMaxStepInput.vue'
import type { Prop } from '~/interfaces/generated/Prop.ts';
import { COLLECTION_VALUE_TYPES, VALID_PROPS_TYPES } from '~/constants/props.ts';
import { createProp, isCollectionProp, isKnownProp, isNumericProp, isValidProp } from '~/utils/props.ts';

const { confirm } = useConfirmModal();
const { t } = useI18n();

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

const iconsMap: Record<AllPropsType, string> = {
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


const propTypeOption = computed(() => asOption(prop.value.prop_type));
const propTypeOptions: SelectMenuItem[] = VALID_PROPS_TYPES.map((v) => asOption(v));

const valueTypeOption = computed(() => asOption(isValidProp(prop.value) && isCollectionProp(prop.value) ? prop.value.value_type : null));
const valueTypeOptions: SelectMenuItem[] = COLLECTION_VALUE_TYPES.map((v) => asOption(v as AllPropsType));

const shouldShowDefaultInput = computed(() => prop.value.prop_type !== 'Null');

function asOption(v: AllPropsType): SelectMenuItem {
  return {
    label: v.replace(/([A-Z])/g, " $1").trim(),
    type: 'item',
    value: v,
    icon: iconsMap[v] || 'i-lucide-circle'
  }
}

async function onNewPropTypeSelected(newPropType: { value: ValidPropType }) {
  if (prop.value.prop_type === newPropType.value) return;

  const update = () => {
    const current = prop.value;
    const subType: CollectionValueTypes = (newPropType.value === 'Select' || newPropType.value === 'Array') ? 'String' : null;

    const propName = isKnownProp(current) ? current.prop_name : 'test';
    const description = isKnownProp(current) ? current.description : '';

    const newPropObject = createProp(
      newPropType.value as AllPropsType, 
      propName, 
      description, 
      subType
    );

    prop.value = newPropObject;
  };

  if (props.showAlertOnTypeChange === false) {
    update();
    console.log(prop.value);
    return;
  }

  const ok = await confirm({
    title: t('modals.change_prop_type_title', 'Change prop type'),
    message: t('modals.change_prop_type_message', { prop: prop.value.prop_type != 'Unknown' ? prop.value.prop_name : '', type: newPropType.value }),
    confirmLabel: t('actions.save', 'Yes, change'),
    cancelLabel: t('actions.cancel', 'Cancel'),
    color: 'primary'
  });

  if (ok) {
    update();
  }

  console.log(prop.value);
}

function onValueTypeChanged(item: SelectMenuItem) {
  if (!isValidProp(prop.value) || !isCollectionProp(prop.value)) return;

  if (typeof item === 'object' && item !== null && 'value' in item) {
    prop.value.value_type = (item as { value: any }).value;
  }

  if (prop.value.prop_type === 'Select') prop.value.options = [];
  
  prop.value.default = [];
  prop.value.value = []
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