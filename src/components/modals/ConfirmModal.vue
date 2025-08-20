<template>
  <n-modal
    v-model:show="show"
    :auto-focus="true"
    :type="type"
    preset="dialog"
    :title="title"
    :negative-text="negative"
    :positive-text="positive"
    @positive-click="emit('confirm')"
    @negative-click="emit('decline')"
  >
    {{ message }}
  </n-modal>    
</template>

<script setup lang="ts">
import { NModal } from 'naive-ui';
import { PropType } from 'vue';

const show = defineModel<boolean>("show");
defineProps({
  positive: { 
    type: String, 
    required: true
  },
  negative: { 
    type: String, 
    required: true 
  },
  title: { 
    type: String, 
    required: true 
  },
  message: { 
    type: String, 
    required: true 
  },
  type: {
    type: String as PropType<"default" | "error" | "info" | "success" | "warning">,
    default: "warning"
  }
});

const emit = defineEmits<{
  (e: "confirm"): void
  (e: "decline"): void
}>();
</script>