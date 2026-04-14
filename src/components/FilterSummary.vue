<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useFilterStore } from '@/stores/useFilterStore'
import { useCollectionStore } from '@/stores/collection'
import { globalFilterConfig } from '@/filters/globalFilterConfig'

type Chip = {
  key: string
  label: string
  value: unknown
}

const filterStore = useFilterStore()
const collectionStore = useCollectionStore()

const activeChips = computed<Chip[]>(() => {
  return globalFilterConfig.flatMap(cfg => {
    const value = filterStore.filters[cfg.key]
    if (value === '' || value === null || value === undefined) return []
    if (Array.isArray(value)) {
      if (value.length === 0) return []
      return value.map(v => ({ key: cfg.key, label: cfg.label, value: v }))
    }
    return [{ key: cfg.key, label: cfg.label, value }]
  })
})

function displayValue(key: string, value: unknown) {
  if (key === 'collectionId') {
    const id = Number(value)
    const name = collectionStore.getCollection(id)?.name
    return name ?? String(value ?? '')
  }
  const cfg = globalFilterConfig.find(c => c.key === key)
  if (!cfg) return String(value ?? '')
  const opt = cfg.options?.find(o => o.value === value)
  return opt?.label ?? String(value ?? '')
}

function removeChip(key: string, value: unknown) {
  const cfg = globalFilterConfig.find(c => c.key === key)
  if (!cfg) return
  const current = filterStore.filters[key]
  if (Array.isArray(current)) {
    const next = current.filter(v => v !== value)
    filterStore.setFilter(key, next)
    return
  }
  filterStore.setFilter(key, cfg.type === 'select' ? [] : '')
}

onMounted(async () => {
  if (collectionStore.collections.length === 0) {
    await collectionStore.getCollections()
  }
})
</script>

<template>
  <div class="filter-summary">
    <span class="label">Filters</span>
    <v-chip
      v-for="chip in activeChips"
      :key="`${chip.key}-${String(chip.value)}`"
      closable
      size="small"
      density="compact"
      class="chip"
      @click.stop
      @click:close.stop="removeChip(chip.key, chip.value)"
    >
      {{ chip.label }}: {{ displayValue(chip.key, chip.value) }}
    </v-chip>
  </div>
</template>

<style scoped>
.filter-summary {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 4px;
  padding: 2px 0;
  min-height: 28px;
}
.label {
  font-weight: 600;
  margin-right: 2px;
  font-size: 0.82rem;
  line-height: 1.1;
  text-transform: uppercase;
  letter-spacing: 0.03em;
  color: rgb(var(--v-theme-on-surface), 0.68);
}
.chip {
  max-width: 220px;
  font-size: 0.77rem;
}
</style>
