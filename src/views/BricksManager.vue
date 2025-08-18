<template>
  <n-space vertical>
    <n-card>
      <div class="header">
        <div>
          <Blocks />
          <h2>Bricks</h2>
        </div>
        <n-button text circle @click="onNewBrick">
          <CirclePlus />
        </n-button>
      </div>
      <n-grid :cols="1" x-gap="16" y-gap="16">
        <n-gi v-for="brick in bricks" :key="brick.name">
          <BrickCard :brick="brick" @edit="onEditBrick" @changed="updateBricks" />
        </n-gi>
      </n-grid>
    </n-card>

    <!-- Modal per nuovo brick -->
    <n-modal v-model:show="showModal" :draggable="true" preset="dialog" :title="editMode ? 'Edit Brick' : 'New Brick'">
      <n-form class="new-brick-form">
        <n-form-item
          label="Name"
          :validation-status="feedback ? 'error' : undefined"
          :show-feedback="feedback ? true : false"
          :feedback="feedback"
        >
          <n-input
            :v-model:value="currentBrick.name"
            :default-value="editMode ? currentBrick.name : null"
            placeholder="Type your brick's name"
            @update:value="onNameInput"
          />
        </n-form-item>

        <n-form-item label="Tags">
          <n-dynamic-tags
            :round="true"
            :default-value="currentBrick.tags"
            :value="currentBrick.tags"
            @update:value="onTagsInput"
          />
        </n-form-item>

        <n-form-item v-if="!editMode" label="Description">
          <n-input
            v-model:value="currentBrick.description"
            type="textarea"
            placeholder="Type your brick's description"
          />
        </n-form-item>
      </n-form>

      <template #action>
        <n-space justify="end">
          <n-button @click="showModal = false">Cancel</n-button>
          <n-button
            type="primary"
            :disabled="editMode ? deepEqual(startBrick, currentBrick) || feedback != null : feedback || !currentBrick.name ? true : false"
            @click="editMode ? updateBrick() : createBrick()"
          >   
          {{ editMode ? 'Save' : 'Create' }}
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { CirclePlus, Blocks } from "lucide-vue-next";
import { NSpace, NButton, NGrid, NGi, NCard, NModal, NForm, NFormItem, NInput, NDynamicTags } from "naive-ui";
import BrickCard from "../components/BrickCard.vue";
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Brick } from "interfaces/brick";
import { deepEqual } from "../utils";

const bricks = ref<Brick[]>([]);
const showModal = ref(false);
const feedback = ref<string | null>(null);
const editMode = ref(false);
const startBrick = ref<Brick|null>(null);
const currentBrick = ref<Brick>({
  name: null, 
  description: '',
  author: null,
  props: [],
  dependencies: [],
  tags: [],
  version: [0, 1, 0],
  enabled: true,
});

onMounted(async () => {
  await updateBricks();
});

function onNewBrick() {
  currentBrick.value = { 
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
  showModal.value = true;
  editMode.value = false;
}

function onEditBrick(brick: Brick) {
  currentBrick.value = JSON.parse(JSON.stringify(brick));
  startBrick.value = JSON.parse(JSON.stringify(brick));
  feedback.value = null;
  showModal.value = true;
  editMode.value = true;
};

function onNameInput(value: string) {
  const componentNameRegex = /^[a-zA-Z][a-zA-Z0-9]*(?:_[a-zA-Z0-9]+)*$/;
  const other_bricks = startBrick.value ? bricks.value.filter((brick) => brick.name !== startBrick.value.name) : bricks.value;
  
  if (!componentNameRegex.test(value)) {
    feedback.value = "Must start with a letter. Only letters, numbers, dashes, and underscores are allowed."
  }
  else if (other_bricks.filter((brick) => brick.name === value).length > 0) {
    feedback.value = "A brick with this name already exists."
  }
  else {
    feedback.value = null;
  }

  if (editMode) currentBrick.value.name = value;
}

function onTagsInput(value: string[]) {
  currentBrick.value.tags = Array.from(new Set(value));
}

async function createBrick() {
  console.log(currentBrick.value);
  await invoke("new_brick", { brick: currentBrick.value });
  await updateBricks();
  showModal.value = false; 
}

async function updateBrick() {
  try {
    if (startBrick.value.name !== currentBrick.value.name) {
      await invoke("rename_brick", { oldName: startBrick.value.name, newName: currentBrick.value.name });
    }

    if (!deepEqual(startBrick.value, currentBrick.value)) {
      await invoke("save_brick", { brick: currentBrick.value });
      await updateBricks();
    }

    showModal.value = false;
  } catch (error) {
    console.error(error);
  }
}

async function updateBricks() {
  bricks.value = await invoke<Brick[]>("load_bricks", {});
}
</script>

<style scoped>
::deep(n-card__content:first-child) {
  padding-top: 0;
}

.header { 
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-left: 0.5rem;
  margin-right: 0.5rem;
}

.header div {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  gap: 0.5rem;
}
</style>

<style>
.new-brick-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>