<template>
  <UModal v-model:open="isOpen" title="Confirm deletion" color="red">
    <template #body>
      <p>Are you sure you want to delete "{{ propToDelete.prop_name }}"?</p>
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
import type { Brick, Prop } from '~/interfaces/brick';

const { deleteProp } = usePropActions();

const isOpen = ref(false);
const propBrick = ref<Brick|null>(null);
const propToDelete = ref<Prop|null>(null);

const open = (brick: Brick, prop: Prop) => {
  console.log('prop da eliminare: ', prop);
  propToDelete.value = prop;
  propBrick.value = brick;
  isOpen.value = true;
};

const onDelete = () => {
  deleteProp(propBrick.value, propToDelete.value)
  isOpen.value = false;
};

defineExpose({ open });
</script>