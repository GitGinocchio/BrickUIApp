<template>
  <UFormField class="w-full" :error="error">
    <div class="flex items-center gap-2 w-full">
      <UInputTags
        class="flex-1"
        :model-value="tags"
        @add-tag="validateTag"
        @update:model-value="onTagsUpdate"
        :add-on-blur="true"
        :add-on-paste="true"
        :add-on-tab="true"
        :max="props.max ?? 50"
      />
      <UButton
        v-if="show_reset && isDirty"
        variant="link"
        padding="none"
        size="xs"
        :label="t('actions.reset')"
        icon="i-heroicons-arrow-path"
        class="shrink-0"
        @click="resetToDefault"
      />
    </div>
  </UFormField>
</template>

<script setup lang="ts">
import { computed, type PropType } from 'vue';
import type { CollectionValueTypes } from '~/interfaces';

const toast = useToast();
const { t } = useI18n();

type TagValue = string | number;

const tags = defineModel<TagValue[]>("value", { default: () => [] });
const props = defineProps({
  min: {
    type: Number,
    default: 0
  },
  max: {
    type: Number,
    default: null
  },
  min_value: {
    type: Number,
    default: null
  },
  max_value: {
    type: Number,
    default: null
  },
  value_type: {
    type: String as PropType<CollectionValueTypes>,
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

const resetToDefault = () => (tags.value = [...props.default]);

const error = computed(() => validateNumberOfTags(tags.value))

function onTagsUpdate(newTags: TagValue[]) {
  if (props.value_type === 'Int') {
    newTags = newTags.map(v => parseInt(String(v), 10)).filter(n => !isNaN(n));
  } else if (props.value_type === 'Float') {
    newTags = newTags.map(v => parseFloat(String(v).replace(',', '.'))).filter(n => !isNaN(n));
  }

  newTags = [...new Set(newTags)];

  if (props.min_value) {
    newTags = newTags.filter(v => typeof v === 'string' ? v.length >= props.min_value : v >= props.min_value)
  }

  if (props.max_value) {
    newTags = newTags.filter(v => typeof v === 'string' ? v.length <= props.max_value : v <= props.max_value)
  }

  tags.value = newTags
}

function validateNumberOfTags(tags: TagValue[]) {
  if (tags.length < props.min) {
    return t('array.reason.too_short', { n: props.min })
  } else if (props.max && tags.length > props.max) {
    return t('array.reason.too_long', { n: props.max })
  }

  return false
}

function validateTag(tag: TagValue) {
  let reason: string | null = null;

  let parsed = props.value_type === 'Int' ? parseInt(String(tag), 10) : parseFloat(String(tag))

  if (props.value_type in ['Int', 'Float'] && isNaN(parsed)) {
    reason = props.value_type === 'Int' ? t('array.reason.not_an_integer') : t('array.reason.not_a_float')
  }

  if (props.min_value !== null && (typeof tag === 'number' ? parsed : String(tag).length) < props.min_value) {
    reason = t('array.reason.less_than', { n: props.min_value })
  } else if (props.max_value !== null && (typeof tag === 'number' ? parsed : String(tag).length) > props.max_value) {
    reason = t('array.reason.greater_than', { n: props.max_value })
  }

  if (!reason) return;

  toast.add({
    title: t('array.invalid_value_title'),
    description: t('array.invalid_value_description', { val: tag, reason: reason }),
    color: 'error',
    icon: 'i-heroicons-exclamation-triangle'
  });
}

const isDirty = computed(() => {
  const current = tags.value || [];
  const defaults = props.default || [];

  if (current.length !== defaults.length) return true;

  return !current.every((val, index) => String(val) === String(defaults[index]));
});
</script>