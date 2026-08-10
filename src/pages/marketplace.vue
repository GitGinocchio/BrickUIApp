<template>
  <div class="h-full overflow-y-auto flex flex-col gap-4 m-4">
    
    <Header :sections="sections" />

    <UInput
      icon="i-lucide-search"
      size="sm"
      :placeholder="t('search_placeholder') || 'Search widgets or themes...'"
      class="max-w-md"
      :ui="{ root: 'rounded-xl' }"
    />

    <UTabs :items="tabs" class="w-full" size="sm" variant="pill">
      <template #content="{ item }">
        <div class="py-4">
          
          <div v-if="item.key === 'bricks'" class="grid grid-cols-[repeat(auto-fill,minmax(25rem,1fr))] gap-4">
             <p class="text-neutral-400 italic">{{ t('download_bricks_hint') }}</p>
          </div>

          <div v-else-if="item.key === 'walls'">
            <p class="text-neutral-400 italic">{{ t('download_themes_hint') }}</p>
          </div>

          <div v-else-if="item.key === 'wallpapers'">
            <p class="text-neutral-400 italic text-center py-10">
              {{ t('coming_soon') }}
            </p>
          </div>

        </div>
      </template>
    </UTabs>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { ShoppingBasket } from 'lucide-vue-next';
import Header from '#components/Header.vue';

const { t } = useI18n();

// Definizione delle sezioni per l'Header (Breadcrumb)
const sections = computed(() => [
  { icon: ShoppingBasket, label: t('marketplace') },
]);

// Configurazione dei Tab per UTabs
const tabs = computed(() => [
  { 
    key: 'bricks', 
    label: 'Bricks', 
    icon: 'i-lucide-cuboid' 
  },
  { 
    key: 'walls', 
    label: 'Walls', 
    icon: 'i-lucide-layout-dashboard' 
  },
  { 
    key: 'wallpapers', 
    label: 'Wallpapers', 
    icon: 'i-lucide-image' 
  }
]);
</script>

<style scoped>
/* Niente più CSS! 
   Abbiamo rimosso .brick-grid e .container perché gestiti 
   direttamente dalle utility class di Tailwind.
*/
</style>