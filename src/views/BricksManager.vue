<template>
  <n-space vertical>
    <n-card>
      <div class="header">
        <h2>Bricks</h2>
        <n-button text circle @click="onNewBrick"><CirclePlus /></n-button>
      </div>
      <n-grid :cols="1" x-gap="16" y-gap="16">
        <n-gi v-for="brick in bricks" :key="brick.name">
          <BrickCard :brick="brick" />
        </n-gi>
      </n-grid>
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import { CirclePlus } from "lucide-vue-next";
import { NSpace, NButton, NGrid, NGi, NCard } from "naive-ui";
import BrickCard from "../components/BrickCard.vue"
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Brick } from 'interfaces/brick';

const bricks = ref<Brick[]>([]);

onMounted(async () => {
  bricks.value = await invoke<Brick[]>("get_bricks", {});
});


async function onNewBrick() {
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

</style>
