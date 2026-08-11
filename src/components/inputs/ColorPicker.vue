<template>
  <div class="flex items-center gap-2">
    <UPopover :popper="{ placement: 'bottom-start' }">
      <template #default="{ open }">
        <UButton
          color="neutral"
          variant="subtle"
          :class="{ 'ring-2 ring-primary': open }"
        >
          <div 
            class="w-4 h-4 rounded-full border border-neutral-300 dark:border-neutral-700" 
            :style="{ backgroundColor: color }"
          />
          <span class="ml-2 font-mono text-xs">{{ color }}</span>
        </UButton>
      </template>

      <!-- Pannello del Picker -->
      <template #content>
        <div class="p-3 flex flex-col gap-3">
          <div class="flex flex-col items-center">
            <UColorPicker
              v-model="color" 
              size="xs" 
              :alpha="!skip_alpha"
              :format="format"
            />
          </div>
          
          <!-- Azioni -->
          <div class="flex border-t border-neutral-200 dark:border-neutral-800 pt-2 gap-2">
            <UButton v-if="enable_saved" size="xs" variant="ghost" icon="i-lucide-save" :label="t('actions.save')" @click="saveColor" />
            <UButton v-if="enable_saved" size="xs" variant="ghost" color="error" icon="i-lucide-trash" :label="t('actions.remove')" @click="removeColor" />
            <UButton size="xs" variant="ghost" icon="i-lucide-rotate-ccw" :label="t('actions.clear')" @click="clearColor" />
          </div>

          <!-- Swatches (Preferiti) -->
          <div v-if="enable_saved || enable_swatches" class="grid grid-cols-6 gap-1">
            <button
              v-if="enable_swatches"
              v-for="c in props.swatches"
              :key="c"
              class="w-5 h-5 rounded-sm border border-neutral-200"
              :style="{ backgroundColor: c }"
              @click="color = c"
            />
            <button
              v-if="enable_saved"
              v-for="c in saved"
              :key="c"
              class="w-5 h-5 rounded-sm border border-neutral-200"
              :style="{ backgroundColor: c }"
              @click="color = c"
            />
          </div>
        </div>
      </template>
    </UPopover>
  </div>
</template>

<script lang="ts" setup>
import type { PropType } from 'vue';

const { t } = useI18n();

const color = defineModel<string>("color", { required: true });
const saved = defineModel<string[]>("saved", { default: () => [] });

type Format = "hex" | "rgb" | "hsl" | "cmyk" | "lab";

const props = defineProps({
  format: {
    type: String as PropType<Format>,
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
  skip_alpha: {
    type: Boolean,
    required: false,
    default: false
  },
  default: {
    type: String,
    required: false,
    default: '#000000'
  }
});

const saveColor = () => {
  if (!saved.value.includes(color.value)) {
    saved.value = [color.value, ...saved.value];
  }
};

const clearColor = () => {
  color.value = props.default;
};

const removeColor = () => {
  if (saved.value) {
    saved.value = saved.value.filter((c: string) => c !== color.value);
  }
};
</script>