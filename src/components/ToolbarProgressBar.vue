<template>
  <v-progress-linear
    :active="processing"
    :color="color"
    :indeterminate="indeterminate"
    :value="progress"
    class="ma-0"
    height="4"
    style="top: -2px"
  />
</template>

<script setup lang="ts">
const props = defineProps<{
  processing: boolean
  progress: number
  indeterminate: boolean
  color: string
}>()
</script>
