<template>
  <v-container fluid>
    <v-row>
      <v-col align="center" justify="center">
        <v-progress-circular indeterminate color="primary" />
        <h5>{{ text }}</h5>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
defineProps<{
  text: string
}>()
</script>
