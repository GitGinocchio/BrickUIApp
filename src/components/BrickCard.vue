<template>
  <n-card :title="brick.name" :segmented="true" hoverable>
    <div v-if="brick.icon" class="brick-icon">
      <img :src="brick.icon" alt="Brick Icon" />
    </div>

    <p v-if="brick.description">{{ brick.description }}</p>

    <n-space justify="space-between" align="center" class="mb-3">
      <n-switch 
        v-model:value="brick.enabled" 
        @update:value="$emit('toggle')"
        />
      <n-button size="small" @click="$emit('open-settings', brick.name)">Settings</n-button>
    </n-space>

    <div v-if="brick.tags.length">
      <strong>Tags:</strong>
      <n-space>
        <n-tag v-for="tag in brick.tags" :key="tag" type="info">{{ tag }}</n-tag>
      </n-space>
    </div>

    <p v-if="brick.license"><strong>License:</strong> {{ brick.license }}</p>
    <p v-if="brick.author"><strong>Author:</strong> {{ brick.author }}</p>
    <p>
      <strong>Version:</strong> {{ brick.version.join('.') }}
    </p>
    <p v-if="brick.dependencies.length">
      <strong>Dependencies:</strong> {{ brick.dependencies.join(', ') }}
    </p>

    <div v-if="brick.props.length" class="props-section">
      <h4>Properties</h4>
      <div v-for="(prop, i) in brick.props" :key="i" class="prop-item">
        <strong>{{ prop.prop_type }}:</strong>
        <template v-if="prop.prop_type === 'string'">
          <em>Value:</em> {{ prop.value }}, <em>Default:</em> {{ prop.default }}
        </template>
        <template v-else-if="prop.prop_type === 'int'">
          <em>Value:</em> {{ prop.value }}, <em>Default:</em> {{ prop.default }}
        </template>
        <template v-else-if="prop.prop_type === 'bool'">
          <em>Value:</em> {{ prop.value ? 'true' : 'false' }}, <em>Default:</em> {{ prop.default ? 'true' : 'false' }}
        </template>
        <template v-else-if="prop.prop_type === 'array'">
          <em>Value:</em> [{{ prop.value.join(', ') }}], <em>Default:</em> [{{ prop.default.join(', ') }}]
        </template>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { NSwitch, NSpace } from 'naive-ui';
import { Brick } from 'interfaces/brick'

defineProps<{
  brick: Brick
}>()

defineEmits(['open-settings', 'toggle'])
</script>

<style scoped>
.brick-icon img {
  max-width: 48px;
  max-height: 48px;
  margin-bottom: 8px;
}
.props-section {
  margin-top: 1rem;
}
.prop-item {
  margin-bottom: 0.5rem;
}
</style>
