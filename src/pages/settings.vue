<template>
  <div class="flex flex-col h-full overflow-y-auto p-4 gap-6">
    
    <Header :sections="headerSections" />

    <UCard class="flex flex-col overflow-visible w-full">
      <template #header>
        <h3 class="text-base font-semibold leading-6">General Settings</h3>
      </template>

      <UFormField label="Language" help="Select your preferred language" class="mb-4">
        <USelect color="primary" 
          v-model="settings.language" 
          :items="languages" 
          class="w-full max-w-xs" 
        />
      </UFormField>

      <UFormField label="Theme">
        <USelect color="primary" 
          v-model="settings.theme" 
          :items="themesOptions" 
          class="w-full max-w-xs" 
        />
      </UFormField>
    </UCard>

    <div class="h-full grid grid-cols-1 md:grid-cols-2 gap-6">
      <UCard class="flex flex-col overflow-visible">
        <template #header>
          <h3 class="text-base font-semibold">Sidebar</h3>
        </template>
        <UFormField label="Position">
          <USelect color="primary" v-model="settings.sidebar.position" :items="sidebarPositionOptions" class="w-full" />
        </UFormField>
      </UCard>

      <UCard class="flex flex-col overflow-visible">
        <template #header>
          <h3 class="text-base font-semibold">Notifications</h3>
        </template>
        <UFormField label="Position">
          <USelect color="primary" v-model="settings.notifications.position" :items="notificationPositionOptions" class="w-full" />
        </UFormField>
      </UCard>
    </div>

    <UCard class="flex flex-col overflow-visible" :ui="{ body: 'flex flex-col gap-6' }">
      <template #header>
        <h3 class="text-base font-semibold">Windows Integration</h3>
      </template>

      <div class="space-y-4">
        <h4 class="text-sm font-medium text-neutral-400">Taskbar Behavior</h4>
        <UAlert 
          icon="i-lucide-triangle-alert" 
          color="warning" 
          variant="subtle" 
          title="Attention"
          description="Changing taskbar behavior may make navigation harder." 
        />
        <UFormField label="Behavior">
          <USelect color="primary" v-model="settings.taskbar.behavior" :items="taskBarBehaviorOptions" class="w-full max-w-xs" />
        </UFormField>
      </div>

      <USeparator />

      <div class="space-y-4">
        <h4 class="text-sm font-medium text-neutral-400">Start Menu</h4>
        <UAlert icon="i-lucide-info" color="info" variant="subtle" description="Restart app to apply start menu changes." />
        <UFormField label="Behavior">
          <USelect color="primary" v-model="settings.startmenu.behavior" :items="startMenuBehaviorOptions" class="w-full max-w-xs" />
        </UFormField>
      </div>
    </UCard>

    <UCard class="flex flex-col overflow-visible" :ui="{ body: 'flex flex-col gap-4 overflow-visible' }">
      <template #header>
        <h3 class="text-base font-semibold">System & Boot</h3>
      </template>

      <UFormField label="System Tray Icon" description="Show BrickUI in the system tray">
        <USwitch v-model="settings.systemtray.enabled" />
      </UFormField>

      <UFormField 
        label="Hide on Close" 
        description="Minimize to tray instead of closing"
        :disabled="!settings.systemtray.enabled"
      >
        <template #help>
          <span class="text-xs">You can reopen it by double-clicking the tray icon.</span>
        </template>
        <USwitch v-model="settings.systemtray.hidetaskbaricon" :disabled="!settings.systemtray.enabled" />
      </UFormField>

      <UFormField label="Autostart" description="Launch BrickUI when Windows starts">
        <USwitch v-model="settings.autostart" @update:model-value="onAutoStartChanged" />
      </UFormField>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { enable as enableAutoStart, isEnabled as isAutoStartEnabled, disable as disableAutoStart } from '@tauri-apps/plugin-autostart';
import { SettingsIcon } from 'lucide-vue-next';
import Header from '#components/Header.vue';
import { 
  notificationPositions,
  sidebarPositions, 
  startMenuBehaviors, 
  taskBarBehaviors, 
  themes 
} from '#interfaces/settings';

const { settings } = useAppState();
const colorMode = useColorMode();
const { t } = useI18n();

// Trasformazione opzioni per USelect
const formatOption = (val: string) => ({
  label: val.replaceAll("-", " ").replace(/\b\w/g, c => c.toUpperCase()),
  value: val
});

const languages = [
  { label: 'English', value: 'en' },
  { label: 'Italiano', value: 'it' }
];

const themesOptions = themes.map((value) => formatOption(value));
const sidebarPositionOptions = sidebarPositions.map((value) => formatOption(value));
const notificationPositionOptions = notificationPositions.map((value) => formatOption(value));
const taskBarBehaviorOptions = taskBarBehaviors.map((value) => formatOption(value));
const startMenuBehaviorOptions = startMenuBehaviors.map((value) => formatOption(value));

const headerSections = [
  { icon: SettingsIcon, label: t("settings") }
];

onMounted(async () => {
  const actualValue = await isAutoStartEnabled();
  if (settings.value.autostart !== actualValue) {
    settings.value.autostart = actualValue;
  }
});

watch(() => settings.value.theme, (newTheme) => {
  colorMode.preference = newTheme; 
}, { immediate: true });

async function onAutoStartChanged(value: boolean) {
  value ? await enableAutoStart() : await disableAutoStart();
}
</script>