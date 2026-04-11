<template>
  <n-card class="brick-card" :segmented="true" hoverable @click.prevent="openBrickTab">
    <!-- Header -->
    <div class="brick-header">
      <div class="brick-title-icon">
        <n-image
          class="brick-icon"
          :class="{ loaded: isIconLoaded || brick.icon == null }"
          :show-toolbar="false"
          :preview-disabled="true"
          :src="iconUrl"
          @load="onIconLoad"
          lazy
        >
          <template #placeholder>
            <Cuboid v-show="(!isIconLoaded && canShowDefaultIcon) || brick.icon == null" :size="46" />
          </template>
        </n-image>
        <div class="brick-title">
          <strong>{{ brick.name }}</strong>
        </div>
        <n-button text circle size="tiny" @click.stop="onOpenBrick">
          <ExternalLink :size="18" />
        </n-button>
      </div>
      <div class="brick-controls" v-if="offline">
        <n-switch v-model:value="brick.enabled" @update:value="toggleBrick(brick)" @click.stop="() => {}" />
        <n-dropdown :options="brickOptions" trigger="click" :animated="true" @select="handleBrickAction">
          <n-button text circle @click.stop="() => {}"><MoreVertical/></n-button>
        </n-dropdown>
      </div>
    </div>

    <!-- Tags -->
    <n-space class="brick-tags" size="small" wrap>
      <n-tag round v-for="tag in brick.tags" :key="tag" type="info">{{ tag }}</n-tag>
    </n-space>

    <!-- Description -->
    <div class="brick-description">
      <span v-if="brick.description">{{ brick.description }}</span>
      <span v-else>Nessuna descrizione</span>
    </div>

    <div class="footer">
      <span v-if="brick.version">(v {{ brick.version.join('.') }})</span>

      <div class="download-icon" v-if="!offline">
        <div class="icon">
          <n-button strong circle>
            <template #icon>
              <n-icon><Download /></n-icon>
            </template>
          </n-button>
        </div>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { NSpace, NTag, NCard, NIcon, NButton, NDropdown, NImage } from "naive-ui"
import { ExternalLink, Download, Trash2, Copy, MoreVertical, Cuboid, Share2, Pencil } from "lucide-vue-next"
import type { Brick } from "../interfaces/brick";
import { invoke } from "@tauri-apps/api/core";
import { appDataDir, sanitizePath } from "#utils/path";
import { useBrickActions } from "~/composables/useBrickActions";
const router = useRouter();
const { bricks, theme } = useAppState();
const { renameBrick, deleteBrick, toggleBrick, duplicateBrick, shareBrick } = useBrickActions(bricks, theme);

const props = defineProps({
  brick: {
    type: Object as PropType<Brick>,
    required: true
  },
  offline: {
    type: Boolean,
    required: false,
    default: false
  }
});

const canShowDefaultIcon = ref<boolean>(false);
const isIconLoaded = ref<boolean>(false);
const iconUrl = ref<string | null>(null);

function onIconLoad() {
  setTimeout(() => { 
    isIconLoaded.value = true; 
    canShowDefaultIcon.value = true;
  }, 15);
}

onMounted(async () => {
  await nextTick();
  
  if (props.brick.icon) {
    iconUrl.value = await sanitizePath(props.brick.icon, { root: `${appDataDir}/bricks/${props.brick.name}` });
  }
});

const brickOptions = ref([
  { label: 'Rename', key: 'rename', icon: () => h(Pencil) },
  { label: 'Delete', key: 'delete', type: 'error', icon: () => h(Trash2) },
  { label: 'Duplicate', key: 'duplicate', icon: () => h(Copy) },
  { label: 'Share', key: 'share', icon: () => h(Share2)}
]);

async function handleBrickAction(action: string) {
  switch (action) {
    case "rename":
      await renameBrick(props.brick);
      break;
    case "delete":
      await deleteBrick(props.brick);
      break;
    case "duplicate":
      duplicateBrick(props.brick);
      break;
    case "share":
      await shareBrick(props.brick);
      break;
    default:
      console.error(`azione non riconosciuta: ${action}`);
  }
}

async function openBrickTab() {
  await router.push(`bricks/${props.brick.name}`)
}

async function onOpenBrick() {
  if (props.offline) {
    await invoke("open_brick", { brickName: props.brick.name });
  }
}
</script>

<style scoped>
.brick-card {
  display: flex;
  flex-direction: column;
  cursor: pointer;
}

::v-deep(.n-card__content) {
  display: flex;
  flex-direction: column;
}

.brick-card:hover {
  box-shadow: var(--n-box-shadow-hover, 0 4px 20px rgba(0,0,0,0.25));
  background-color: var(--n-color-hover);
  transform: translateY(-2px);
}

.brick-header {
  display: flex;
  align-items: center;
  gap: 10px;
  justify-content: space-between;
}

.brick-title-icon{
  display: flex;
  gap: 6px;
}

.brick-icon {
  width: 46px;
  height: 46px;
  display: flex;

  filter: blur(var(--blur, 5px));
  transition: filter 0.5s ease;
}

.brick-icon img {
  width: 46px;
  height: 46px;
  border-radius: 6px;
}

.brick-icon.loaded  {
  --blur: 0px;
}

.brick-title {
  display: flex;
  align-items: center;
  gap: 6px;
}

.brick-description {
  display: flex;
  flex: 1;
  margin-top: 10px;
  font-size: 14px;
  color: #666;

  display: -webkit-box;
  line-clamp: 3;
  -webkit-line-clamp: 3; /* numero di righe visibili */
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

.brick-tags {
  margin-top: 8px;
}

.brick-controls {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 0.5rem;
}

.footer {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-top: 1rem;
}

.footer span{
  color: #666;
  font-weight: bold;
}

.download-icon{
  display: flex;
}

</style>
