<template>
  <UModal v-model:open="isOpen" :title="t('modals.import_brick_title')">
    <template #body>
      <div class="space-y-4">
        <UAlert 
          title="Warning" 
          color="warning"
          variant="soft" 
          icon="i-lucide-alert-triangle" 
          description="Careful when importing bricks from untrusted sources" 
        />
        <UAlert 
                  :title="t('modals.info')" 
          color="info"
          variant="soft" 
          icon="i-lucide-info" 
        >
          <template #description>
                    <span v-html="t('modals.import_verify_instructions_html')"></span>
          </template>
        </UAlert>
      </div>
    </template>
    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton color="neutral" variant="soft" :label="t('actions.cancel')" @click="isOpen = false" />
                <UButton color="warning" :label="t('actions.confirm')" @click="onConfirm" />
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
const isOpen = ref(false);
const { t } = useI18n();
const brickPath = ref<string|null>(null);
const brickName = ref<string|null>(null);

const { importBrick } = useBrickActions();

async function onConfirm() {
  isOpen.value = false;
  importBrick(brickPath.value, brickName.value);
  await navigateTo(`/bricks/${brickName.value}`);
}

function open(brick: [string, string]) {
  brickPath.value = brick[0];
  brickName.value = brick[1];
  isOpen.value = true;
};

defineExpose({ open });
</script>