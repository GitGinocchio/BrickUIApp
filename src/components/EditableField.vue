<template>
  <div v-if="loading" class="skeleton" :style="{ width: typeof width === 'number' ? `${width}px` : width, height: typeof height === 'number' ? `${height}px` : height }"></div>
  <div class="field" v-else>
    <slot v-if="canEdit && edit && $slots['edit']" name="edit" :size="size" :width="width" :height="height" />
    <slot v-else name="view" :size="size" :width="width" :height="height" />

    <div class="actions">
      <UButton v-if="canEdit" @click="onEditClicked" variant="ghost" circle>
        <PencilOffIcon v-if="edit" :size="16" />
        <PencilIcon v-else :size="16" />
      </UButton>
      <UButton v-if="canEdit && edit && hasDiscardListener" @click="onDiscardClicked" variant="ghost" circle>
        <XCircleIcon :size="16" />
      </UButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { PencilIcon, PencilOffIcon, XCircleIcon } from 'lucide-vue-next';

const instance = getCurrentInstance()

const hasDiscardListener = computed(() => {
  return !!instance?.vnode.props?.onDiscard
})


const props = defineProps<{
  canEdit?: boolean
  loading?: boolean
  width?: string | number
  height?: string | number
  size?: 'small' | 'medium' | 'large'
}>()

const emit = defineEmits<{
  (e: 'discard'): void
  (e: 'confirm'): void
}>()

const edit = ref<boolean>(false);

function onEditClicked() {
  edit.value = !edit.value;
}

function onDiscardClicked() {
  edit.value = !edit.value;
  emit('discard');
}
</script>

<style scoped>
.field {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 1rem;
}

.field .actions {
  display: flex;
  gap: 0.5rem;
}

.skeleton {
  background: linear-gradient(90deg, rgba(255,255,255,0.04) 0%, rgba(255,255,255,0.02) 50%, rgba(255,255,255,0.04) 100%);
  border-radius: 6px;
}
</style>