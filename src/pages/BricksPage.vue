<template>
  <div class="container">
    <Header :sections="sections">
      <template #actions>
        <n-button text circle @click="onNewBrick">
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
        @edit="onEditBrick" 
        @changed="updateBricks" 
      />
    </div>
  </div>
  <BrickModal
    v-model:brick="brick" 
    v-model:edit-mode="editMode"
    v-model:initial-brick="initialBrick"
    v-model:show="show"
    v-model:feedback="feedback"
  />
</template>

<script setup lang="ts">
import { computed, inject, onMounted, Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Brick } from "../interfaces/brick";
//import { useI18n } from "vue-i18n";
import { listen } from "@tauri-apps/api/event";
import BrickInfoCard from "../components/BrickInfoCard.vue";
import Header from "../components/Header.vue";
import BrickModal from "../components/modals/BrickModal.vue";
import { Blocks, CirclePlus } from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

const { t } = useI18n();
const router = useRouter();

const sections = computed(() => {
  return [
    { icon: Blocks, label: t('bricks'), onclick: () => router.push('/bricks') },
  ]
});

const show = ref(false);
const feedback = ref<string | null>(null);
const editMode = ref(false);
const initialBrick = ref<Brick|null>(null);
const bricks = inject("bricks") as Ref<Brick[]>;
const brick = ref<Brick>({
  name: null, 
  description: '',
  author: null,
  props: [],
  dependencies: [],
  tags: [],
  version: [0, 1, 0],
  enabled: true,
});

onMounted(async () => await loadBricks());
listen("update_bricks",async () => await updateBricks());

function onNewBrick() {
  brick.value = { 
    name: null, 
    description: '', 
    author: null, 
    props: [], 
    dependencies: [], 
    tags: [], 
    version: [0, 1, 0], 
    enabled: true 
  }
  feedback.value = null;
  show.value = true;
  editMode.value = false;
}

function onEditBrick(b: Brick) {
  brick.value = JSON.parse(JSON.stringify(b));
  initialBrick.value = JSON.parse(JSON.stringify(b));
  feedback.value = null;
  show.value = true;
  editMode.value = true;
};

async function updateBricks() {
  bricks.value = await invoke<Brick[]>("get_bricks", {});
}

async function loadBricks() {
  bricks.value = await invoke<Brick[]>("load_bricks", {});
}
</script>

<style scoped>
::deep(n-card__content:first-child) {
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
  grid-template-columns: repeat(auto-fill, minmax(25rem, 1fr));
  gap: 1rem
}

.bricks-scroll {
  overflow-y: auto;
  overflow-x: hidden;
  flex: 1;
}
</style>

<style>
.new-brick-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>