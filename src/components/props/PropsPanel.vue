<template>
  <div class="brick-props-manager">
    <!-- Header: Layout e Toggle Edit -->
    <div class="props-header flex items-center justify-between mb-4 px-1">
      <div class="display-modes">
        <UFormField size="sm" class="flex flex-col gap-2">
          <UButton
            :variant="layout === 'adaptive' ? 'solid' : 'ghost'"
            color="primary"
            icon="i-lucide-layout-dashboard"
            @click="layout = 'adaptive'"
          />
          <UButton
            :variant="layout === 'list' ? 'solid' : 'ghost'"
            color="primary"
            icon="i-lucide-layout-list"
            @click="layout = 'list'"
          />
          <UButton
            :variant="layout === 'grid' ? 'solid' : 'ghost'"
            color="primary"
            icon="i-lucide-layout-grid"
            @click="layout = 'grid'"
          />
        </UFormField>
      </div>

      <div class="props-actions flex items-center gap-2">
        <UButton
          v-if="editMode"
          icon="i-lucide-plus"
          size="sm"
          color="neutral"
          variant="ghost"
          @click="onNewProp"
        />
        <USwitch
          v-model="editMode"
          size="lg"
          @update:model-value="onToggleEditMode"
        >
          <template>
            <UIcon :name="editMode ? 'i-lucide-pencil' : 'i-lucide-eye'" class="h-4 w-4" />
          </template>
        </USwitch>
      </div>
    </div>

    <!-- New Prop Template (Floating) -->
    <div v-if="newProp" class="prop-item p-4 border border-primary-500/50 rounded-lg mb-4 bg-primary-500/5">
      <div class="flex items-center gap-2 mb-4">
        <UInput
          v-model="newProp.prop_name"
          placeholder="Prop name..."
          class="flex-1"
          @keydown.enter="onSavePropName(newProp)"
        />
        <UButton icon="i-lucide-save" color="primary" @click="onSavePropName(newProp)" />
        <UButton icon="i-lucide-trash-2" color="error" variant="ghost" @click="onRemoveProp(newProp)" />
      </div>
      <PropEditWrapper :prop="newProp" :all-props="brick.props" />
    </div>

    <!-- Props List with Sortable -->
    <sortable
      v-if="brick.props.length > 0"
      :options="sortableOptions"
      :list="brick.props"
      item-key="prop_name"
      tag="div"
      :class="['props-container', layout === 'adaptive' ? 'adaptive-grid' : layout]"
      @update="onPropsOrderUpdate"
    >
      <!-- @vue-ignore -->
      <template #item="{ element, index }">
        <div 
          class="prop-row group relative py-2"
          :class="{ 'cursor-grab active:cursor-grabbing': editMode }"
        >
          <UAccordion
            :items="[{ slot: 'content', label: element.prop_name, disabled: editMode ? false : element.description == null  }]"
            :unmount-on-hide="false"
            :ui="{ trailingIcon: 'hidden', label: 'hidden', trigger: 'py-0 cursor-auto' }"
            multiple
          >
            <!-- @vue-ignore -->
            <template #leading="{ item, open }">
              <div class="flex items-center w-full py-2 px-2 gap-1">
                <UIcon :name="open ? 'i-lucide-chevron-up' : 'i-lucide-chevron-down'" v-if="editMode" />
                <UIcon :name="open ? 'i-lucide-circle-x' : 'i-lucide-circle-question-mark'" v-else-if="!editMode && element.description" />
                <div class="flex items-center gap-2">
                  <div v-if="editingPropNames.has(element.prop_name)" @click.stop>
                    <UInput
                      :model-value="editingPropNames.get(element.prop_name)"
                      @update:model-value="(val) => editingPropNames.set(element.prop_name, val)"
                      @keydown.enter="onSavePropName(element)"
                      size="xs"
                    />
                  </div>
                  <span v-else class="font-semibold text-sm">{{ element.prop_name }}</span>
                </div>
              </div>
            </template>

            <template #trailing>
              <div class="flex items-center gap-1">
                <div class="hidden group-hover:flex items-center gap-1 mr-2" v-if="editMode">
                  <UButton
                    v-if="editingPropNames.has(element.prop_name)"
                    icon="i-lucide-circle-x"
                    color="neutral"
                    variant="ghost"
                    size="xs"
                    @click.stop="togglePropNameEditMode(element)"
                  />
                  <UButton
                    :icon="editingPropNames.has(element.prop_name) ? 'i-lucide-save' : 'i-lucide-pencil'"
                    color="neutral"
                    variant="ghost"
                    size="xs"
                    @click.stop="editingPropNames.has(element.prop_name) ? onSavePropName(element) : togglePropNameEditMode(element)"
                  />
                  <UButton icon="i-lucide-copy" color="neutral" variant="ghost" size="xs" @click.stop="onDuplicateProp(element)" />
                  <UButton icon="i-lucide-trash-2" color="error" variant="ghost" size="xs" @click.stop="onRemoveProp(element)" />
                </div>
              </div>
            </template>

            <template #content>
              <div class="pl-4 border-dashed ml-3.5 mb-4">
                <PropEditWrapper
                  v-if="editMode"
                  v-model:prop="brick.props[index]"
                  :all-props="brick.props"
                />
                <p v-else-if="element.description" class="text-xs text-gray-500 italic">
                  {{ element.description }}
                </p>
              </div>
            </template>
          </UAccordion>

          <div class="prop-value-wrapper mt-1 px-2">
            <PropViewWrapper v-model:prop="brick.props[index]" :brick_name="brick.name" />
          </div>
        </div>
      </template>
    </sortable>

    <div v-else class="text-center py-10 opacity-50">
      <p>This brick has no props!</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Brick, Prop } from '~/interfaces/brick';
