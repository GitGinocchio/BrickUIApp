<template>
  <UApp>
    <NuxtLayout fallback="default">
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>

<script setup lang="ts">
import { useAppState } from "~/composables/useAppState"

const { init } = useAppState();

onBeforeMount(async () => {
  await init();
})
</script>