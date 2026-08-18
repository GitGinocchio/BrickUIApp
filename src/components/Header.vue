<template>
  <header class="flex items-center justify-between px-2 shrink-0">
    <div class="flex items-center gap-4">
      <UButton
        icon="i-lucide-panel-left"
        color="primary"
        variant="soft"
        class="lg:hidden -ml-2"
        @click="isSidebarOpen = true"
        v-if="settings.sidebar.position === 'left'"
      />

      <UBreadcrumb :items="breadcrumbItems">
        
        <template #item-leading="{ item, index }">
          <div v-if="isImageUrl(item.icon)" class="relative w-6 h-6 rounded overflow-hidden shrink-0">
            <img 
              :src="item.icon" 
              :class="[
                'w-6 h-6 object-cover transition-all duration-300',
                loadedMap[index] ? 'opacity-100 blur-0' : 'opacity-50 blur-sm'
              ]"
              @load="onImageLoad(index)"
            />
          </div>

          <div v-else-if="item.icon" class="flex items-center justify-center shrink-0">
            <component
              class="size-5 text-neutral-500" 
              :is="item.icon" 
              v-if="typeof item.icon !== 'string'"
            />

            <UIcon
              class="size-5 text-neutral-500" 
              v-else 
              :name="item.icon"
            />
          </div>
        </template>

        <template #item-label="{ item }">
          <div class="flex items-center h-8"> 
            <UButton 
              v-if="item.to" 
              variant="ghost"
              color="primary"
              class="text-lg text-neutral font-semibold px-2 py-1"
              :ui="{ label: 'font-semibold text-lg' }"
              @click="item.click"
            >
              {{ item.label }}
            </UButton>
            
            <span 
              v-else 
              class="text-lg text-neutral-500 font-semibold whitespace-nowrap px-2 py-1"
            >
              {{ item.label }}
            </span>
          </div>
        </template>

        <template #separator>
          <UIcon name="i-lucide-chevron-right" class="size-4 text-neutral-500 mx-1" />
        </template>
        
      </UBreadcrumb>
    </div>

    <div class="flex items-center gap-4">
      <slot name="actions" />
      <UButton
        icon="i-lucide-panel-left"
        color="primary"
        variant="soft"
        class="lg:hidden -ml-2"
        @click="isSidebarOpen = true"
        v-if="settings.sidebar.position === 'right'"
      />
    </div>
  </header>
</template>

<script setup lang="ts">
import { type BreadcrumbItem } from '@nuxt/ui';

const { isSidebarOpen, settings } = useAppState();

const props = defineProps<{
  sections: { icon?: any, label: string, to?: string }[]
}>();

const loadedMap = ref<Record<number, boolean>>({});

const breadcrumbItems = computed<BreadcrumbItem[]>(() => {
  return props.sections.map((section) => ({
    label: section.label,
    icon: section.icon,
    // Nuxt UI usa 'click' per le funzioni
    to: section.to
  }));
});

function isImageUrl(icon: any): icon is string {
  return typeof icon === 'string' && (icon.includes('/') || icon.startsWith('data:'));
}

function onImageLoad(index: number) {
  loadedMap.value[index] = true;
}
</script>