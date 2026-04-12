<template>
  <NSkeleton
    v-if="loading"
    text
    :width="width"
    :height="height"
    :size="size"
    style="display: block;"
  />
  <div class="field" v-else>
    <slot v-if="canEdit && edit && $slots['edit']" name="edit" :size="size" :width="width" :height="height" />
    <slot v-else name="view" :size="size" :width="width" :height="height" />

    <div class="actions">
      <NButton v-if="canEdit" @click="onEditClicked" tertiary circle :size="size">
        <PencilOffIcon v-if="edit" :size="16" />
        <PencilIcon v-else :size="16" />
      </NButton>
      <NButton v-if="canEdit && edit && hasDiscardListener" @click="onDiscardClicked" tertiary circle :size="size">
        <XCircleIcon :size="16" />
      </NButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { PencilIcon, PencilOffIcon, XCircleIcon } from 'lucide-vue-next';
import { NButton, NSkeleton } from 'naive-ui';

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
</style>