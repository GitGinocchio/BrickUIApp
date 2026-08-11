<template>
  <div v-if="brick" class="relative flex-1 overflow-y-auto" @scroll="handleScroll">
    <Header 
      :sections="sections" 
      :style="{ 
        // Mescoliamo il colore di sfondo del tema (--ui-bg) con la trasparenza
        // Quando headerOpacity è 0, è 100% --ui-bg. Quando è 1, è l'80% del colore.
        backgroundColor: `color-mix(in srgb, var(--ui-bg), transparent ${headerOpacity * 20}%)`,
        
        // Il blur aumenta man mano che scendi
        backdropFilter: `blur(${headerOpacity * 12}px)`,
        
        // Il bordo usa il colore del bordo del tema (--ui-border) con opacità dinamica
        borderColor: `color-mix(in srgb, var(--ui-border), transparent ${(1 - headerOpacity) * 100}%)`
      }"
      class="sticky top-0 z-50 w-full transition-all duration-150 border-b px-6 py-4"
    >
      <template #actions>
        <div class="flex items-center gap-2">
          <USwitch v-model="brick.enabled" @update:model-value="toggleBrick(brick)" />
          <UDropdownMenu :items="dropdownItems">
            <UButton color="neutral" variant="ghost" icon="i-lucide-ellipsis-vertical" />
          </UDropdownMenu>
        </div>
      </template>
    </Header>

    <div class="relative w-full h-[35vh] overflow-hidden bg-neutral-900">
      <img
        :src="bannerUrl || defaultBannerUrl"
        class="w-full h-full object-cover transition-all duration-500"
        :class="[isBannerLoaded ? 'blur-0 opacity-100' : 'blur-xl opacity-50']"
        @load="onBannerLoad"
      />
      <div class="hidden absolute inset-0 bg-linear-to-b from-black/60 via-transparent to-transparent" />
    </div>

    <div class="px-6 py-4 -mt-12 relative z-10">
      <UTabs size="md" :items="tabItems" v-model="activeTabIndex" class="w-full">
        <template #info>
          <div class="relative p-4 bg-neutral-900/50 rounded-lg border border-neutral-800 min-h-200px">
            <div class="absolute top-2 right-2">
              <UButton
                :icon="descriptionEditMode ? 'i-lucide-pencil-off' : 'i-lucide-pencil'"
                variant="ghost"
                color="neutral"
                size="xs"
                @click="onEditDescription"
              />
            </div>

            <UTextarea
              v-if="descriptionEditMode"
              v-model="brick.description"
              autoresize
              :placeholder="t('placeholders.brick_description')"
              class="w-full"
            />
            <div 
              v-else-if="brick.description" 
              v-html="renderedDescription" 
              class="prose prose-invert prose-sm max-w-none description-content"
            />
            <p v-else class="text-neutral-500 italic">{{ t('bricks.no_description') }}</p>
          </div>
        </template>

        <template #props>
          <div class="p-4 bg-neutral-900/50 rounded-lg border border-neutral-800">
            <PropsPanel v-model:brick="brick" />
          </div>
        </template>

        <template #emits>
          <div class="p-8 text-center text-neutral-500 border-2 border-dashed border-neutral-800 rounded-lg">
            <UIcon name="i-lucide-wifi" class="size-8 mx-auto mb-2 opacity-20" />
            <p>{{ t('messages.work_in_progress') }}</p>
          </div>
        </template>

        <template #permissions>
          <div class="p-8 text-center text-neutral-500 border-2 border-dashed border-neutral-800 rounded-lg">
            <UIcon name="i-lucide-shield" class="size-8 mx-auto mb-2 opacity-20" />
            <p>{{ t('messages.work_in_progress') }}</p>
          </div>
        </template>

      </UTabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import PropsPanel from "~/components/props/PropsPanel.vue";

