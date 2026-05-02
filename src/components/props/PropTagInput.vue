<template>
  <div class="w-full flex flex-col gap-1.5">
    <!-- Container principale che emula l'input field -->
    <div 
      class="relative flex flex-wrap gap-1.5 p-2 min-h-10 rounded-md border transition-all duration-200 bg-white dark:bg-neutral-900 shadow-sm"
      :class="containerClasses"
    >
      <!-- Tag renderizzati tramite UBadge -->
      <UBadge
        v-for="(tag, index) in modelValue"
        :key="index"
        size="xs"
        variant="subtle"
        :color="inputError ? 'error' : 'primary'"
        class="rounded-md flex items-center gap-1 group animate-in fade-in zoom-in duration-150"
      >
        {{ tag }}
        <UIcon
          name="i-lucide-x"
          class="w-3.5 h-3.5 cursor-pointer opacity-60 group-hover:opacity-100 transition-opacity"
          @click.stop="removeTag(index)"
        />
      </UBadge>

      <!-- Input testuale per l'inserimento -->
      <UInput
        v-model="newTag"
        variant="none"
        :placeholder="dynamicPlaceholder"
        :disabled="isAtMax"
        class="flex-1 min-w-20"
        :ui="{ 
          base: 'h-6 p-0 shadow-none ring-0 focus:ring-0 text-sm bg-transparent',
          root: 'inline-flex flex-1'
        }"
        @keydown.enter.prevent="addTag"
        @keydown.backspace="handleBackspace"
      />

      <!-- Counter discreto in basso a destra (opzionale, integrato nel bordo) -->
      <div 
        v-if="prop.max" 
        class="absolute bottom-1 right-1.5 text-[9px] font-mono pointer-events-none"
        :class="isAtMax ? 'text-error-500 font-bold' : 'text-neutral-400'"
      >
        {{ modelValue.length }}/{{ prop.max }}
      </div>
    </div>

    <!-- Messaggio di errore o descrizione (sempre sotto) -->
    <p v-if="inputError || prop.description" 
       class="text-[11px] px-1 transition-colors"
       :class="inputError ? 'text-error-500 font-medium' : 'text-neutral-400'">
      {{ inputError || prop.description }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import type { ArrayPropType } from '~/interfaces/brick';

const props = defineProps<{
  modelValue: any[] | null | undefined;
  prop: ArrayPropType<any>;
}>();

const emit = defineEmits(['update:modelValue']);

const newTag = ref('');
const inputError = ref<string | null>(null);

// Gestione reattiva dell'array di BrickUI
const modelValue = computed({
  get: () => Array.isArray(props.modelValue) ? props.modelValue : [],
  set: (v) => emit('update:modelValue', v)
});

// Helper per le validazioni dell'interfaccia ArrayPropType
const isAtMax = computed(() => props.prop.max !== undefined && modelValue.value.length >= props.prop.max);

const dynamicPlaceholder = computed(() => {
  if (isAtMax.value) return 'Limite raggiunto';
  if (modelValue.value.length === 0) return 'Aggiungi...';
  return '';
});

// Stili dinamici del contenitore
const containerClasses = computed(() => {
  if (inputError.value) return 'border-error-500 ring-1 ring-error-500';
  if (isAtMax.value) return 'border-neutral-300 dark:border-neutral-700 bg-neutral-50 dark:bg-neutral-800/50 cursor-not-allowed';
  return 'border-neutral-200 dark:border-neutral-800 focus-within:ring-2 focus-within:ring-primary-500/40 focus-within:border-primary-500';
});

/**
 * Logica di aggiunta con validazione completa dell'interfaccia ArrayPropType
 */
const addTag = () => {
  inputError.value = null;
  const raw = newTag.value.trim();
  if (!raw) return;

  // 1. Controllo max items
  if (isAtMax.value) {
    inputError.value = `Massimo ${props.prop.max} elementi.`;
    return;
  }

  // 2. Controllo tipo e range (min_value / max_value)
  let val: any = raw;
  const isNumeric = props.prop.value_type === 'Integer' || props.prop.value_type === 'Float' || typeof props.prop.min_value === 'number';

  if (isNumeric) {
    val = Number(raw);
    if (isNaN(val)) {
      inputError.value = "Inserisci un numero.";
      return;
    }
    if (props.prop.min_value !== undefined && val < props.prop.min_value) {
      inputError.value = `Valore minimo: ${props.prop.min_value}`;
      return;
    }
    if (props.prop.max_value !== undefined && val > props.prop.max_value) {
      inputError.value = `Valore massimo: ${props.prop.max_value}`;
      return;
    }
  }

  // 3. Controllo duplicati
  if (modelValue.value.includes(val)) {
    inputError.value = "Già presente.";
    return;
  }

  modelValue.value = [...modelValue.value, val];
  newTag.value = '';
};

const removeTag = (index: number) => {
  inputError.value = null;
  const next = [...modelValue.value];
  next.splice(index, 1);
  modelValue.value = next;
};

// UX: Backspace per cancellare l'ultimo tag
const handleBackspace = () => {
  if (newTag.value === '' && modelValue.value.length > 0) {
    removeTag(modelValue.value.length - 1);
  }
};
</script>