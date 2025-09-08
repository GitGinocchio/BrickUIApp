<template>
  <svg
    :width="gridWidth"
    :height="gridHeight"
    style="display: block;"
  >
    <rect
      v-for="(_, index) in totalSquares"
      :key="index"
      :x="getX(index)"
      :y="getY(index)"
      :width="width"
      :height="height"
      :style="getRectStyle(index)"
      @mouseenter="handleMouseEnter(index)"
      @mouseleave="handleMouseLeave"
    />
  </svg>
</template>

<script lang="ts" setup>
import { ref, computed, type HTMLAttributes } from "vue";

interface InteractiveGridPatternProps {
  className?: HTMLAttributes["class"];
  squaresClassName?: HTMLAttributes["class"];
  width?: number;
  height?: number;
  squares?: [number, number];
}

const props = withDefaults(defineProps<InteractiveGridPatternProps>(), {
  width: 40,
  height: 40,
  squares: () => [24, 24],
});

const horizontal = computed(() => props.squares[0]);
const vertical = computed(() => props.squares[1]);
const totalSquares = computed(() => horizontal.value * vertical.value);
const hoveredSquare = ref<number | null>(null);

const gridWidth = computed(() => props.width * horizontal.value);
const gridHeight = computed(() => props.height * vertical.value);

// Stile SVG
const svgStyle = computed(() => ({
  position: 'absolute',
  top: '0',
  left: '0',
  width: '100%',
  height: '100%',
  border: '1px solid rgba(156, 163, 175, 0.3)', // border-gray-400/30
}));

function getX(index: number) {
  return (index % horizontal.value) * props.width;
}

function getY(index: number) {
  return Math.floor(index / horizontal.value) * props.height;
}

// Stile dei rettangoli
function getRectStyle(index: number) {
  return {
    stroke: 'rgba(156, 163, 175, 0.3)', // stroke-gray-400/30
    fill: hoveredSquare.value === index ? 'rgba(156, 163, 175, 0.3)' : 'transparent',
    transition: hoveredSquare.value === index
      ? 'all 0.1s ease-in-out'
      : 'all 1s ease-in-out',
  };
}

function handleMouseEnter(index: number) {
  hoveredSquare.value = index;
}

function handleMouseLeave() {
  hoveredSquare.value = null;
}
</script>
