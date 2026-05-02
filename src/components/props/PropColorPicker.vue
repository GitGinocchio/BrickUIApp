<template>
  <div class="flex items-center gap-2">
    <UPopover :popper="{ placement: 'bottom-start' }">
      <!-- Trigger: un bottone che mostra il colore attuale -->
      <template #default="{ open }">
        <UButton
          color="neutral"
          variant="subtle"
          :class="{ 'ring-2 ring-primary-500': open }"
        >
          <div 
            class="w-4 h-4 rounded-full border border-neutral-300 dark:border-neutral-700" 
            :style="{ backgroundColor: modelValue }"
          />
          <span class="ml-2 font-mono text-xs">{{ modelValue }}</span>
        </UButton>
      </template>

      <!-- Pannello del Picker -->
      <template #content>
        <div class="p-3 flex flex-col gap-3">
          <UColorPicker v-model="modelValue" :alpha="!prop.skip_alpha" />
          
          <!-- Azioni che avevi in NaiveUI -->
          <div class="flex border-t border-neutral-200 dark:border-neutral-800 pt-2 gap-2">
            <UButton size="xs" variant="ghost" icon="i-lucide-save" @click="saveColor">Save</UButton>
            <UButton size="xs" variant="ghost" icon="i-lucide-rotate-ccw" @click="clearColor">Reset</UButton>
            <UButton size="xs" variant="ghost" color="error" icon="i-lucide-trash" @click="removeColor">Remove</UButton>
          </div>

          <!-- Swatches (Preferiti) -->
          <div v-if="allSwatches.length" class="grid grid-cols-6 gap-1">
            <button
              v-for="c in allSwatches"
              :key="c"
              class="w-5 h-5 rounded-sm border border-neutral-200"
              :style="{ backgroundColor: c }"
              @click="modelValue = c"
            />
          </div>
        </div>
      </template>
    </UPopover>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{ prop: any, modelValue: string }>();
const emit = defineEmits(['update:modelValue']);

const modelValue = computed({
  get: () => props.modelValue,
  set: (v) => emit('update:modelValue', v)
});

const allSwatches = computed(() => [...(props.prop.saved || []), ...(props.prop.swatches || [])]);

const saveColor = () => {
  const saved = props.prop.saved || [];
  if (!saved.includes(modelValue.value)) {
    props.prop.saved = [modelValue.value, ...saved];
  }
};

const clearColor = () => modelValue.value = props.prop.default || '#000000';

const removeColor = () => {
  if (props.prop.saved) {
    props.prop.saved = props.prop.saved.filter((c: string) => c !== modelValue.value);
  }
};
</script>