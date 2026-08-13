<template>
  <UFormField class="w-full" :error="error">
    <USelectMenu
      :model-value="targetValue"
      @update:model-value="updateModelValue"
      :default-value="prop.default"
      :items="prop.options"
      :multiple="isMultiple"
      :required="prop.min !== null && prop.min >= 0"
      class="w-full"
      clear
    />
  </UFormField>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import type { SelectPropType } from '~/interfaces';

const { t } = useI18n();
const prop = defineModel<SelectPropType>("prop", { required: true });
const isMultiple = computed(() => (prop.value.min !== null && prop.value.min > 1) || (prop.value.max !== null && prop.value.max > 1));
const error = computed(() => validateSelect(targetValue.value))
const targetValue = computed(() => {
  const rawValue = props.editMode ? prop.value.default : prop.value.value;

  if (isMultiple.value) {
    // Per selezione multipla, USelectMenu vuole sempre un Array
    if (Array.isArray(rawValue)) return rawValue;
    return rawValue != null ? [rawValue] : [];
  } else {
    // Per selezione singola, USelectMenu vuole un elemento singolo
    if (Array.isArray(rawValue)) return rawValue[0] ?? null;
    return rawValue ?? null;
  }
});

const props = defineProps({
  editMode: {
    type: Boolean,
    default: false
  }
});

function validateSelect(value: string | number | string[] | number[]): boolean | string {
  if (!Array.isArray(value)) {
    return false;
  }

  let reason = null;

  if (prop.value.max && value.length > prop.value.max) {
    reason = t('select.reason.too_long', { n: prop.value.max })
  } else if (prop.value.min && value.length < prop.value.min) {
    reason = t('select.reason.too_short', { n: prop.value.min })
  }

  if (reason !== null) return t('select.invalid_value_description', { reason: reason })
  else false
}

function updateModelValue(newValue: string | number | (string | number)[] | null | undefined) {
  let formattedValue: string[] | number[];

  if (isMultiple.value) {
    // 1. Uniformiamo l'input in un array
    if (Array.isArray(newValue)) {
      formattedValue = newValue as string[] | number[];
    } else if (newValue != null) {
      formattedValue = [newValue] as string[] | number[];
    } else {
      formattedValue = [];
    }

    // 2. Appliciamo il clamp basato su prop.value.max se definito
    const maxLimit = prop.value.max;
    if (maxLimit !== null && maxLimit !== undefined && maxLimit > 0) {
      if (formattedValue.length > maxLimit) {
        formattedValue = formattedValue.slice(0, maxLimit) as string[] | number[];
      }
    }
  } else {
    // Se è singolo, memorizziamo un array con 1 elemento (o vuoto se deselezionato)
    formattedValue = newValue != null ? ([newValue] as string[] | number[]) : [];
  }

  // 3. Assegnazione al modello
  if (props.editMode) {
    prop.value.default = formattedValue;
  } else {
    prop.value.value = formattedValue;
  }
}

</script>