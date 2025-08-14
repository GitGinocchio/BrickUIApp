<template>
  <n-card class="card" :segmented="true" hoverable>
    <template #header>
      <n-space justify="space-between" align="center" class="w-full">
        <div v-if="brick.icon" class="brick-icon">
          <img :src="brick.icon" alt="Brick Icon" />
        </div>
        <div class="brick-title">
          <strong>{{ brick.name }}</strong> - {{ brick.author }}
          <n-button text circle @click="onOpenBrick"><ExternalLink :size="18" /></n-button>
          <span class="brick-version">(v {{ brick.version.join('.') }})</span>
          <span class="brick-version" v-if="brick.license">License: {{ brick.license }}</span>
        </div>
        <n-switch v-model:value="brick.enabled" @update:value="onToggle" />
      </n-space>
      <n-space>
        <n-tag round v-for="tag in brick.tags" :key="tag" type="info">{{ tag }}</n-tag>
      </n-space>
    </template>

    <!-- Info generali -->
    <!--
    <div v-if="brick.dependencies.length">
      <strong>Dependencies:</strong> {{ brick.dependencies.join(', ') }}
    </div>
    -->

    <n-collapse>
      <n-collapse-item :collapsed="false" title="Description">
        <div v-html="renderedDescription" class="description"></div>
      </n-collapse-item>
      <n-collapse-item title="Props" class="properties-container">
        <div v-for="prop in brick.props" :key="prop.prop_name">
          <BrickProp :prop="prop" @update:prop="onPropUpdate" />
        </div>
      </n-collapse-item>
      <n-collapse-item title="Emits" class="emits-container">
        <div v-for="prop in brick.props" :key="prop.prop_name">

        </div>
      </n-collapse-item>
    </n-collapse>
  </n-card>
</template>

<script setup lang="ts">
import { NSwitch, NSpace, NTag, NCard, NCollapse, NCollapseItem, NButton } from 'naive-ui'
import { ExternalLink } from 'lucide-vue-next';
import { Brick, Prop } from 'interfaces/brick'
import BrickProp from './BrickProp.vue';
import { emitTo } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { computed } from 'vue';
import MarkdownIt from 'markdown-it';

const md = new MarkdownIt();

const renderedDescription = computed(() => md.render(brick.description));

const { brick } = defineProps<{ brick: Brick }>()

async function onToggle() {
  await emitTo("window", "toggle_brick", { brick: brick });
  await invoke("save_brick", { brick: brick });
}

async function onPropUpdate(prop: Prop) {
  const index = brick.props.findIndex(p => p.prop_name === prop.prop_name);
  if (index !== -1) brick.props[index] = prop;
  else brick.props.push(prop);

  await invoke("save_brick", { brick: brick });

  await emitTo("window", "update_prop", { brick_name: brick.name, prop_name: prop.prop_name, prop_value: prop.value })
}

async function onOpenBrick() {
  await invoke("open_brick", { brickName: brick.name });
}

</script>

<style scoped>
::v-deep(.n-card-header) {
  padding-bottom: 0.5rem !important;
}

::v-deep(.n-collapse .n-collapse-item .n-collapse-item__content-wrapper .n-collapse-item__content-inner) {
  padding-top: 0;
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
