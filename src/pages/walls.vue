<template>
  <div class="flex flex-col h-full overflow-y-auto p-4 gap-6">
    <Header :sections="headerSections" />
  </div>
</template>

<script lang="ts" setup>
import { BrickWall } from 'lucide-vue-next';

const { t } = useI18n();

const headerSections = computed(() => [
  { icon: BrickWall, label: t('walls') }
]);
</script>

<style>

</style>