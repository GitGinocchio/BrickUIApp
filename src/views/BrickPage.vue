<template>
    <p>Ciao</p>
</template>

<script setup lang="ts">
import { Brick } from 'interfaces/brick';
import { onMounted, PropType } from 'vue';



const props = defineProps({
  name: {
    type: String,
    required: true
  }
});

onMounted(() => {
    console.log(props.name);
});

</script>