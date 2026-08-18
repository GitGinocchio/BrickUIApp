<template>
  <UModal v-model:open="state.open" :title="state.title || t('modals.confirm')" :color="state.color">
    <template #body>
      <div v-html="state.message" />
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton color="neutral" variant="soft" :label="state.cancelLabel || t('actions.cancel')" @click="onCancel" />
        <UButton :color="state.color" :label="state.confirmLabel || t('actions.ok')" @click="onConfirm" />
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { useConfirmModalState, useConfirmModal } from '~/composables/useConfirmModal';
const { t } = useI18n();

const { resolve, state } = useConfirmModal();

function onConfirm() {
  resolve(true);
}
function onCancel() {
  resolve(false);
}
</script>
