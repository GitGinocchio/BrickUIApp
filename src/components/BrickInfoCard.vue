<template>
  <UCard 
    class="brick-card transition-all duration-300 hover:-translate-y-1 hover:shadow-xl cursor-pointer group"
    :ui="{  body: 'p-4 flex flex-col h-full', footer: 'px-4 py-3 bg-gray-50/50 dark:bg-gray-800/50' }"
    @click="openBrickTab"
  >
    <!-- Header -->
    <div class="flex items-start justify-between gap-3 mb-4">
      <div class="flex items-center gap-3">
        <!-- Icona con effetto Blur caricamento -->
        <div class="relative w-12 h-12 shrink-0">
          <img
            v-if="iconUrl"
            :src="iconUrl"
            class="w-full h-full rounded-lg object-cover transition-all duration-500"
            :class="isIconLoaded ? 'blur-0 opacity-100' : 'blur-md opacity-0'"
            @load="onIconLoad"
          />
          <div 
            v-if="!isIconLoaded || !iconUrl" 
            class="absolute inset-0 flex items-center justify-center bg-gray-100 dark:bg-gray-800 rounded-lg"
          >
            <UIcon name="i-lucide-cuboid" class="w-8 h-8 text-gray-400" />
          </div>
        </div>

        <div class="flex flex-col">
          <div class="flex items-center gap-1">
            <strong class="text-lg leading-tight">{{ brick.name }}</strong>
            <UButton 
              icon="i-lucide-external-link" 
              variant="ghost" 
              color="neutral"
              size="xs"
              class="opacity-0 group-hover:opacity-100 transition-opacity"
              @click.stop="onOpenBrick" 
            />
          </div>
        </div>
      </div>

      <!-- Controls (Solo Offline/Locale) -->
      <div v-if="offline" class="flex items-center gap-1" @click.stop>
        <USwitch 
          v-model="brick.enabled" 
          size="sm" 
          @update:model-value="toggleBrick(brick)" 
        />
        <UDropdownMenu :items="dropdownItems" :popper="{ placement: 'bottom-end' }">
          <UButton icon="i-lucide-more-vertical" variant="ghost" color="neutral" />
        </UDropdownMenu>
      </div>
    </div>

    <!-- Tags -->
    <div class="flex flex-wrap gap-1 mb-3">
      <UBadge 
        v-for="tag in brick.tags" 
        :key="tag" 
        variant="subtle" 
        size="md"
        class="rounded-full"
      >
        {{ tag }}
      </UBadge>
    </div>

    <!-- Description -->
    <p class="text-sm text-gray-500 dark:text-gray-400 line-clamp-3 flex-1">
      {{ brick.description || 'Nessuna descrizione' }}
    </p>

    <!-- Footer -->
    <template #footer>
      <div class="flex items-center justify-between">
        <span class="text-xs font-bold text-gray-400">
          {{ brick.version ? `v ${brick.version.join('.')}` : '' }}
        </span>

        <div v-if="!offline" class="download-section">
          <UButton 
            icon="i-lucide-download" 
            size="sm" 
            color="primary" 
            variant="soft" 
            class="rounded-full" 
          />
        </div>
      </div>
    </template>
  </UCard>
</template>

<script setup lang="ts">
import type { Brick } from "~/interfaces/brick";
import { invoke } from "@tauri-apps/api/core";
import { appDataDir, sanitizePath } from "#utils/path";
import { useBrickActions } from "~/composables/useBrickActions";
import { type DropdownMenuItem } from "@nuxt/ui";

// Props
const props = defineProps<{
  brick: Brick;
  offline?: boolean;
}>();

// State & Composables
const router = useRouter();
const { toggleBrick, duplicateBrick, shareBrick, openRenameBrickModal, openDeleteBrickModal } = useBrickActions();

const isIconLoaded = ref(false);
const iconUrl = ref<string | null>(null);

// Dropdown Menu items (Configurazione Nuxt UI)
const dropdownItems = computed<DropdownMenuItem[][]>(() => [
  [
    { 
      label: 'Rename', 
      icon: 'i-lucide-pencil', 
      onSelect: () => openRenameBrickModal(props.brick, null)
    },
    { 
      label: 'Duplicate', 
      icon: 'i-lucide-copy', 
      onSelect: () => duplicateBrick(props.brick) 
    },
    { 
      label: 'Share', 
      icon: 'i-lucide-share-2', 
      onSelect: () => shareBrick(props.brick)
    }
  ],
  [
    { 
      label: 'Delete', 
      icon: 'i-lucide-trash-2', 
      // In Nuxt UI v4 si usa spesso la proprietà class per lo stile 
      // o slot specifici se il tipo base non prevede 'color' direttamente
      class: 'text-red-500 dark:text-red-400', 
      onSelect: () => openDeleteBrickModal(props.brick)
    }
  ]
]);

function onIconLoad() {
  setTimeout(() => { isIconLoaded.value = true; }, 50);
}

onMounted(async () => {
  if (props.brick.icon) {
    iconUrl.value = await sanitizePath(props.brick.icon, { 
      root: `${appDataDir}/bricks/${props.brick.name}` 
    });
  }
});

async function openBrickTab() {
  await router.push(`/bricks/${props.brick.name}`);
}

async function onOpenBrick() {
  if (props.offline) {
    await invoke("open_brick", { brickName: props.brick.name });
  }
}
</script>