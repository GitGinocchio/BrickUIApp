<template>
  <NNotificationProvider
    v-if="isReady"
    :theme="theme.Notification"
    :placement="settings.notifications.position"
  >
    <slot />
  </NNotificationProvider>
</template>

<script setup lang="ts">
import "#assets/css/overlay.css";
const { settings, theme, isReady } = useAppState();

</script>