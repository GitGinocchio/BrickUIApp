<template>
  <NNotificationProvider
    :theme="theme.Notification"
    :placement="settings.notifications.position"
  >
    <slot />
  </NNotificationProvider>
</template>

<script setup lang="ts">
import "#assets/css/overlay.css";

import { NNotificationProvider } from "naive-ui";
const { settings, theme } = useAppState();

</script>