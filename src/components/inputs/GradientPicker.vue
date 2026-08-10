<template>
  <div class="flex items-center gap-3 w-full">
    <div class="relative flex-1 h-9 bg-neutral-900 border border-neutral-700 rounded-md p-1 group">
      
      <div 
        ref="previewRef"
        class="relative h-full w-full rounded sm overflow-hidden cursor-crosshair"
        :style="{ background: gradientString }"
        @dblclick.prevent="addStopAt($event)"
      >
        <div class="absolute inset-0 -z-10 opacity-20" :style="checkerboardStyle" />

        <div 
          v-for="(stop, index) in steps" 
          :key="index"
          class="absolute top-1/2 -translate-y-1/2 -translate-x-1/2 z-20"
          :style="{ left: stop.position + '%' }"
        >
          <UPopover :ui="{ content: 'p-3 w-48 flex flex-col gap-3' }">
            <button
              class="size-4 rounded-full border-2 border-white shadow-lg cursor-grab active:cursor-grabbing ring-1 ring-black/20"
              :style="{ backgroundColor: stop.color }"
              @mousedown.prevent="startDrag(index, $event)"
            />

            <template #content>
              <div class="space-y-3">
                <div class="flex flex-col gap-1.5">
                  <span class="text-xs font-medium text-neutral-400">Color</span>
                  <UInput 
                    v-model="stop.color" 
                    type="color" 
                    size="xs" 
                    variant="outline"
                    @update:model-value="(val) => onSetColor(index, val)"
                  />
                </div>

                <div class="flex flex-col gap-1.5">
                  <span class="text-xs font-medium text-neutral-400">Position (%)</span>
                  <UInput
                    v-model.number="stop.position"
                    type="number"
                    size="xs"
                    :step="0.5"
                    :min="0"
                    :max="100"
                  />
                </div>

                <div class="flex gap-2 pt-2 border-t border-neutral-800">
                  <UButton
                    icon="i-lucide-copy"
                    size="xs"
                    variant="ghost"
                    color="neutral"
                    class="flex-1"
                    @click="duplicateStop(index)"
                  />
                  <UButton
                    icon="i-lucide-trash"
                    size="xs"
                    variant="ghost"
                    color="error"
                    class="flex-1"
                    @click="removeStop(index)"
                  />
                </div>
              </div>
            </template>
          </UPopover>
        </div>
      </div>
    </div>

    <UButton
      v-if="canReset"
      icon="i-lucide-undo-2"
      variant="ghost"
      color="neutral"
      size="sm"
      @click="onRestoreDefault"
    />
  </div>
</template>

<script setup lang="ts">
import type { GradientType, GradientStop } from '#interfaces/brick';
import type { PropType } from 'vue';

const steps = defineModel<GradientStop[]>("value", { default: () => [] });

const props = defineProps({
  type: {
    type: String as PropType<GradientType>,
    default: 'Linear' as GradientType
  },
  default: {
    type: Object as PropType<GradientStop[]>,
    default: []
  },
  skip_alpha: {
    type: Boolean,
    default: false
  }
});

const previewRef = ref<HTMLElement | null>(null);
const draggingStop = ref<number | null>(null);

// Calcolo se mostrare il reset
const canReset = computed(() => {
  if (!props.default || steps.value.length !== props.default.length) return true;
  return JSON.stringify(steps.value) !== JSON.stringify(props.default);
});

const gradientString = computed(() => {
  const sorted = [...steps.value].sort((a, b) => a.position - b.position);
  if (sorted.length === 0) return 'transparent';
  const stops = sorted.map(s => `${s.color} ${s.position}%`).join(', ');
  return `${props.type.toLowerCase()}-gradient(90deg, ${stops})`;
});

const checkerboardStyle = computed(() => ({
  backgroundImage: `conic-gradient(#333 90deg, transparent 90deg 180deg, #333 180deg 270deg, transparent 270deg)`,
  backgroundSize: '12px 12px'
}));

function addStopAt(event: MouseEvent) {
  if (!previewRef.value) return;
  const rect = previewRef.value.getBoundingClientRect();
  const pos = Math.round(((event.clientX - rect.left) / rect.width) * 100);
  steps.value = [...steps.value, { color: '#3b82f6', position: pos }];
}

function removeStop(index: number) {
  steps.value = steps.value.filter((_, i) => i !== index);
}

function duplicateStop(index: number) {
  const stop = steps.value[index];
  if (!stop) return;
  const newPos = Math.min(stop.position + 5, 100);
  steps.value.splice(index + 1, 0, { ...stop, position: newPos });
}

function startDrag(index: number, _event: MouseEvent) {
  draggingStop.value = index;
  window.addEventListener("mousemove", onDrag);
  window.addEventListener("mouseup", stopDrag);
}

function onDrag(event: MouseEvent) {
  if (draggingStop.value === null || !previewRef.value) return;
  
  const rect = previewRef.value.getBoundingClientRect();
  let pos = ((event.clientX - rect.left) / rect.width) * 100;
  pos = Math.min(Math.max(0, pos), 100);

  steps.value[draggingStop.value].position = parseFloat(pos.toFixed(2));
}

function stopDrag() {
  draggingStop.value = null;
  window.removeEventListener("mousemove", onDrag);
  window.removeEventListener("mouseup", stopDrag);
}

function onSetColor(index: number, color: string) {
  steps.value[index].color = color;
}

function onRestoreDefault() {
  if (props.default) steps.value = JSON.parse(JSON.stringify(props.default));
}
</script>