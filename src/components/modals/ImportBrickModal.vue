<template>
  <UModal v-model:open="isOpen" title="Import Brick">
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
          title="Info" 
          color="info"
          variant="soft" 
          icon="i-lucide-info" 
        >
          <template #description>
            You can verify the brick by yourself by changing the file extension from <code>.brick</code> to <code>.zip</code> and look to the code inside.
          </template>
        </UAlert>
      </div>
    </template>
    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton color="neutral" variant="soft" label="Cancel" @click="isOpen = false" />
        <UButton color="warning" label="Confirm" @click="onConfirm" />
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
const isOpen = ref(false);
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