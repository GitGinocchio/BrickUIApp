<template>
  <div class="gradient-editor">
    <!-- Anteprima del gradiente -->
    <div 
      class="gradient-preview"
      ref="previewRef"
      :style="{ background: gradientString }"
      @dblclick.prevent="addStopAt($event)"
    >
      <!-- Stop handles -->
      <n-color-picker
        v-for="(stop, index) in value"
        :key="index"
        size="large"
        class="stop-handle"
        :style="{ left: stop.position + '%', background: stop.color }"
        :show-alpha="!skip_alpha"
        :value="stop.color"
        @update:value="(color) => onSetColor(index, color)"
        @mousedown.prevent="startDrag(index, $event)"
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
</template>

<script setup lang="ts">
import { NColorPicker, NButton, NInputNumber } from 'naive-ui'
import { colorStringToRGBA } from '#utils/color';
import { GradientType } from '#interfaces/brick';

interface Stop {
  color: string
  position: number
}

const props = defineProps({
  skip_alpha: {
    type: Boolean,
    value: false
  }
});

// Model reattivo, usa sempre v-model:value nel parent
const steps = defineModel<Stop[]>("value", { 
  default: [{ color: '#00000000', position: 50 }] 
})

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
  const sorted = steps.value.slice().sort((a, b) => a.position - b.position)
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
    newStops = [{ color: props.skip_alpha ? '#000000FF' : '#00000000', position: 50 }]
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
</script>



<style scoped>


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
  flex-grow: 1;
}

.gradient-preview::before {
  content: "";
  position: absolute; 
  inset: 0;
  background-image: 
    linear-gradient(45deg, #f5f5f5 25%, transparent 25%), 
    linear-gradient(-45deg, #f5f5f5 25%, transparent 25%),
    linear-gradient(45deg, transparent 75%, #f5f5f5 75%), 
    linear-gradient(-45deg, transparent 75%, #f5f5f5 75%);
  background-size: 17px 17px;
  background-position: 8.5px 8.5px, 8.5px 0, 0 0, 0 8.5px;
  z-index: 0;
}

.gradient-preview::after {
  content: "";
  position: absolute;
  inset: 0;
  background: v-bind('gradientString'); /* Vue 3 <style scoped> binding */
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

.stop-handle {
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
