<template>
  <div class="header">
    <n-breadcrumb>
      <n-breadcrumb-item v-for="item in sections" @click="item.onclick">
        <component v-if="item.defaultIcon" :is="item.defaultIcon" v-show="!loaded" />
        <img
          v-if="typeof item.icon === 'string'"
          v-show="loaded"
          :src="item.icon"
          :height="24"
          class="image"
          alt="brick icon"
          @load="loaded = true"
          @error="loaded = false"
        />
        <component v-else :is="item.icon" />
        <h2>{{ item.label }}</h2>
      </n-breadcrumb-item> 
    </n-breadcrumb>
    <slot name="actions">
      <div></div>
    </slot>
  </div>
</template>

<script setup lang="ts">
import { NBreadcrumb, NBreadcrumbItem } from 'naive-ui';
import { PropType, ref } from 'vue';

const loaded = ref<boolean>(false);

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
  border-radius: 0.25rem;
}

:deep(.n-breadcrumb-item__link) {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
</style>