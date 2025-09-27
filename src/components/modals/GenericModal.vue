<template>
  <n-modal
    v-model:show="show"
    :auto-focus="true"
    :type="type"
    preset="dialog"
    :title="title"
    :negative-text="negative"
    :positive-text="positive"
    @keyup.enter="emit('confirm')"
    @keyup.escape="emit('decline')"
    @positive-click="emit('confirm')"
    @negative-click="emit('decline')"
  >
    <div class="content">
      <div class="message" v-if="message">
        {{ message }}
      </div>

      <!-- slot extra sotto al messaggio -->
      <div class="extras">
        <slot name="extra" />
      </div>
    </div>

    <!-- slot per i bottoni custom -->
    <template #action>
      <slot name="actions">
        <!-- fallback: bottoni di default -->
        <!--
        <n-button @click="emit('decline')">{{ negative }}</n-button>
        <n-button type="primary" @click="emit('confirm')">{{ positive }}</n-button>
        -->
      </slot>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { NModal, NButton } from 'naive-ui';
import { PropType } from 'vue';

const show = defineModel<boolean>('show');
defineProps({
  positive: { type: String, required: false },
  negative: { type: String, required: false },
  title: { type: String, required: true },
  message: { type: String, required: false, default: null },
  type: {
    type: String as PropType<'default' | 'error' | 'info' | 'success' | 'warning'>,
    default: 'warning'
  }
});

const emit = defineEmits<{
  (e: 'confirm'): void
  (e: 'decline'): void
}>();
</script>

<style scoped>
.content {
  display: flex;
  flex-direction: column;
  margin-top: 1.25rem;
  gap: 1rem;
}

.extras {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
