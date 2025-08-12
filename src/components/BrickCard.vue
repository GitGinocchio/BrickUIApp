<template>
  <n-card class="card" :segmented="true" hoverable>
    <template #header>
      <n-space justify="space-between" align="center" class="w-full">
        <div class="brick-title">
          <strong>{{ brick.name }}</strong> - {{ brick.author }}
          <span class="brick-version">(v {{ brick.version.join('.') }})</span>
        </div>
        <n-switch v-model:value="brick.enabled" @update:value="$emit('toggle')" />
      </n-space>
      <n-space>
        <n-tag round v-for="tag in brick.tags" :key="tag" type="info">{{ tag }}</n-tag>
      </n-space>
      <p v-if="brick.description" class="description">{{ brick.description }}</p>
    </template>

    <!-- Icona -->
    <div v-if="brick.icon" class="brick-icon">
      <img :src="brick.icon" alt="Brick Icon" />
    </div>

    <!-- Info generali -->
    <!--
    <div class="info-grid">
      <div v-if="brick.license"><strong>License:</strong> {{ brick.license }}</div>
      <div v-if="brick.dependencies.length">
        <strong>Dependencies:</strong> {{ brick.dependencies.join(', ') }}
      </div>
    </div>
    -->

    <n-collapse>
      <n-collapse-item title="Props" class="properties-container">
        <div v-for="(prop, i) in brick.props" :key="i">
          <BrickProp :prop="prop" />
        </div>
      </n-collapse-item>
      <n-collapse-item title="Emits" class="emits-container">
        <div v-for="(prop, i) in brick.props" :key="i">

        </div>
      </n-collapse-item>
    </n-collapse>

    <!-- Proprietà -->
    <!--
    <div v-if="brick.props.length" class="props-section">
      <h4>Properties</h4>
      <div v-for="(prop, i) in brick.props" :key="i" class="prop-item">
        <strong>{{ prop.prop_type }}:</strong>
        <template v-if="['string', 'int'].includes(prop.prop_type)">
          <em>Value:</em> {{ prop.value }}, <em>Default:</em> {{ prop.default }}
        </template>
        <template v-else-if="prop.prop_type === 'bool'">
          <em>Value:</em> {{ prop.value ? 'true' : 'false' }},
          <em>Default:</em> {{ prop.default ? 'true' : 'false' }}
        </template>
        <template v-else-if="prop.prop_type === 'array'">
          <em>Value:</em> [{{ prop.value.join(', ') }}],
          <em>Default:</em> [{{ prop.default.join(', ') }}]
        </template>
      </div>
    </div>
    -->
  </n-card>
</template>

<script setup lang="ts">
import { NSwitch, NSpace, NTag, NCard, NCollapse, NCollapseItem } from 'naive-ui'
import { Brick } from 'interfaces/brick'
import BrickProp from './BrickProp.vue';

defineProps<{ brick: Brick }>()
defineEmits(['toggle'])
</script>

<style scoped>
::v-deep(.n-card-header) {
  padding-bottom: 0.5rem !important;
}

.properties-container { 
  font-weight: bold;
}


.brick-icon {
  text-align: center;
  margin-bottom: 8px;
}
.brick-icon img {
  max-width: 48px;
  max-height: 48px;
}

.brick-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.brick-version {
  color: #888;
  font-size: 0.9rem;
}

.description {
  font-size: 1rem; 
  font-weight: normal;
  margin-top: 0rem;
  margin-bottom: 0.5rem;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.props-section {
  padding-top: 0.0rem;
}
</style>
