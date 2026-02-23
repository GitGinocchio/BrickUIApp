<template>
  <n-notification-provider
    :theme="currentNotificationTheme"
    :placement="settings.notifications.position"
  >
    <Overlay />
  </n-notification-provider>
</template>

<script setup lang="ts">
import { NNotificationProvider } from "naive-ui";
import Overlay from "./Overlay.vue";
import { inject, ref, watchEffect } from "vue";


// Prendi settings e theme dal main.ts
const settings = inject("settings") as any;
const theme = inject("theme") as any;

// Tema reattivo per le notifiche
const currentNotificationTheme = ref(theme.value.Notification);
watchEffect(() => {
  currentNotificationTheme.value = theme.value.Notification;
});
</script>
