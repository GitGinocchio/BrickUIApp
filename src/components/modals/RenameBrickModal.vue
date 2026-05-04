<template>
  <UModal v-model:open="isOpen" title="Edit Brick">
    <template #body>
      <UFormField label="Name" :error="feedback">
        <UInput
          class="w-full"
          v-model="newName" 
          placeholder="Type your brick's name"
          @update:model-value="validate"
        />
      </UFormField>
    </template>
    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton color="neutral" variant="soft" label="Cancel" @click="isOpen = false" />
        <UButton 
          label="Save" 
          :disabled="!newName || !!feedback" 
          @click="onSave" 
        />
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { BRICK_NAME_REGEX } from "~/constants/brick";
import type { Brick } from "~/interfaces/brick";

const { bricks } = useAppState();
const { renameBrick } = useBrickActions();

const isOpen = ref(false);
const brick = ref<Brick | null>(null);
const newName = ref("");
const feedback = ref<string | null>(null);
const should_redirect = ref<boolean>(false);

const open = (b: Brick, redirect: boolean) => {
  isOpen.value = false;
  brick.value = b;
  newName.value = b.name;
  should_redirect.value = redirect;
  isOpen.value = true;
};

const validate = (v: string) => {
  if (!v) feedback.value = null;
  else if (!BRICK_NAME_REGEX.test(v)) feedback.value = "Invalid name format";
  else if (brick.value && v !== brick.value.name && bricks.value.some(b => b.name === v)) feedback.value = "Duplicate name";
  else feedback.value = null;
};

const onSave = async () => {
  if (!brick.value || feedback.value) return;
  await renameBrick(brick.value.name, newName.value);
  if (should_redirect.value) navigateTo(`/bricks/${newName.value}`);
  isOpen.value = false;
};

defineExpose({ open });
</script>