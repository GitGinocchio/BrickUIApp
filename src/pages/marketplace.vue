<template>
  <div class="container">
    <Header :sections="sections"></Header>
    <n-input placeholder="Search widgets or themes…">
      <template #prefix><Search :size="16" /></template>
    </n-input>
    <n-tabs type="line">
      <n-tab-pane name="bricks" tab="Bricks">
        <div class="brick-grid">
          <p>Qui si possono scaricare bricks.</p>
        </div>
      </n-tab-pane>
      <n-tab-pane name="walls" tab="Walls">
        <p>Qui si possono scaricare temi.</p>
      </n-tab-pane>
      <n-tab-pane name="wallpapers" tab="Wallpapers">
        <p>Qui si possono scaricare temi.</p>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<script setup lang="ts">
import { ShoppingBasket, Search } from 'lucide-vue-next';
import { NTabs, NTabPane, NInput } from "naive-ui"

import Header from '#components/Header.vue';

const { t } = useI18n();

const sections = computed(() => {
  return [
    { icon: ShoppingBasket, label: t('marketplace') },
  ]
});

</script>

<style scoped>
.brick-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(25rem, 1fr));
  gap: 1rem
}

.container {
  display: flex;
  flex-direction: column;
  padding: 16px;
  gap: 1rem;
}

.header {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 0.5rem;
  margin-left: 0.5rem;
  margin-left: 0.5rem;
}
</style>