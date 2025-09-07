<template>
  <div class="sections">
    <n-card title="General Settings">
      <n-form label-placement="left" :label-width="labelWidth">
        <n-form-item label="Language">
          <n-select :options="languages" v-model:value="settings.language" />
        </n-form-item>
        <n-form-item label="Theme">
          <n-select :options="themesOptions" v-model:value="settings.theme"></n-select>
        </n-form-item>
      </n-form>
    </n-card>
    <n-card title="Notifications">
      <n-form label-placement="left" :label-width="labelWidth">
        <n-form-item label="Position">
          <n-select :options="notificationPositionOptions" v-model:value="settings.notifications.position"></n-select>
        </n-form-item>
      </n-form>
    </n-card>
    <n-card title="Windows Taskbar">
      <n-form label-placement="left" :label-width="labelWidth">
        <n-alert type="warning">This may make navigation harder. Use with caution.</n-alert>
        <n-form-item label="Behavior">
          <n-select :options="taskBarBehaviorOptions" v-model:value="settings.taskbar.behavior"></n-select>
        </n-form-item>
      </n-form>
    </n-card>
    <n-card title="Windows Start Menu">
      <n-form label-placement="left" :label-width="labelWidth">
        <n-alert type="warning">This may make navigation harder. Use with caution.</n-alert>
        <n-alert type="warning">This setting will take effect only after restarting the app.</n-alert>
        <n-alert type="info">
          For security reasons, the Start menu may still open while interacting with this app, bricks, or other apps with higher privileges.
        </n-alert>
        <n-form-item label="Behavior">
          <n-select :options="startMenuBehaviorOptions" v-model:value="settings.startmenu.behavior"></n-select>
        </n-form-item>
      </n-form>
    </n-card>
    <n-card title="System Tray Icon">
      <n-form label-placement="left" :label-width="labelWidth">
        <n-form-item label="Enabled">
          <n-switch v-model:value="settings.systemtray.enabled"></n-switch>
        </n-form-item>
        <n-form-item label="Hide on close">
          <n-tooltip trigger="hover" placement="top">
            <template #trigger>
              <n-switch
                :disabled="!settings.systemtray.enabled"
                v-model:value="settings.systemtray.hidetaskbaricon"
              />
            </template>
            <div style="max-width: 25rem; white-space: normal;">
              Nasconde l'app dalla barra delle applicazioni quando si clicca il tasto chiudi in alto a destra. 
              Puoi riaprirla cliccando due volte con il tasto sinistro sulla system tray icon.
            </div>
          </n-tooltip>
        </n-form-item>
      </n-form>
    </n-card>
    <n-card title="Autostart">
      <n-form label-placement="left" :label-width="labelWidth">
        <n-form-item label="Enabled">
          <n-switch v-model:value="settings.autostart" @update:value="onAutoStartChanged"></n-switch>
        </n-form-item>
      </n-form>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { enable as enableAutoStart, isEnabled as isAutoStartEnabled, disable as disableAutoStart } from '@tauri-apps/plugin-autostart';
import { notificationPositions, Settings, startMenuBehaviors, taskBarBehaviors, themes } from '../interfaces/settings'
import { NSelect, NForm, NFormItem, NAlert, NSwitch, NTooltip } from 'naive-ui'
import { inject, onMounted, ref, Ref } from 'vue'

const settings = inject("settings") as Ref<Settings>;

const labelWidth = ref<string>("8rem");

const languages = [
  { label: 'English', value: 'en' },
  { label: 'Italiano', value: 'it' }
]

const themesOptions = themes.map(theme => ({
  label: theme.replaceAll("-", " ").replace(/\b\w/g, c => c.toUpperCase()),
  value: theme
}))

const notificationPositionOptions = notificationPositions.map(pos => ({
  label: pos.replaceAll("-", " ").replace(/\b\w/g, c => c.toUpperCase()),
  value: pos
}))

const taskBarBehaviorOptions = taskBarBehaviors.map(pos => ({
  label: pos.replaceAll("-", " ").replace(/\b\w/g, c => c.toUpperCase()),
  value: pos
}))

const startMenuBehaviorOptions = startMenuBehaviors.map(pos => ({
  label: pos.replaceAll("-", " ").replace(/\b\w/g, c => c.toUpperCase()),
  value: pos
}))

onMounted(async () => {
  const actualValue = await isAutoStartEnabled();
  if (settings.value.autostart !== actualValue) {
    settings.value.autostart = actualValue;
  }
});

async function onAutoStartChanged(value: boolean) {
  value ? await enableAutoStart() : await disableAutoStart();
}
</script>


<style scoped>
.sections {
  display: flex;
  flex-direction: column;
  padding: 16px;
  gap: 1rem;
}

:deep(.n-alert) {
  margin-left: v-bind("labelWidth");
}

:deep(.n-form) {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

:deep(.n-switch) {
  display: flex;
  justify-content: flex-start;
}
</style>

