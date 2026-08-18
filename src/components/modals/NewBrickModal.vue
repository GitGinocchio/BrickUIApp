<template>
  <UModal v-model:open="isOpen" :title="t('modals.new_brick_title')" class="sm:max-w-md">
    <template #body>
      <UFormField :label="t('labels.name')" :error="feedback">
        <UInput
          class="w-full"
          v-model="brickName" 
          :placeholder="t('placeholders.brick_name')"
          @update:model-value="validate"
        />
      </UFormField>
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton 
          :label="t('actions.cancel')" 
          color="neutral" 
          variant="soft" 
          @click="isOpen = false" 
        />
        <UButton 
                  :label="t('actions.create')" 
          :disabled="!brickName || !!feedback" 
          @click="handleCreate" 
        />
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { BRICK_NAME_REGEX } from "~/constants/brick";
import type { Brick } from "~/interfaces";

const { bricks } = useAppState(); 
const { newBrick } = useBrickActions();
const { t } = useI18n();

const isOpen = ref(false);
const brickName = ref("");
const feedback = ref<string | null>(null);

// Metodo per aprire la modale e resettare i campi
const open = () => {
  brickName.value = "";
  feedback.value = null;
  isOpen.value = true;
};

// Validazione speculare a quella di Edit Brick
const validate = (v: string) => {
  if (!v) {
    feedback.value = null;
  } else if (!BRICK_NAME_REGEX.test(v)) {
    feedback.value = t('errors.invalid_name_format');
  } else if (bricks.value.some(b => b.name.toLowerCase() === v.toLowerCase())) {
    feedback.value = t('errors.name_exists');
  } else {
    feedback.value = null;
  }
};

async function handleCreate() {
  if (!brickName.value || feedback.value) return;

  const brick: Brick = {
    name: brickName.value, 
    description: '',
    author: null,
    props: [],
    dependencies: [],
    tags: [],
    version: [0, 1, 0],
    enabled: true,
  };

  await newBrick(brick);
  navigateTo(`/bricks/${brick.name}`);
  isOpen.value = false;
}

// Esponiamo il metodo open per il componente padre
defineExpose({ open });
</script>