<template>
  <div class="flex flex-col w-full space-y-6">
    <UFormField label="Description:">
      <UTextarea
        v-model="prop.description"
        placeholder="Type the prop's description (Markdown supported)"
        class="w-full"
        autoresize
        :maxlength="512"
        help="Markdown supported"
      />
    </UFormField>

    <UFormField label="Type:">
      <USelectMenu
        v-model="propTypeOption"
        class="w-full"
        :icon="(propTypeOption as any).icon"
        :items="propTypeOptions"
        @update:model-value="onNewPropTypeSelected"
      />
    </UFormField>

    <UFormField 
      v-if="['Select', 'Array'].includes(prop.prop_type)" 
      label="Value type"
    >
      <USelectMenu
        v-model="(prop as any).value_type"
        :options="valueTypeOptions"
        @update:model-value="onValueTypeChanged"
      >
        <template #item="{ item }">
          <UIcon :name="item.icon" class="w-4 h-4" />
          <span>{{ item.label }}</span>
        </template>
      </USelectMenu>
    </UFormField>

    <UFormField v-if="prop.prop_type === 'Select'" label="Options:">
      <div class="flex flex-wrap gap-2 p-2 border border-neutral-800 rounded-md">
        <UBadge
          v-for="(opt, index) in prop.options"
          :key="index"
          variant="soft"
          size="sm"
          closable
          @close="removeOption(index)"
        >
          {{ opt }}
        </UBadge>
        <UInput
          v-model="newOptionInput"
          placeholder="Add option..."
          variant="none"
          size="xs"
          class="flex-1 min-w-100px"
          @keydown.enter.prevent="addOption"
        />
      </div>
    </UFormField>

    <div class="grid grid-cols-3 gap-4">
      <UFormField v-if="shouldShowDefaultInput" label="Default value:">
        <UInput class="w-full" v-if="prop.prop_type === 'String'" v-model="prop.default" />
        <UTextarea class="w-full" v-else-if="prop.prop_type === 'Text'" v-model="prop.default" autoresize />
        <UInput
          v-else-if="['Int', 'Float'].includes(prop.prop_type)"
          v-model.number="prop.default"
          class="w-full"
          type="number"
          :step="prop.prop_type === 'Float' ? 0.1 : 1"
        />
        <USwitch v-else-if="prop.prop_type === 'Bool'" class="w-full" v-model="prop.default" />
        <div v-else-if="prop.prop_type === 'Color'" class="flex flex-1 w-full gap-2 items-center">
          <UPopover>
            <UButton label="Choose color" color="neutral" variant="outline">
              <template #leading>
                <span :style="{ backgroundColor: prop.default || '#000000' }" class="size-3 rounded-full" />
              </template>
            </UButton>

            <template #content>
              <UColorPicker v-model="prop.default" class="p-2" />
            </template>
          </UPopover>
        </div>
        <UInput 
          v-else-if="['Date', 'Datetime'].includes(prop.prop_type)" 
          v-model="prop.default" 
          class="w-full"
          :type="prop.prop_type === 'Date' ? 'date' : 'datetime-local'" 
        />
      </UFormField>
  
      <UFormField :label="minLabel" v-if="['Int', 'Float', 'Array', 'Select'].includes(prop.prop_type)">
        <UInput class="w-full" v-model.number="(prop as any).min" type="number" />
      </UFormField>
      <UFormField :label="maxLabel" v-if="['Int', 'Float', 'Array', 'Select'].includes(prop.prop_type)">
        <UInput class="w-full" v-model.number="(prop as any).max" type="number" />
      </UFormField>
    </div>

    <div v-if="['Date', 'Datetime'].includes(prop.prop_type)" class="flex gap-4">
      <UFormField label="Allow past:">
        <USwitch :model-value="(prop as any).allow_past" @update:model-value="v => toggleDateConstraint('past', v)" />
      </UFormField>
      <UFormField label="Allow future:">
        <USwitch :model-value="(prop as any).allow_future" @update:model-value="v => toggleDateConstraint('future', v)" />
      </UFormField>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type Prop, createProp, type PropTypeValue, propTypeValues } from '#interfaces/brick';
import type { SelectMenuItem } from '@nuxt/ui';

// --- Props & Emits ---
const prop = defineModel<Prop>("prop", { required: true });
const props = defineProps<{
  allProps: Prop[],
  showAlertOnTypeChange?: boolean
}>();

const newOptionInput = ref('');

