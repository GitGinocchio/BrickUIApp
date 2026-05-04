<template>
  <UFormField :error="errorMsg">
    <template #label v-if="show_reset">
      <div class="flex justify-between w-full">
        <UButton
          v-if="isDirty"
          variant="link"
          padding="none"
          size="xs"
          label="Reset"
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
import { ref, watch } from 'vue';
import type { CollectionValueType } from '~/interfaces/brick';

const toast = useToast();

const tags = defineModel<any[]>("value", { 
  default: [], 
  get: (v) => (Array.isArray(v) ? v.map(String) : []),
  set: (newTags: any[]) => {
    let processed = newTags;
    if (props.value_type === 'Integer') {
      processed = newTags.map(v => parseInt(v)).filter(n => !isNaN(n));
    } else if (props.value_type === 'Float') {
      processed = newTags.map(v => parseFloat(String(v).replace(',', '.'))).filter(n => !isNaN(n));
    }

    const validTags = processed.filter(val => {
      const [isValid, reason] = validateValue(val);

      if (!isValid) {
        toast.add({
          title: 'Valore non valido',
          description: `Il valore "${val}" è stato rimosso perché è ${reason}.`,
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
    type: Object as PropType<CollectionValueType>,
    default: 'String'
  },
  default: {
    type: Object as PropType<any[]>,
    default: []
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

function validateValue(value: string | number): [boolean, string] {
  let isValid = true;
  let reason = null;

  if (props.value_type === 'Integer' || props.value_type === 'Float') {
    if (props.min_value !== undefined && value as number < props.min_value) {
      isValid = false;
      reason = `inferiore a ${props.min_value}`;
    } else if (props.max_value !== undefined && value as number > props.max_value) {
      isValid = false;
      reason = `superiore a ${props.max_value}`;
    }
  } else {
    const len = String(value).length;
    if (props.min_value !== undefined && len < props.min_value) {
      isValid = false;
      reason = `troppo corto (min ${props.min_value})`;
    } else if (props.max_value !== undefined && len > props.max_value) {
      isValid = false;
      reason = `troppo lungo (max ${props.max_value})`;
    }
  }

  return [isValid, reason];
}

const isDirty = computed(() => {
  return JSON.stringify(tags.value) !== JSON.stringify(props.default);
});

watch(() => tags.value, (newVal) => {
  if (props.min && newVal.length < props.min) {
    errorMsg.value = `Devi inserire almeno ${props.min} elementi.`;
  } else {
    errorMsg.value = false;
  }
}, { immediate: true });
</script>