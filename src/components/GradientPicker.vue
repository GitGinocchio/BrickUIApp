<template>
  <div class="gradient-picker">
    <div class="gradient-editor">
      <!-- Anteprima del gradiente -->
      <div 
        class="gradient-preview"
        ref="previewRef"
        :style="{ background: gradientString }"
        @dblclick.prevent="addStopAt($event)"
      >
        <div 
          v-for="(stop, index) in value" 
          :key="index"
          class="stop-handle"
          :style="{ left: stop.position + '%', position: 'absolute' }"
          @mousedown.prevent="startDrag(index, $event)"
        >
          <!-- Stop handles -->
          <n-color-picker
            size="large"
            :show-alpha="!skip_alpha"
            :value="stop.color"
            @update:value="(color) => onSetColor(index, color)"
          >
            <template #label>
              <!-- Vuoto per togliere la label default -->
            </template>
            <template #action>
              <div class="actions"> 
                <!--
                <n-select
                  size="small"
                  :options="gradientTypeOptions"
                />
                -->
                <n-input-number 
                  size="small" 
                  :precision="2"
                  :default-value="0.0"
                  placeholder="Stop color position"
                  :step="0.5"
                  :max="100"
                  :min="0"
                  :value="parseFloat(stop.position.toFixed(2))"
                  @update:value="(position) => onSetPosition(index, position ?? 0)"
                />
                <div class="bottom">
                  <n-button size="small" @click="duplicateStop(index)">Clone</n-button>
                  <n-button size="small" @click="removeStop(index)">Remove</n-button>
                </div>   
              </div>
            </template>
          </n-color-picker>
        </div>
      </div>
    </div>
    <div>
      <n-button v-if="canReset" class="restore-button" @click="onRestoreDefault">
        <template #icon>
          <Undo2 :size="16" />
        </template>
      </n-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Undo2 } from 'lucide-vue-next';
import { NColorPicker, NButton, NInputNumber, NTooltip } from 'naive-ui'
import { colorStringToRGBA } from '#utils/color';
import { GradientType } from '#interfaces/brick';

interface Stop {
  color: string
  position: number
}

const props = defineProps({
  color: {
    type: Object as PropType<String>,
    default: "grey"
  },
  default: {
    type: Object as PropType<Stop[]>,
    default: [],
    required: false
  },
  skip_alpha: {
    type: Boolean,
    value: false
  }
});

// Model reattivo, usa sempre v-model:value nel parent
const steps = defineModel<Stop[]>("value", { 
  default: []
})

const canReset = computed(() => {
  if (steps.value.length !== props.default?.length) return true;

  const serialize = (arr: any[]) => arr.map(obj => JSON.stringify(obj)).sort();

  const sortedA = serialize(steps.value);
  const sortedB = serialize(props.default);

  return sortedA.some((val, index) => val !== sortedB[index]);
});

/*
const gradientTypeOptions = [
  { label: "Linear", value: GradientType.LINEAR },
  { label: "Radial", value: GradientType.RADIAL },
  { label: "Conic", value: GradientType.CONIC }
];
*/
const gradientType = defineModel<GradientType>("type", { default: GradientType.LINEAR });
//const showGradientTypeSelect = ref<boolean>(false);

const previewRef = ref<HTMLElement | null>(null)
const draggingStop = ref<number | null>(null)

// Gradiente CSS
const gradientString = computed(() => {
  const sorted = steps.value?.slice().sort((a, b) => a.position - b.position) ?? []
  return `${gradientType.value.toLowerCase()}-gradient(90deg, ${sorted.map(s => `${s.color} ${s.position}%`).join(', ')})`
})

// Aggiungi uno stop al doppio click
function addStopAt(event: MouseEvent) {
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  const pos = ((event.clientX - rect.left) / rect.width) * 100
  steps.value = [...steps.value, { color: props.skip_alpha ? '#000000FF' : '#00000000', position: Math.round(pos) }]
}

// Rimuovi uno stop
function removeStop(index: number) {
  let newStops = steps.value.slice()
  newStops.splice(index, 1)
  if (newStops.length === 0) {
    newStops = []
  }
  steps.value = newStops
}

function duplicateStop(index: number) {
  const stopToDuplicate = steps.value[index]
  if (!stopToDuplicate) return

  const pos = stopToDuplicate.position + 3 > 100 ? stopToDuplicate.position - 3 : stopToDuplicate.position + 3;

  // Copia lo stop e sposta leggermente la posizione per renderlo visibile
  const newStop = {
    color: stopToDuplicate.color,
    position: Math.min(pos, 100) // evita oltre 100%
  }

  // Inserisci subito dopo lo stop originale
  const newStops = steps.value.slice()
  newStops.splice(index + 1, 0, newStop)
  steps.value = newStops
}

function onDrag(event: MouseEvent) {
  if (draggingStop.value === null || !previewRef.value) return

  const width = previewRef.value.offsetWidth
  const rect = previewRef.value.getBoundingClientRect()
  
  let relativeX = event.clientX - rect.left
  relativeX = Math.min(Math.max(0, relativeX), width)

  const pos = (relativeX / width) * 100

  const newStops = steps.value.slice()
  newStops[draggingStop.value] = { ...newStops[draggingStop.value], position: pos }
  steps.value = newStops
}

function onSetColor(index: number, color: string) {
  const newStops = steps.value.slice()
  newStops[index] = { ...newStops[index], color: colorStringToRGBA(color) }
  steps.value = newStops
}

function onSetPosition(index: number, position: number) {
  const newStops = steps.value.slice()
  newStops[index] = { ...newStops[index], position }
  steps.value = newStops
}

function startDrag(index: number, _event: MouseEvent) {
  draggingStop.value = index
  document.addEventListener("mousemove", onDrag)
  document.addEventListener("mouseup", stopDrag)
}

function stopDrag() {
  draggingStop.value = null
  document.removeEventListener("mousemove", onDrag)
  document.removeEventListener("mouseup", stopDrag)
}

function onRestoreDefault() {
  steps.value = props.default;
}
</script>



<style scoped>
.gradient-picker {
  display: flex;
  flex-direction: row;
}

.restore-button {
  width: 1rem;
}

.gradient-editor {
  width: 100%;
  height: 34px;
  
  border: 1px solid rgba(255, 255, 255, 0.24);
  border-radius: 3px;
  box-sizing: border-box;
  padding: 3px;
  display: flex;
}

.gradient-preview {
  position: relative;
  display: flex;
  align-items: center;
  flex-grow: 1;
  
}

.gradient-preview::before {
  content: "";
  position: absolute; 
  inset: 0;
  background-image: conic-gradient(
    v-bind('props.color') 90deg, 
    transparent 90deg 180deg, 
    v-bind('props.color') 180deg 270deg, 
    transparent 270deg
  );
  background-size: 18px 18px;
  z-index: 0;
}

.gradient-preview::after {
  content: "";
  position: absolute;
  inset: 0;
  background: v-bind('gradientString');
  z-index: 1;
}

.gradient-preview {
  position: relative;
  overflow: hidden; /* così non sborda */
}

.gradient-preview > * {
  position: relative;
  z-index: 2; /* stop handle sopra al gradiente */
}

.stop-handle :deep(.n-color-picker) {
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 16px;
  height: 16px;
  border-radius: 50%;
  border: 2px solid #fff;
  cursor: pointer;
  box-shadow: 0 0 2px rgba(0, 0, 0, 0.5);
}

.actions {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.actions .bottom {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  gap: 0.25rem;
}

.actions .bottom button {
  margin: 0;
}

::v-deep(n-color-picker-action) {
  justify-content: flex-start;
}

</style>
