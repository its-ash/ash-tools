<script setup lang="ts">
import type { Tool } from '../composables/useTools'

const props = defineProps<{ tool: Tool, size?: number }>()
const px = computed(() => `${props.size ?? 40}px`)
</script>

<template>
  <svg
    class="tool-tile shrink-0"
    viewBox="0 0 24 24"
    aria-hidden="true"
    :style="{ color: categoryColor(tool.category), width: px, height: px }"
    v-html="tool.icon"
  />
</template>
