<template>
  <v-list dense>
    <template v-for="meta in metaProps" :key="meta.name">
      <v-list-item dense two-line>
        <v-list-item-title>{{ meta.name }}</v-list-item-title>
        <v-list-item-subtitle v-if="meta.value.startsWith('http')">
          <a :href="meta.value" target="_blank">{{ meta.value }}</a>
        </v-list-item-subtitle>
        <v-list-item-subtitle v-else>
          {{ meta.value }}
        </v-list-item-subtitle>
      </v-list-item>
    </template>
  </v-list>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  item: Record<string, any>
}>()

const metaProps = computed(() => {
  if (!props.item?.meta) return []

  return Object.entries(props.item.meta as Record<string, unknown>).map(([name, value]) => ({
    name,
    value: String(value ?? ''),
  }))
})
</script>
