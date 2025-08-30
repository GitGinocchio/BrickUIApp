<template>
  <div class="sections">
    <n-card title="General Settings">
      <n-form label-placement="left" label-width="100">
        <n-form-item label="Language">
          <n-select :options="languages" v-model:value="settings.language" />
        </n-form-item>
        <n-form-item label="Theme">
          <n-select :options="themesOptions" v-model:value="settings.theme"></n-select>
        </n-form-item>
      </n-form>
    </n-card>
    <n-card title="Notifications">
      <n-form label-placement="left" label-width="100">
        <n-form-item label="Position">
          <n-select :options="notificationPositionOptions" v-model:value="settings.notifications.position"></n-select>
        </n-form-item>
      </n-form>
    </n-card>
    <n-card title="Windows Taskbar">
      <n-form label-placement="left" label-width="100">
        <n-alert type="warning">This may make navigation harder. Use with caution.</n-alert>
        <n-form-item label="Behavior">
          <n-select :options="taskBarBehaviorOptions" v-model:value="settings.taskbar.behavior"></n-select>
        </n-form-item>
      </n-form>
    </n-card>
    <n-card title="Windows Start Menu">
      <n-form label-placement="left" label-width="100">
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
  </div>
</template>

<script setup lang="ts">
import { NSelect, NForm, NFormItem, NAlert } from 'naive-ui'
import { inject, Ref } from 'vue'

import { notificationPositions, Settings, startMenuBehaviors, taskBarBehaviors, themes } from '../interfaces/settings'

const settings = inject("settings") as Ref<Settings>;

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
</script>


<style scoped>
.sections {
  display: flex;
  flex-direction: column;
  padding: 16px;
  gap: 1rem;
}

:deep(.n-alert) {
  margin-left: 100px;
}

:deep(.n-form) {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
</style>

