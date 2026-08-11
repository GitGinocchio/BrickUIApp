<template>
  <div class="flex flex-col min-h-screen overflow-y-auto gap-4 p-4">
    <Header :sections="sections">
      <template #actions>
        <UButton 
          icon="i-lucide-circle-plus" 
          variant="ghost" 
          @click="openNewBrickModal" 
        />
      </template>
    </Header>

    <div class="grid grid-cols-[repeat(auto-fit,minmax(20rem,1fr))] gap-4">
      <BrickInfoCard 
        v-for="brick in bricks" 
        :key="brick.name" 
        :brick="brick" 
        :offline="true"
        class="transform-gpu transition-all duration-200"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Blocks } from "lucide-vue-next";
import BrickInfoCard from "#components/BrickInfoCard.vue";
import Header from "#components/Header.vue";

const { t } = useI18n();
const router = useRouter();
const { bricks } = useAppState();
const { openNewBrickModal } = useBrickActions();

const sections = computed(() => [
  { icon: Blocks, label: t('bricks.title'), onclick: () => router.push('/bricks') },
]);
</script>