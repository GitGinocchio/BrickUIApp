<template>
  <div class="flex items-center gap-2">
    <UPopover :popper="{ placement: 'bottom-start' }">
      <template #default="{ open }">
        <UButton
          :color="buttonColor"
          :variant="buttonVariant"
          :size="buttonSize"
          :class="[
            { 'ring-2 ring-primary': open },
            'justify-between',
            ui.button
          ]"
          v-bind="$attrs"
        >
          <div class="flex flex-row">
            <div 
              class="w-4 h-4 rounded-full border border-neutral-300 dark:border-neutral-700 shrink-0 overflow-hidden bg-checkerboard"
              :class="ui.colorPreview"
            >
              <div 
                class="w-full h-full" 
                :style="{ backgroundColor: color }" 
              />
            </div>
            <span class="ml-2 font-mono text-xs" :class="ui.label">{{ color }}</span>
          </div>
          <UButton
            v-if="enable_copy"
            icon="i-lucide-copy"
            size="xs"
            variant="link"
          />
        </UButton>
      </template>

      <!-- Pannello del Picker -->
      <template #content>
        <div class="p-3 flex flex-col gap-3" :class="ui.content">
          <!-- Layout affiancato: ColorPicker + Alpha Slider Verticale -->
          <div class="flex items-center gap-3">
            <UColorPicker
              v-model="internalBaseColor"
              size="xs" 
              :format="format"
            />

            <!-- Slider Alpha Verticale -->
            <div v-if="!skip_alpha" class="flex flex-col items-center gap-1 h-full py-1">
              <span class="text-[10px] font-mono text-neutral-400">
                {{ Math.round(internalAlpha * 100) }}%
              </span>
              <USlider
                v-model="internalAlpha"
                orientation="vertical"
                :min="0"
                :max="1"
                :step="0.01"
                size="xs"
                class="h-32"
              />
            </div>
          </div>
          
          <!-- Azioni -->
          <div class="flex border-t border-neutral-200 dark:border-neutral-800 pt-2 gap-2">
            <UButton v-if="enable_saved" size="xs" variant="ghost" icon="i-lucide-save" :label="t('actions.save')" @click="saveColor" />
            <UButton v-if="enable_saved" size="xs" variant="ghost" color="error" icon="i-lucide-trash" :label="t('actions.remove')" @click="removeColor" />
            <UButton size="xs" variant="ghost" icon="i-lucide-rotate-ccw" :label="t('actions.clear')" @click="clearColor" />
          </div>

          <!-- Swatches (Preferiti) con sfondo a scacchiera -->
          <div v-if="enable_saved || enable_swatches" class="grid grid-cols-6 gap-1">
            <button
              v-if="enable_swatches"
              v-for="c in props.swatches"
              :key="c"
              class="w-5 h-5 rounded-sm border border-neutral-200 dark:border-neutral-800 overflow-hidden bg-checkerboard"
              @click="setColorFromExternal(c)"
            >
              <div class="w-full h-full" :style="{ backgroundColor: c }" />
            </button>
            <button
              v-if="enable_saved"
              v-for="c in saved"
              :key="c"
              class="w-5 h-5 rounded-sm border border-neutral-200 dark:border-neutral-800 overflow-hidden bg-checkerboard"
              @click="setColorFromExternal(c)"
            >
              <div class="w-full h-full" :style="{ backgroundColor: c }" />
            </button>
          </div>
        </div>
      </template>
    </UPopover>
  </div>
</template>

<script lang="ts" setup>
import { ref, watch, type PropType } from 'vue';
import { parseColorToRgba, formatColorOutput, type ColorFormat } from '~/utils/color';

defineOptions({
  inheritAttrs: false
});

const { t } = useI18n();

const color = defineModel<string>("color", { required: true });
const saved = defineModel<string[]>("saved", { default: () => [] });

interface UIConfig {
  button?: string;
  colorPreview?: string;
  label?: string;
  content?: string;
}

const props = defineProps({
  format: {
    type: String as PropType<ColorFormat>,
    required: false,
    default: "hex"
  },
  swatches: {
    type: Array as PropType<string[]>,
    required: false,
    default: () => []
  },
  enable_swatches: {
    type: Boolean,
    required: false,
    default: true
  },
  enable_saved: {
    type: Boolean,
    required: false,
    default: true
  },
  enable_copy: {
    type: Boolean,
    required: false,
    default: false
  },
  skip_alpha: {
    type: Boolean,
    required: false,
    default: false
  },
  default: {
    type: String,
    required: false,
    default: '#000000'
  },
  ui: {
    type: Object as PropType<UIConfig>,
    default: () => ({})
  },
  buttonColor: {
    type: Object as PropType<"neutral" | "primary" | "secondary" | "success" | "info" | "warning" | "error">,
    default: 'neutral'
  },
  buttonVariant: {
    type: Object as PropType<"link" | "subtle" | "solid" | "outline" | "soft" | "ghost">,
    default: 'subtle'
  },
  buttonSize: {
    type: Object as PropType<"md" | "xs" | "sm" | "lg" | "xl">,
    default: 'md'
  }
});

// Stato isolato per evitare feedback loop / flickering con UColorPicker
const internalBaseColor = ref('#000000');
const internalAlpha = ref(1);
let isInternalChange = false;

// Scompone una stringa colore in ingresso (HEX, RGB, HSL, ecc.)
const parseInputColor = (val: string) => {
  if (!val) return;
  const { r, g, b, alpha } = parseColorToRgba(val);
  
  // Imposta il valore base senza il canale alpha per UColorPicker
  internalBaseColor.value = formatColorOutput(r, g, b, 1, props.format, true);
  internalAlpha.value = props.skip_alpha ? 1 : alpha;
};

// Inizializza al primo mount
parseInputColor(color.value);

// Aggiorna color.value unificando base + alpha nel formato prescelto
watch([internalBaseColor, internalAlpha], () => {
  const { r, g, b } = parseColorToRgba(internalBaseColor.value);
  
  isInternalChange = true;
  color.value = formatColorOutput(r, g, b, internalAlpha.value, props.format, props.skip_alpha);
});

// Sincronizza se il modello viene modificato esternamente dal genitore
watch(color, (newVal) => {
  if (isInternalChange) {
    isInternalChange = false;
    return;
  }
  parseInputColor(newVal);
});

const setColorFromExternal = (c: string) => {
  parseInputColor(c);
};

const saveColor = () => {
  if (!saved.value.includes(color.value)) {
    saved.value = [color.value, ...saved.value];
  }
};

const clearColor = () => {
  parseInputColor(props.default);
};

const removeColor = () => {
  if (saved.value) {
    saved.value = saved.value.filter((c: string) => c !== color.value);
  }
};
</script>

<style scoped>
.bg-checkerboard {
  background-image: conic-gradient(
    #e5e5e5 90deg, 
    #ffffff 90deg 180deg, 
    #e5e5e5 180deg 270deg, 
    #ffffff 270deg
  );
  background-size: 8px 8px;
}

.dark .bg-checkerboard {
  background-image: conic-gradient(
    #262626 90deg, 
    #171717 90deg 180deg, 
    #262626 180deg 270deg, 
    #171717 270deg
  );
  background-size: 8px 8px;
}
</style>