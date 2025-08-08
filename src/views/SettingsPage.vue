<template>
  <n-card title="General Settings">
    <n-form label-placement="left" label-width="120">
      <n-form-item label="Language">
        <n-select :options="languages" v-model:value="lang" />
      </n-form-item>
      <n-form-item label="Theme">
        <n-switch v-model:value="darkMode" />
      </n-form-item>
    </n-form>
  </n-card>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const lang = ref('en')
const darkMode = ref(false)
const languages = [
  { label: 'English', value: 'en' },
  { label: 'Italiano', value: 'it' }
]
</script>
