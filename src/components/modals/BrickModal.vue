<template>
  <n-modal 
    v-model:show="show" 
    :draggable="true" 
    preset="dialog" 
    :title="editMode ? 'Edit Brick' : 'New Brick'"
    @keyup.enter="onEnterClicked"
  >
    <n-form class="new-brick-form">
      <n-form-item
        label="Name"
        :validation-status="feedback ? 'error' : undefined"
        :show-feedback="feedback ? true : false"
        :feedback="feedback"
      >
        <n-input
          :v-model:value="brick.name"
          :default-value="editMode ? brick.name : null"
          placeholder="Type your brick's name"
          @update:value="onNameInput"
        />
      </n-form-item>

      <n-form-item v-if="!editMode" label="Description">
        <n-input
          v-model:value="brick.description"
          type="textarea"
          placeholder="Type your brick's description"
        />
      </n-form-item>

      <n-form-item label="Tags">
        <n-dynamic-tags
          :round="true"
          :default-value="brick.tags"
          :value="brick.tags"
          @update:value="onTagsInput"
        />
      </n-form-item>
    </n-form>

    <template #action>
      <n-space justify="end">
        <n-button @click="show = false">Cancel</n-button>
        <n-button
          type="primary"
          :disabled="editMode ? deepEqual(initialBrick, brick) || feedback != null : feedback || !brick.name ? true : false"
          @click="editMode ? updateBrick() : createBrick()"
        >   
        {{ editMode ? 'Save' : 'Create' }}
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { NModal, NForm, NFormItem, NDynamicTags, NInput } from 'naive-ui';
import { invoke } from '@tauri-apps/api/core';
import { Brick } from '../../interfaces/brick';
import { deepEqual } from '../../utils/misc';
import { inject, onMounted, onUnmounted, Ref, ref } from 'vue';

const show = defineModel<boolean>("show");
const brick = defineModel<Brick>("brick");
const editMode = defineModel<Boolean>("editMode");
const initialBrick = defineModel<Brick>("initialBrick");
const feedback = defineModel<string|null>("feedback", { default: null });
const bricks = inject("bricks") as Ref<Brick[]>;

function onNameInput(value: string) {
  const componentNameRegex = /^[a-zA-Z][a-zA-Z0-9]*(?:_[a-zA-Z0-9]+)*$/;
  const other_bricks = initialBrick.value ? bricks.value.filter((brick) => brick.name !== initialBrick.value.name) : bricks.value;
  
  if (!componentNameRegex.test(value)) {
    feedback.value = "Must start with a letter. Only letters, numbers, dashes, and underscores are allowed."
  }
  else if (other_bricks.filter((brick) => brick.name === value).length > 0) {
    feedback.value = "A brick with this name already exists."
  }
  else {
    feedback.value = null;
  }

  console.log(value, feedback.value);

  //if (editMode.value) brick.value.name = value;
  brick.value.name = value;
}

function onTagsInput(value: string[]) {
  brick.value.tags = Array.from(new Set(value));
}

async function createBrick() {
  console.log(brick.value);
  await invoke("new_brick", { brick: brick.value });
  await updateBricks();
  show.value = false; 
}

async function updateBrick() {
  try {
    if (initialBrick.value.name !== brick.value.name) {
      await invoke("rename_brick", { oldName: initialBrick.value.name, newName: brick.value.name });
    }

    if (!deepEqual(initialBrick.value, brick.value)) {
      await invoke("save_brick", { brick: brick.value });
      await updateBricks();
    }

    show.value = false;
  } catch (error) {
    console.error(error);
  }
}

async function onEnterClicked() {
  console.log('enter clicked');
  if (editMode.value) {
    if (deepEqual(initialBrick.value, brick.value) || feedback != null) {
      return;
    }
    await updateBrick();
  } else {
    if (feedback || !brick.value.name) {
      return;
    }
    await createBrick();
  }
}

async function updateBricks() {
  bricks.value = await invoke<Brick[]>("get_bricks", {});
}
</script>