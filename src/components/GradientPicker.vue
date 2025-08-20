<template>
  <div class="gradient-editor">
    <!-- Anteprima del gradiente -->
    <div 
      class="gradient-preview" 
      :style="{ background: gradientString }"
      @dblclick.prevent="addStopAt($event)"
    >
      <!-- Stop handles -->
      <n-color-picker
        v-for="(stop, index) in value"
        :key="index"
        size="small"
        class="stop-handle"
        :style="{ left: stop.position + '%', background: stop.color }"
        :value="stop.color"
        @update:value="(color) => onSetColor(index, color)"
        @mousedown.prevent="startDrag(index, $event)"
      >
      <template #label>
        <!-- Vuoto per togliere la label default -->
      </template>
      <template #action>
        <n-button size="small" @click="removeStop(index)">Remove</n-button>
      </template>
      </n-color-picker>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { NColorPicker, NButton } from 'naive-ui'

interface Stop {
  color: string
  position: number
}

// Model reattivo, usa sempre v-model:value nel parent
const steps = defineModel<Stop[]>("value", { 
  default: [{ color: '#ffffff00', position: 50 }] 
})

const draggingStop = ref<number | null>(null)
const wasDragging = ref(false)

// Gradiente CSS
const gradientString = computed(() => {
  const sorted = steps.value.slice().sort((a, b) => a.position - b.position)
  return `linear-gradient(90deg, ${sorted.map(s => `${s.color} ${s.position}%`).join(', ')})`
})

// Aggiungi uno stop al doppio click
function addStopAt(event: MouseEvent) {
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  const pos = ((event.clientX - rect.left) / rect.width) * 100
  steps.value = [...steps.value, { color: '#ffffff00', position: Math.round(pos) }]
}

// Rimuovi uno stop
function removeStop(index: number) {
  let newStops = steps.value.slice()
  newStops.splice(index, 1)
  if (newStops.length === 0) {
    newStops = [{ color: '#ffffff', position: 50 }]
  }
  steps.value = newStops
}

const dragOffset = ref(0) // nuova variabile globale per il drag

function startDrag(index: number, event: MouseEvent) {
  draggingStop.value = index
  wasDragging.value = false

  const stopHandle = document.querySelectorAll(".stop-handle")[index] as HTMLElement
  if (stopHandle) {
    // salva l'offset tra il cursore e il centro dello stop
    const rect = stopHandle.getBoundingClientRect()
    dragOffset.value = event.clientX - (rect.left + rect.width / 2)
  }

  document.addEventListener("mousemove", onDrag)
  document.addEventListener("mouseup", stopDrag)
}

function onDrag(event: MouseEvent) {
  if (draggingStop.value === null) return
  wasDragging.value = true

  const preview = document.querySelector(".gradient-preview") as HTMLElement
  if (!preview) return

  const width = preview.offsetWidth
  const left = preview.getBoundingClientRect().left

  // posizione X relativa al centro dello stop, correggendo con l'offset
  let relativeX = event.clientX - left - dragOffset.value

  // limita tra 0 e la larghezza del div
  relativeX = Math.min(Math.max(0, relativeX), width)

  // calcola la percentuale
  const pos = (relativeX / width) * 100

  const newStops = steps.value.slice()
  newStops[draggingStop.value] = { ...newStops[draggingStop.value], position: pos }
  steps.value = newStops
}

function onSetColor(index: number, color: string) {
  const newStops = steps.value.slice()
  newStops[index] = { ...newStops[index], color }
  steps.value = newStops
}

function stopDrag() {
  draggingStop.value = null
  dragOffset.value = 0
  document.body.style.userSelect = '' // Riabilita selezione testo
  document.removeEventListener("mousemove", onDrag)
  document.removeEventListener("mouseup", stopDrag)
}
</script>



<style scoped>
.gradient-editor {
  width: 100%;
  height: 34px;
  border: 1px solid rgb(224, 224, 230);
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
</style>
