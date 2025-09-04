<template>
  <n-card class="brick-card" :segmented="true" hoverable @click="openTemplate">
    <!-- Header -->
    <div class="brick-header">
      <div class="brick-title-icon">
        <div class="brick-icon">
          <img v-if="brick.icon" :src="brick.icon" alt="icon-icon" />
        </div>
        <div class="brick-title">
          <strong>{{ brick.name }}</strong>
        </div>
      </div>
      <n-button text circle size="tiny" @click="openLink">
        <ExternalLink :size="18" />
      </n-button>
    </div>

        <!-- Tags -->
    <n-space class="brick-tags" size="small" wrap>
      <n-tag round v-for="tag in brick.tags" :key="tag" type="info">{{ tag }}</n-tag>
    </n-space>

    <!-- Description -->
    <div class="brick-description">
      <span>{{ brick.description }}</span>
    </div>

    <div class="version-download">
      <!-- Version -->
      <span class="brick-version" v-if="brick.version">(v {{ brick.version }})</span>
      <!-- Install icon-->
      <div class="download-icon">
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
import {
  NSpace,
  NTag,
  NCard,
  NIcon,
  NButton,
  NSwitch,
  NCollapse,
  NCollapseItem
} from "naive-ui"
import { ExternalLink, CloudDownload, Download } from "lucide-vue-next"


interface Brick {
  name: string
  icon?: string
  description: string
  category?: string
  version?: string
  link?: string
  tags?: string[]
}

const props = defineProps<{
  brick: Brick
}>()

const openLink = () => {
  if (props.brick.link) {
    window.open(props.brick.link, "_blank")
  }
}

const openTemplate = () => {
  // funzione che aprirà la pagina dettagliata del brick
}
</script>

<style scoped>
.brick-card {
  display: flex;
  flex-direction: column;
  cursor: pointer;
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


.brick-icon img {
  width: 16px;
  height: 16px;
  border-radius: 6px;
}

.brick-title {
  display: flex;
  align-items: center;
  gap: 6px;
}

.brick-description {
  margin-top: 10px;
  font-size: 14px;
  color: #666;

  display: -webkit-box;
  -webkit-line-clamp: 3; /* numero di righe visibili */
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

.brick-tags {
  margin-top: 8px;
}

.version-download{
  display: flex;
  justify-content: space-between;
  align-items: baseline;
}

.version-download span{
  color: #666;
  font-weight: bold;
}

.download-icon{
  display: flex;
  margin-top: 15px;
}

</style>
