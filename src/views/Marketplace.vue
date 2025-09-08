<template>
  <div class="container">
    <Header :sections="sections"></Header>
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
import Header from '../components/Header.vue';
import { ShoppingBasket, Search } from 'lucide-vue-next';
import { NTabs, NTabPane, NInput } from "naive-ui"
import { computed, inject, Ref } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const bricks = inject("bricks") as Ref<Brick[]>;
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