<template>
  <div class="container">
    <div class="header">
      <ShoppingBasket :size="24" /><h2>Marketplace</h2>
    </div>
    <n-input placeholder="Search widgets or themes…">
      <template #prefix><Search :size="16" /></template>
    </n-input>
    <n-tabs type="line">
      <n-tab-pane name="bricks" tab="Bricks">
        <div class="brick-grid">
          <BrickInfoCard
            v-for="brick in bricks"
            :key="brick.name"
            :brick="brick"
            :offline="false"
          />
          <BrickInfoCard
            v-for="brick in bricks"
            :key="brick.name"
            :brick="brick"
            :offline="false"
          />
          <BrickInfoCard
            v-for="brick in bricks"
            :key="brick.name"
            :brick="brick"
            :offline="false"
          />
          <BrickInfoCard
            v-for="brick in bricks"
            :key="brick.name"
            :brick="brick"
            :offline="false"
          />
          <BrickInfoCard
            v-for="brick in bricks"
            :key="brick.name"
            :brick="brick"
            :offline="false"
          />
          <BrickInfoCard
            v-for="brick in bricks"
            :key="brick.name"
            :brick="brick"
            :offline="false"
          />
          <BrickInfoCard
            v-for="brick in bricks"
            :key="brick.name"
            :brick="brick"
            :offline="false"
          />
          <BrickInfoCard
            v-for="brick in bricks"
            :key="brick.name"
            :brick="brick"
            :offline="false"
          />
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
import { Brick } from 'interfaces/brick';
import BrickInfoCard from '../components/BrickInfoCard.vue';
import { ShoppingBasket, Search } from 'lucide-vue-next';
import { NTabs, NTabPane, NInput } from "naive-ui"
import { inject, Ref } from 'vue';

const bricks = inject("bricks") as Ref<Brick[]>;

</script>

<style scoped>
.brick-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(25rem, 1fr));
  gap: 1rem
}

.container { 
  padding: 16px;
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