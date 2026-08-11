<template>
  <div class="flex flex-col h-full overflow-y-auto p-4 gap-6">
    
    <Header :sections="headerSections" />

    <UCard class="flex flex-col overflow-visible w-full">
      <template #header>
        <h3 class="text-base font-semibold leading-6">{{ t('settings.general.title') }}</h3>
      </template>

      <UFormField :label="t('settings.language')" :help="t('settings.language_help')" class="mb-4">
        <USelect color="primary" 
          v-model="settings.language" 
          :items="languages" 
          class="w-full max-w-xs" 
        />
      </UFormField>

      <UFormField :label="t('settings.theme')">
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
          <h3 class="text-base font-semibold">{{ t('settings.sidebar.title') }}</h3>
        </template>
        <UFormField :label="t('settings.position')">
          <USelect color="primary" v-model="settings.sidebar.position" :items="sidebarPositionOptions" class="w-full" />
        </UFormField>
      </UCard>

      <UCard class="flex flex-col overflow-visible">
        <template #header>
          <h3 class="text-base font-semibold">{{ t('settings.notifications.title') }}</h3>
        </template>
        <UFormField :label="t('settings.position')">
          <USelect color="primary" v-model="settings.notifications.position" :items="notificationPositionOptions" class="w-full" />
        </UFormField>
      </UCard>
    </div>

    <UCard class="flex flex-col overflow-visible" :ui="{ body: 'flex flex-col gap-6' }">
      <template #header>
        <h3 class="text-base font-semibold">{{ t('settings.windows_integration') }}</h3>
      </template>

      <div class="space-y-4">
        <h4 class="text-sm font-medium text-neutral-400">{{ t('settings.taskbar.title') }}</h4>
        <UAlert 
          icon="i-lucide-triangle-alert" 
          color="warning" 
          variant="subtle" 
          :title="t('settings.taskbar.attention')"
          :description="t('settings.taskbar.change_warning')" 
        />
        <UFormField :label="t('settings.behavior')">
          <USelect color="primary" v-model="settings.taskbar.behavior" :items="taskBarBehaviorOptions" class="w-full max-w-xs" />
        </UFormField>
      </div>

      <USeparator />

      <div class="space-y-4">
        <h4 class="text-sm font-medium text-neutral-400">{{ t('settings.start_menu.title') }}</h4>
        <UAlert icon="i-lucide-info" color="info" variant="subtle" :description="t('settings.start_menu.restart_hint')" />
        <UFormField :label="t('settings.behavior')">
          <USelect color="primary" v-model="settings.startmenu.behavior" :items="startMenuBehaviorOptions" class="w-full max-w-xs" />
        </UFormField>
      </div>
    </UCard>

    <UCard class="flex flex-col overflow-visible" :ui="{ body: 'flex flex-col gap-4 overflow-visible' }">
      <template #header>
        <h3 class="text-base font-semibold">System & Boot</h3>
      </template>

      <UFormField :label="t('settings.system_tray.icon')" :description="t('settings.system_tray.description')">
        <USwitch v-model="settings.systemtray.enabled" />
      </UFormField>

      <UFormField 
              :label="t('settings.system_tray.hide_on_close')" 
              :description="t('settings.system_tray.hide_description')"
        :disabled="!settings.systemtray.enabled"
      >
        <template #help>
                <span class="text-xs">{{ t('settings.system_tray.reopen_hint') }}</span>
        </template>
        <USwitch v-model="settings.systemtray.hidetaskbaricon" :disabled="!settings.systemtray.enabled" />
      </UFormField>

      <UFormField :label="t('settings.autostart.title')" :description="t('settings.autostart.description')">
        <USwitch v-model="settings.autostart" @update:model-value="onAutoStartChanged" />
      </UFormField>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { enable as enableAutoStart, isEnabled as isAutoStartEnabled, disable as disableAutoStart } from '@tauri-apps/plugin-autostart';
import { SettingsIcon } from 'lucide-vue-next';
import Header from '#components/Header.vue';
import { NOTIFICATION_POSITIONS, SIDEBAR_POSITIONS, STARTMENU_BEHAVIORS, TASKBAR_BEHAVIORS, THEMES } from '~/constants/settings';

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

const themesOptions = THEMES.map((value) => formatOption(value));
const sidebarPositionOptions = SIDEBAR_POSITIONS.map((value) => formatOption(value));
const notificationPositionOptions = NOTIFICATION_POSITIONS.map((value) => formatOption(value));
const taskBarBehaviorOptions = TASKBAR_BEHAVIORS.map((value) => formatOption(value));
const startMenuBehaviorOptions = STARTMENU_BEHAVIORS.map((value) => formatOption(value));

const headerSections = [
  { icon: SettingsIcon, label: t("settings.title") }
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