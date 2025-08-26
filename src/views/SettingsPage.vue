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
        <n-form-item label="Behavior">
          <n-select :options="taskBarBehaviorOptions" v-model:value="settings.taskbar.behavior"></n-select>
        </n-form-item>
      </n-form>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { NSelect, NForm, NFormItem } from 'naive-ui'
import { inject, Ref } from 'vue'

import { notificationPositions, Settings, taskBarBehaviors, themes } from '../interfaces/settings'

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
</script>


<style scoped>
.sections {
  display: flex;
  flex-direction: column;
  padding: 16px;
  gap: 1rem;
}
</style>