import { Sortable } from 'sortablejs-vue3'
import PropViewWrapper from '../props/PropViewWrapper.vue';
import PropEditWrapper from '../props/PropEditWrapper.vue';

const { saveBrick } = useBrickActions();

// Props & Model
const brick = defineModel<Brick>("brick", { required: true });

// State
const newProp = ref<Prop | null>(null);
const editMode = ref(false);
const editingPropNames = ref<Map<string, string>>(new Map());
const layout = ref<'grid' | 'list' | 'adaptive'>('adaptive');

// Toast e Dialog (Nuxt UI)
const toast = useToast();
// Per il dialog di conferma, Nuxt UI usa un pattern diverso (UModal o componente dedicato)
// Qui assumiamo di usare un'interfaccia di conferma custom o UModal

// Sortable Config
const sortableOptions = computed(() => ({
  animation: 150,
  handle: editMode.value ? '.prop-row' : undefined,
  ghostClass: 'opacity-50',
  dragClass: 'bg-primary-50/10',
  filter: 'input, button, .prop-value-wrapper', // Evita drag su controlli
  preventOnFilter: false
}));

// Handlers
function onPropsOrderUpdate(event: any) {
  const { oldIndex, newIndex } = event;
  if (oldIndex !== newIndex) {
    const newList = [...brick.value.props];
    const [movedItem] = newList.splice(oldIndex, 1);
    newList.splice(newIndex, 0, movedItem);
    brick.value.props = newList;
    saveBrick(brick.value);
  }
}

function onToggleEditMode() {
  editingPropNames.value.clear();
  newProp.value = null;
}

function togglePropNameEditMode(prop: Prop) {
  if (editingPropNames.value.has(prop.prop_name)) {
    editingPropNames.value.delete(prop.prop_name);
  } else {
    editingPropNames.value.set(prop.prop_name, prop.prop_name);
  }
}

function onSavePropName(prop: Prop) {
  const isCreating = prop === newProp.value;
  const newName = isCreating 
    ? newProp.value?.prop_name.trim() 
    : editingPropNames.value.get(prop.prop_name)?.trim();

  if (!newName || !/^[a-zA-Z]+$/.test(newName)) {
    toast.add({ title: 'Error', description: 'Invalid name (letters only)', color: 'error' });
    return;
  }

  if (brick.value.props.some(p => p.prop_name === newName && p !== prop)) {
    toast.add({ title: 'Error', description: 'Name already taken', color: 'error' });
    return;
  }

  if (isCreating) {
    brick.value.props = [newProp.value!, ...brick.value.props];
    newProp.value = null;
  } else {
    prop.prop_name = newName;
    editingPropNames.value.delete(newName);
  }
  saveBrick(brick.value);
}

const onNewProp = () => {
  newProp.value = { prop_type: 'Null', prop_name: '' };
};

function onDuplicateProp(prop: Prop) {
  const copyName = `${prop.prop_name}Copy`;
  if (brick.value.props.some(p => p.prop_name === copyName)) {
    toast.add({ title: 'Error', description: 'Copy already exists', color: 'error' });
    return;
  }
  const index = brick.value.props.findIndex(p => p.prop_name === prop.prop_name);
  brick.value.props.splice(index + 1, 0, { ...prop, prop_name: copyName });
  saveBrick(brick.value);
}

function onRemoveProp(prop: Prop) {
  // TODO: Implementare conferma tramite UModal per sicurezza
  if (prop === newProp.value) {
    newProp.value = null;
    return;
  }
  brick.value.props = brick.value.props.filter(p => p !== prop);
  saveBrick(brick.value);
}

// Route Guard
onBeforeRouteLeave((_to, _from, next) => {
  if (newProp.value) {
    // TODO: Sostituire con una Modal
    const confirm = window.confirm("Unsaved changes. Discard?");
    confirm ? next() : next(false);
  } else next();
});

watch(() => brick.value, (newVal) => {
  saveBrick(newVal);
}, { deep: true });
</script>

<style scoped>
.props-container.grid, .props-container.adaptive-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr));
  gap: 1rem;
}

@media (max-width: 1024px) {
  .props-container.adaptive-grid {
    grid-template-columns: 1fr;
  }
}

.prop-row {
  transition: all 0.2s ease;
}

.border-dashed {
  background-image: linear-gradient(to bottom, var(--color-secondary-700) 50%, transparent 50%);
  background-size: 1px 8px;
  background-repeat: repeat-y;
  border-left: none;
}
</style>