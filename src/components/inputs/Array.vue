<template>
  <UFormField :error="errorMsg">
    <template #label v-if="show_reset">
      <div class="flex justify-between w-full">
        <UButton
          v-if="isDirty"
          variant="link"
          padding="none"
          size="xs"
          :label="t('actions.reset')"
          icon="i-heroicons-arrow-path"
          @click="resetToDefault"
        />
      </div>
    </template>

    <UInputTags
      v-model="tags"
      :add-on-blur="true"
      :add-on-paste="true"
      :add-on-tab="true"
      :max="props.max"
    />
  </UFormField>
</template>

<script setup lang="ts">
import { ref, computed, watch, type PropType } from 'vue';
import type { CollectionValueTypes } from '~/interfaces';

const toast = useToast();
const { t } = useI18n();

type TagValue = string | number;

const tags = defineModel<TagValue[]>("value", { 
  default: () => [], 
  get: (v) => (Array.isArray(v) ? v.map(String) : []),
  set: (newTags: TagValue[]) => {
    let processed: (string | number)[] = newTags;
    if (props.value_type === 'Integer') {
      processed = newTags.map(v => parseInt(String(v), 10)).filter(n => !isNaN(n));
    } else if (props.value_type === 'Float') {
      processed = newTags.map(v => parseFloat(String(v).replace(',', '.'))).filter(n => !isNaN(n));
    }

    const validTags = processed.filter(val => {
      const [isValid, reasonObj] = validateValue(val);

      if (!isValid) {
        const reasonText = reasonObj ? t(reasonObj.key, reasonObj.params || {}) : '';
        toast.add({
          title: t('array.invalid_value_title'),
          description: t('array.invalid_value_description', { val, reason: reasonText }),
          color: 'error',
          icon: 'i-heroicons-exclamation-triangle'
        });
      }
        
      return isValid;
    });

    return validTags;
  }
});

const props = defineProps({
  min: {
    type: Number,
    default: 0
  },
  max: {
    type: Object as PropType<number | null>,
    default: null
  },
  min_value: {
    type: Number,
    default: null
  },
  max_value: {
    type: Object as PropType<number | null>,
    default: null
  },
  value_type: {
    type: Object as PropType<CollectionValueTypes>,
    default: 'String'
  },
  default: {
    type: Array as PropType<TagValue[]>,
    default: () => []
  },
  show_reset: { 
    type: Boolean, 
    default: true 
  }
});

const errorMsg = ref<string | boolean>(false);

const resetToDefault = () => {
  tags.value = [...props.default];
};

function validateValue(value: string | number): [boolean, { key: string, params?: Record<string, any> } | null] {
  let isValid = true;
  let reasonObj: { key: string, params?: Record<string, any> } | null = null;

  if (props.value_type === 'Integer' || props.value_type === 'Float') {
    if (props.min_value !== undefined && props.min_value !== null && (value as number) < props.min_value) {
      isValid = false;
      reasonObj = { key: 'array.reason.less_than', params: { n: props.min_value } };
    } else if (props.max_value !== undefined && props.max_value !== null && (value as number) > props.max_value) {
      isValid = false;
      reasonObj = { key: 'array.reason.greater_than', params: { n: props.max_value } };
    }
  } else {
    const len = String(value).length;
    if (props.min_value !== undefined && props.min_value !== null && len < props.min_value) {
      isValid = false;
      reasonObj = { key: 'array.reason.too_short', params: { n: props.min_value } };
    } else if (props.max_value !== undefined && props.max_value !== null && len > props.max_value) {
      isValid = false;
      reasonObj = { key: 'array.reason.too_long', params: { n: props.max_value } };
    }
  }

  return [isValid, reasonObj];
}

const isDirty = computed(() => {
  return JSON.stringify(tags.value) !== JSON.stringify(props.default);
});

watch(() => tags.value, (newVal) => {
  if (props.min && newVal && newVal.length < props.min) {
    errorMsg.value = t('array.min_items', { min: props.min });
  } else {
    errorMsg.value = false;
  }
}, { immediate: true });
</script>