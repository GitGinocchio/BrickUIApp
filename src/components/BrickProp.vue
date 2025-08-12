<template>
  <n-tooltip :trigger="prop.description ? 'hover' : 'manual'" placement="bottom">
    <template #trigger>
      <div class="brick-prop">
        <label> {{ formattedPropName }}</label>
        <n-input 
          v-if="prop.prop_type == 'string' || prop.prop_type == 'any'" 
          v-model:value="prop.value"
          :placeholder="prop.default ? prop.default : 'Type a string value...'"
          clearable 
        />
        <n-input-number
          v-if="prop.prop_type == 'int' || prop.prop_type == 'float'"
          v-model:value="prop.value"
          :precision="prop.prop_type == 'int' ? 0 : 2"
          :min="prop.min"
          :max="prop.max"
          :default-value="prop.default"
          :placeholder="prop.default ? `${prop.default}` : 'Type a integer value...'"
          clearable
        />
        <!-- aggiungere il numero minimo e massimo di selezioni, non esiste un metodo base per farlo -->
        <n-select
          v-if="prop.prop_type == 'string-select' || 
                prop.prop_type == 'int-select'    || 
                prop.prop_type == 'float-select'  ||
                prop.prop_type == 'select'"
          v-model:value="prop.selected"
          :options="propOptions"
          :placeholder="prop.default_selected ? `${prop.default_selected}` : 'select a value...'"
          multiple
          clearable
        />
        <!-- Aggiungere la validazione dell'input in quanto non viene fatta -->
        <n-dynamic-tags 
          v-if="prop.prop_type == 'array'         ||
                prop.prop_type == 'int-array'     ||
                prop.prop_type == 'float-array'   ||
                prop.prop_type == 'string-array'"
          v-model:value="propDynamicTags"
          :default-value="defaultPropDynamicTags"
          :round="true"
          :max="prop.max_items"
        />

      </div>
    </template>
    {{ prop.description }}
  </n-tooltip>
</template>

<script setup lang="ts">
import { NInput, NInputNumber, NTooltip, NSelect, NDynamicTags } from 'naive-ui';
import { computed, onMounted, ref, type PropType } from 'vue';
import { Prop as BrickPropType } from 'interfaces/brick';
import { SelectMixedOption } from 'naive-ui/es/select/src/interface';

const formattedPropName = computed(() => {
  const name = prop.prop_name
  if (!name) return ''
  if (/^[A-Z]/.test(name)) return name
  return `${name.charAt(0).toUpperCase() + name.slice(1)}`
});

const propOptions = computed(() => {
  if (
    prop.prop_type !== 'select' &&
    prop.prop_type !== 'string-select' &&
    prop.prop_type !== 'int-select' &&
    prop.prop_type !== 'float-select'
  ) {
    return [];
  }

  return prop.options.map(option => ({ label: option, value: option }));
});

const propDynamicTags = ref<Array<{ label: string, value: any}>>();

onMounted(() => {
  if (
    (prop.prop_type === 'array' ||
     prop.prop_type === 'string-array' ||
     prop.prop_type === 'int-array' ||
     prop.prop_type === 'float-array') &&
    prop.values
  ) {
    propDynamicTags.value = prop.values.map(v => ({
      label: String(v),
      value: v
    }));
  }
});

const defaultPropDynamicTags = computed(() => {
  if (
    prop.prop_type !== 'array' &&
    prop.prop_type !== 'string-array' &&
    prop.prop_type !== 'int-array' &&
    prop.prop_type !== 'float-array' ||
    !prop.default
  ) {
    return [];
  }

  return prop.default.map(value => ({ label: value, value: value }));
})


const { prop } = defineProps({
  prop: {
    type: Object as PropType<BrickPropType>,
    required: true
  }
});

const emit = defineEmits({

});
</script>

<style scoped>
.brick-prop {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.brick-prop label {
  margin-left: 0rem;
}
</style>