import MarkdownIt from "markdown-it";
import { appDataDir, sanitizePath } from "#utils/path";
import type { Brick } from "#interfaces";
import { invoke } from "@tauri-apps/api/core";
import { emitTo } from "@tauri-apps/api/event";
import type { DropdownMenuItem } from "@nuxt/ui";

const { t } = useI18n();
const { 
  duplicateBrick, 
  toggleBrick,
  shareBrick,
  openDeleteBrickModal, 
  openRenameBrickModal 
} = useBrickActions();
const route = useRoute();
const router = useRouter();

const brick = ref<Brick | null>(null);
const activeTabIndex = ref(0);
const isBannerLoaded = ref(false);
const descriptionEditMode = ref(false);
const headerOpacity = ref(0);
const bannerUrl = ref<string | null>(null);
const iconUrl = ref<string | null>(null);

const brickName = computed(() => route.params.name as string);
const defaultBannerUrl = new URL("../../assets/images/banner-brick-iso.svg", import.meta.url).href;
const md = new MarkdownIt();

// Configurazione Tabs per NuxtUI
const tabItems = [
  { label: t('tabs.info'), icon: 'i-lucide-info', slot: 'info' },
    { label: t('tabs.props'), icon: 'i-lucide-cog', slot: 'props' },
    { label: t('tabs.emits'), icon: 'i-lucide-wifi', slot: 'emits' },
    { label: t('tabs.permissions'), icon: 'i-lucide-shield', slot: 'permissions' }
];

// Dropdown Menu Items
const dropdownItems = computed<DropdownMenuItem[]>(() => [
  {
    label: t('actions.rename'),
    icon: 'i-lucide-pencil',
    onSelect: () => openRenameBrickModal(brick.value, true)
  },
  {
    label: t('actions.duplicate'),
    icon: 'i-lucide-copy',
    onSelect: () => duplicateBrick(brick.value, () => router.push(`/bricks/${brick.value!.name}Copy`))
  },
  { 
    label: t('actions.share'), 
    icon: 'i-lucide-share-2', 
    onSelect: () => shareBrick(brick.value)
  },
  {
    type: 'separator'
  },
  {
    label: t('actions.delete'),
    icon: 'i-lucide-trash-2',
    color: 'primary',
    onSelect: () => openDeleteBrickModal(brick.value)
  },
  {
    label: `v. ${brick.value?.version.join(".")}`,
    icon: 'i-lucide-badge-check',
    disabled: true,
    slot: 'version'
  }
]);

const sections = computed(() => [
  { icon: 'i-lucide-blocks', label: t("bricks"), to: "/bricks" },
  { icon: iconUrl.value || 'i-lucide-cuboid', label: brickName.value },
]);

const renderedDescription = computed(() => brick.value ? md.render(brick.value.description) : '');

// Methods
function handleScroll(e: Event) {
  const target = e.target as HTMLElement;
  headerOpacity.value = Math.min(target.scrollTop / 150, 1);
}

function onBannerLoad() {
  isBannerLoaded.value = true;
}

async function onEditDescription() {
  descriptionEditMode.value = !descriptionEditMode.value;
  if (!descriptionEditMode.value) {
    await invoke("save_brick", { brick: brick.value });
    await emitTo("main", "update_bricks");
  }
}

// Data Fetching
onMounted(async () => {
  brick.value = await invoke("get_brick_by_name", { name: brickName.value });
  
  if (brick.value?.banner) {
    bannerUrl.value = await sanitizePath(brick.value.banner, { root: `${appDataDir}/bricks/${brick.value.name}` });
  }

  if (brick.value?.icon) {
    iconUrl.value = await sanitizePath(brick.value.icon, { root: `${appDataDir}/bricks/${brick.value.name}` });
  }
});
</script>

<style scoped>
/* Rimuoviamo gran parte del CSS manuale a favore di Tailwind */
.description-content :deep(p) {
  margin-bottom: 1rem;
  line-height: 1.6;
}
</style>