// --- Mapping Icone ---
const iconsMap: Record<PropTypeValue, string> = {
  Select: 'i-lucide-list-todo',
  Array: 'i-lucide-list',
  String: 'i-lucide-type',
  Text: 'i-lucide-text',
  Int: 'i-lucide-arrow-up-1-0',
  Float: 'i-lucide-decimals-arrow-right',
  Bool: 'i-lucide-toggle-left',
  Color: 'i-lucide-paint-bucket',
  Gradient: 'i-lucide-swatch-book',
  Datetime: 'i-lucide-calendar-clock',
  Date: 'i-lucide-calendar-1',
  Time: 'i-lucide-clock',
  Null: 'i-lucide-circle-slash',
  Deprecated: 'i-lucide-triangle-alert',
  Unknown: 'i-lucide-circle-question-mark'
};

// --- Computed ---
function asOption(v: PropTypeValue): SelectMenuItem {
  return {
    label: v.replace(/([A-Z])/g, " $1").trim(),
    type: 'item',
    value: v,
    icon: iconsMap[v] || 'i-lucide-circle'
  }
}
const propTypeOption = computed(() => asOption(prop.value.prop_type));
const propTypeOptions: SelectMenuItem[] = propTypeValues
  .filter((v) => !['Deprecated', 'Unknown'].includes(v))
  .map((v) => asOption(v));

const valueTypeOptions = [
  { label: 'String', value: 'String', icon: 'i-lucide-type' },
  { label: 'Float', value: 'Float', icon: 'i-lucide-decimals-arrow-right' },
  { label: 'Integer', value: 'Integer', icon: 'i-lucide-arrow-up-1-0' },
];

const minLabel = computed(() => 
  ['Int', 'Float'].includes(prop.value.prop_type) ? 'Min value:' : 'Min items:'
);

const maxLabel = computed(() => 
  ['Int', 'Float'].includes(prop.value.prop_type) ? 'Max value:' : 'Max items:'
);

const shouldShowDefaultInput = computed(() => prop.value.prop_type !== 'Null');

// --- Methods ---

function onNewPropTypeSelected(newPropType: { label: string, value: PropTypeValue, icon: string }) {
  if (prop.value.prop_type === newPropType.value) return;

  const update = () => {
    const current = prop.value;
    const subType = (newPropType.value === 'Select' || newPropType.value === 'Array') ? 'String' : null;
    prop.value = createProp(newPropType.value, current.prop_name, current.description, subType);
  };

  if (props.showAlertOnTypeChange === false) {
    update();
    return;
  }

  // TODO: Sostituire con UModal se preferisci una UI coerente
  if (confirm(`Are you sure you want to change "${prop.value.prop_name}" to ${newPropType.value}? All settings for this prop will be lost.`)) {
    update();
  }
}

function onValueTypeChanged() {
  if (prop.value.prop_type === 'Select') prop.value.options = [];
  else if (prop.value.prop_type === 'Array') prop.value.default = [];
}

// Gestione Opzioni (per Select)
function addOption() {
  const val = newOptionInput.value.trim();
  // Aggiunto il controllo: se non è Select, non fare nulla
  if (!val || prop.value.prop_type !== 'Select') return;

  if (!prop.value.options) prop.value.options = [];
  
if (prop.value.value_type === 'String') {
    // @ts-ignore
    if (!prop.value.options.includes(val)) {
      // @ts-ignore
      prop.value.options.push(val);
    }
  } 
  else {
    const parsed = prop.value.value_type === 'Integer' 
      ? parseInt(val, 10) 
      : parseFloat(val.replace(',', '.'));

    if (!isNaN(parsed)) {
      // @ts-ignore
      if (!prop.value.options.includes(parsed)) {
        // @ts-ignore
        prop.value.options.push(parsed);
      }
    }
  }

  newOptionInput.value = '';
}

function removeOption(index: number) {
  if (prop.value.prop_type != 'Select') return;
  prop.value.options.splice(index, 1);
}

// Vincoli Date
function toggleDateConstraint(type: 'past' | 'future', value: boolean) {
  if (prop.value.prop_type !== 'Date' && prop.value.prop_type !== 'Datetime') return;
  if (type === 'past') {
    prop.value.allow_past = value;
    if (!value && !prop.value.allow_future) prop.value.allow_future = true;
  } else {
    prop.value.allow_future = value;
    if (!value && !prop.value.allow_past) prop.value.allow_past = true;
  }
}
</script>

<style scoped>
/* Layout Grid per simulare l'allineamento di NaiveUI */
.grid-form-item {
  display: grid;
  grid-template-columns: 12rem 1fr;
  align-items: center;
  gap: 1.5rem;
}

@media (max-width: 768px) {
  .grid-form-item {
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }
}

/* Stili specifici per rendere il selettore colore più simile a un input standard */
input[type="color"]::-webkit-color-swatch-wrapper {
  padding: 0;
}
input[type="color"]::-webkit-color-swatch {
  border: none;
  border-radius: 4px;
}
</style>