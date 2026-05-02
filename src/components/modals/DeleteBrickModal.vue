<template>
  <UModal v-model:open="isOpen" title="Confirm deletion" color="red">
    <template #body>
      <p>Are you sure you want to delete "{{ brickToDelete.name }}"?</p>
    </template>
    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton color="neutral" variant="soft" label="Cancel" @click="isOpen = false" />
        <UButton color="error" label="Delete" @click="onDelete" />
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import type { Brick } from '~/interfaces/brick';

const isOpen = ref(false);
const brickToDelete = ref<Brick|null>(null);

const { deleteBrick } = useBrickActions();

const open = (brick: Brick) => {
  console.log('brick da eliminare: ', brick);
  brickToDelete.value = brick;
  isOpen.value = true;
};

const onDelete = () => {
  deleteBrick(brickToDelete.value);
  navigateTo("/bricks");
  isOpen.value = false;
};

defineExpose({ open });
</script>