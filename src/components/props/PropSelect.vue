<template>
  <div class="w-full flex flex-col gap-1.5">
    <!-- Area di Selezione (Finto Input) -->
    <div 
      class="relative flex flex-wrap gap-1.5 p-2 min-h-10 rounded-md border transition-all duration-200 bg-white dark:bg-neutral-900 shadow-sm"
      :class="containerClasses"
    >
      <!-- Elementi Selezionati (UBadge) -->
      <UBadge
        v-for="(item, index) in modelValue"
        :key="index"
        size="xs"
        variant="subtle"
        color="primary"
        class="rounded-md flex items-center gap-1 animate-in fade-in zoom-in duration-150"
      >
        {{ getLabel(item) }}
        <UIcon
          name="i-lucide-x"
          class="w-3.5 h-3.5 cursor-pointer opacity-60 hover:opacity-100"
          @click.stop="removeOption(index)"
        />
      </UBadge>

      <!-- Input per filtrare o mostrare lo stato -->
      <UInput
        v-model="searchQuery"
        variant="none"
        :placeholder="modelValue.length === 0 ? 'Seleziona...' : ''"
        :disabled="isAtMax"
        class="flex-1 min-w-20"
        :ui="{ base: 'h-6 p-0 shadow-none ring-0 focus:ring-0 text-sm bg-transparent' }"
        @focus="showDropdown = true"
        @blur="handleBlur"
      />

      <!-- Dropdown delle Opzioni -->
      <div 
        v-if="showDropdown && filteredOptions.length > 0"
        class="absolute top-full left-0 z-50 w-full mt-1 overflow-hidden bg-white dark:bg-neutral-900 border border-neutral-200 dark:border-neutral-800 rounded-md shadow-lg max-h-48 overflow-y-auto"
      >
        <div
          v-for="option in filteredOptions"
          :key="option"
          class="px-3 py-2 text-sm cursor-pointer hover:bg-primary-50 dark:hover:bg-primary-900/30 transition-colors"
          @mousedown.prevent="addOption(option)"
        >
          {{ getLabel(option) }}
        </div>
      </div>

      <!-- Counter discreto se c'è un limite max -->
      <div 
        v-if="prop.max" 
        class="absolute bottom-1 right-1.5 text-[9px] font-mono pointer-events-none text-neutral-400"
      >
        {{ modelValue.length }}/{{ prop.max }}
      </div>
    </div>

    <!-- Descrizione o Errori -->
    <p v-if="inputError || prop.description" 
       class="text-[11px] px-1 transition-colors"
       :class="inputError ? 'text-error-500 font-medium' : 'text-neutral-400'">
      {{ inputError || prop.description }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import type { SelectablePropType } from '~/interfaces/brick';

const props = defineProps<{
  modelValue: any[] | null | undefined;
  prop: SelectablePropType<any>;
}>();

const emit = defineEmits(['update:modelValue']);

const searchQuery = ref('');
const showDropdown = ref(false);
const inputError = ref<string | null>(null);

const modelValue = computed({
  get: () => Array.isArray(props.modelValue) ? props.modelValue : [],
  set: (v) => emit('update:modelValue', v)
});

// --- Logica di Stato ---

const isAtMax = computed(() => props.prop.max !== undefined && modelValue.value.length >= props.prop.max);

const containerClasses = computed(() => {
  if (inputError.value) return 'border-error-500 ring-1 ring-error-500';
  return 'border-neutral-200 dark:border-neutral-800 focus-within:ring-2 focus-within:ring-primary-500/40 focus-within:border-primary-500';
});

const filteredOptions = computed(() => {
  const options = props.prop.options || [];
  // Filtriamo le opzioni già selezionate e applichiamo la ricerca testuale
  return options.filter(opt => {
    const isSelected = modelValue.value.includes(opt);
    const matchesSearch = getLabel(opt).toLowerCase().includes(searchQuery.value.toLowerCase());
    return !isSelected && matchesSearch;
  });
});

// --- Azioni ---

const getLabel = (opt: any) => String(opt);

const addOption = (option: any) => {
  inputError.value = null;

  if (isAtMax.value) {
    inputError.value = `Selezionati già ${props.prop.max} elementi.`;
    return;
  }

  modelValue.value = [...modelValue.value, option];
  searchQuery.value = '';
  // Se abbiamo raggiunto il max dopo l'aggiunta, chiudiamo
  if (isAtMax.value) showDropdown.value = false;
};

const removeOption = (index: number) => {
  inputError.value = null;
  const next = [...modelValue.value];
  next.splice(index, 1);
  modelValue.value = next;
};

const handleBlur = () => {
  // Piccolo delay per permettere il click sul dropdown
  setTimeout(() => { showDropdown.value = false; }, 150);
};
</script>