<template>
  <n-card :title="widget.name" :segmented="true" hoverable>
    <p>{{ widget.description }}</p>
    <n-space justify="space-between">
      <n-switch v-model:value="widget.enabled" />
      <n-button size="small" @click="$emit('open-settings', widget.name)">Settings</n-button>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
defineProps<{ widget: { name: string, description: string, enabled: boolean } }>()
defineEmits(['open-settings'])
</script>
