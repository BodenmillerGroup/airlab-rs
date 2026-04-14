<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useFilterStore } from '@/stores/useFilterStore'
import { useCollectionStore } from '@/stores/collection'
import { globalFilterConfig } from '@/filters/globalFilterConfig'

const props = defineProps<{
  keyName: string
}>()

const filterStore = useFilterStore()
const collectionStore = useCollectionStore()
const config = computed(() =>
  globalFilterConfig.find(cfg => cfg.key === props.keyName)
)

const isCollectionFilter = computed(() => props.keyName === 'collectionId')

const collectionOptions = computed(() => {
  return [...collectionStore.collections]
    .map(item => ({ label: item.name, value: item.id }))
    .sort((a, b) => a.label.localeCompare(b.label))
})

const value = computed({
  get: () => filterStore.filters[props.keyName],
  set: val => {
    if (config.value?.type === 'number') {
      if (val === '' || val === null || val === undefined) {
        filterStore.setFilter(props.keyName, '')
        return
      }

      const parsed = typeof val === 'number' ? val : Number(val)
      filterStore.setFilter(props.keyName, Number.isNaN(parsed) ? '' : parsed)
      return
    }

    filterStore.setFilter(props.keyName, val)
  },
})

onMounted(async () => {
  if (isCollectionFilter.value && collectionStore.collections.length === 0) {
    await collectionStore.getCollections()
  }
})
</script>

<template>
  <v-select
    v-if="config && isCollectionFilter"
    v-model="value"
    class="filter-field"
    :label="config.label"
    :items="collectionOptions"
    item-title="label"
    item-value="value"
    :clearable="true"
    density="compact"
    hide-details
    variant="outlined"
    prepend-icon="mdi-filter-outline"
  />
  <component
    v-else-if="config"
    :is="config.type === 'select' ? 'v-select' : 'v-text-field'"
    v-model="value"
    class="filter-field"
    :label="config.label"
    :items="config.options"
    :item-title="'label'"
    :item-value="'value'"
    :multiple="config.multiple"
    :clearable="true"
    :type="config.type === 'number' ? 'number' : undefined"
    density="compact"
    hide-details
    variant="outlined"
    prepend-icon="mdi-filter-outline"
  />
</template>

<style scoped>
.filter-field {
  --filter-field-height: 38px;
}

.filter-field :deep(.v-field__input) {
  min-height: var(--filter-field-height);
  padding-top: 0;
  padding-bottom: 0;
  font-size: 0.92rem;
}

.filter-field :deep(.v-label.v-field-label) {
  font-size: 0.84rem;
}

.filter-field :deep(.v-field__prepend-inner) {
  padding-top: 0;
}
</style>
