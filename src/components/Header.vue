<template>
  <div class="header">
    <n-breadcrumb>
      <n-breadcrumb-item v-for="(item, index) in sections" :key="index" @click="item.onclick">
        <!-- Immagine reale -->
        <n-image
          v-if="typeof item.icon === 'string'"
          class="image"
          :class="{ loaded: loadedMap[index] || item.icon == null }"
          :show-toolbar="false"
          :preview-disabled="true"
          :src="item.icon"
          @load="() => onImageLoad(index)"
          lazy
        >
          <template #placeholder>
            <Cuboid v-show="(!loadedMap[index] && readyMap[index]) || item.icon == null" :size="46" />
          </template>
        </n-image>

        <!-- Componente icona vettoriale -->
        <component
          v-else-if="item.icon"
          :is="item.icon"
        />
        <component
          v-else
          :is="item.defaultIcon"
        />

        <h2>{{ item.label }}</h2>
      </n-breadcrumb-item>
    </n-breadcrumb>
    <slot name="actions">
      <div></div>
    </slot>
  </div>
</template>

<script setup lang="ts">
import { NBreadcrumb, NBreadcrumbItem, NImage } from 'naive-ui';
import { Cuboid } from 'lucide-vue-next';
import { PropType, ref } from 'vue';

const loadedMap = ref<Record<number, boolean>>({});
const readyMap = ref<Record<number, boolean>>({});

function onImageLoad(index: number) {
  // piccolo delay per permettere transizione CSS
  setTimeout(() => {
    loadedMap.value[index] = true;
  }, 15);

  readyMap.value[index] = true;
}


defineProps({
  sections: {
    type: Object as PropType<{ icon?: any, defaultIcon?: any, label: string, onclick?: () => void }[]>,
    default: []
  }
});
</script>

<style scoped>
.header {
  display: flex;
  flex-direction: row;
  align-items: baseline;
  justify-content: space-between;
  margin-left: 0.5rem;
  margin-right: 0.5rem;
  margin-top: 1rem;
}

.header div {
  display: flex;
  align-items: center;
  flex-direction: row;
  padding-bottom: 0;
  gap: 0.5rem;
}

.header h2 {
  margin: 0;
}

.image {
  width: 24px;
  height: 24px;
  border-radius: 0.25rem;

  transition: opacity 0.3s ease, filter 0.3s ease;
  opacity: 0.5;
  filter: blur(5px);
}

.image img {
  width: 24px;
  height: 24px;
  border-radius: 0.25rem;
}

.image.loaded {
  opacity: 1;
  filter: blur(0);
}

:deep(.n-breadcrumb ul) {
  display: flex;
  flex-direction: row;
}

:deep(.n-breadcrumb-item__link) {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
</style>