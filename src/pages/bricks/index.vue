<template>
  <div class="container">
    <Header :sections="sections">
      <template #actions>
        <n-button text circle @click="openNewBrickDialog">
          <CirclePlus />
        </n-button>
      </template>
    </Header>
    <div class="brick-grid">
      <BrickInfoCard 
        v-for="brick in bricks" 
        :key="brick.name" 
        :brick="brick" 
        :offline="true"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Blocks, CirclePlus, PartyPopper } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";
import type { Brick } from "#interfaces/brick";
import BrickInfoCard from "#components/BrickInfoCard.vue";
import Header from "#components/Header.vue";
import { NFormItem, NInput, useDialog } from "naive-ui";

const { t } = useI18n();
const router = useRouter();
const { bricks } = useAppState();
const dialog = useDialog();

const sections = computed(() => {
  return [
    { icon: Blocks, label: t('bricks'), onclick: () => router.push('/bricks') },
  ]
});

async function onNewBrick(brickName: string) {
  const brick: Brick = {
    name: brickName, 
    description: '',
    author: null,
    props: [],
    dependencies: [],
    tags: [],
    version: [0, 1, 0],
    enabled: true,
  };
  await invoke("new_brick", { brick: brick });
  bricks.value.push(brick);
}

function openNewBrickDialog() {
  const brickName = ref("");
  const feedback = ref<string | null>(null);

  const componentNameRegex = /^[a-zA-Z][a-zA-Z0-9]*(?:_[a-zA-Z0-9]+)*$/;

  const d = dialog.create({
    title: t('modals.new_brick_title', 'New Brick'),
    icon: () => h(PartyPopper),
    content: () => 
      h(NFormItem, {
        label: t('fields.name', 'Name'),
        validationStatus: feedback.value ? 'error' : undefined,
        feedback: feedback.value,
      }, {
        default: () => h(NInput, {
          value: brickName.value,
          placeholder: t('placeholders.brick_name', "Type your brick's name"),
          onUpdateValue: (v: string) => {
            brickName.value = v;
            
            if (!v) {
              feedback.value = null;
            } else if (!componentNameRegex.test(v)) {
              feedback.value = t('errors.invalid_name', "Must start with a letter. Only letters, numbers, and underscores allowed.");
            } else if (bricks.value.some(b => b.name === v)) {
              feedback.value = t('errors.duplicate_name', "A brick with this name already exists.");
            } else {
              feedback.value = null;
            }

            d.positiveButtonProps = {
              disabled: !brickName.value || feedback.value !== null
            };
          },
          onKeyup: async (e: KeyboardEvent) => {
            if (e.key === 'Enter' && brickName.value && !feedback.value) {
              d.destroy();
              await onNewBrick(brickName.value);
            }
          }
        })
      }),
    positiveText: t('actions.create', 'Create'),
    negativeText: t('actions.cancel', 'Cancel'),
    positiveButtonProps: {
      disabled: true 
    },
    onPositiveClick: async () => {
      await onNewBrick(brickName.value);
    }
  });
}
</script>

<style scoped>
::deep(.n-card__content:first-child) {
  padding-top: 0;
}

::deep(.n-layout-scroll-container) {
  overflow-y: hidden;
}

.container {
  display: flex;
  flex-direction: column;
  overflow-y: hidden;
  padding: 1rem;
  gap: 1rem;
}

n-card.full-height {
  flex: 1;
  flex-grow: 1;
  display: flex;
  flex-direction: column;
}

/* Se vuoi che la griglia si espanda e scrolli */
.brick-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(20rem, 1fr));
  gap: 1rem
}

.bricks-scroll {
  overflow-y: auto;
  overflow-x: hidden;
  flex: 1;
}

.new-brick-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>