import { defineStore } from 'pinia'
import { ref } from 'vue'
import { globalFilterConfig } from '@/filters/globalFilterConfig'

export const useFilterStore = defineStore('filter', () => {
  const filters = ref<Record<string, any>>({})

  // init with defaults
  globalFilterConfig.forEach(cfg => {
    filters.value[cfg.key] = cfg.type === 'select' ? [] : ''
  })
  filters.value.cloneGlobalSearch = ''
  filters.value.collectionGlobalSearch = ''
  filters.value.storageGlobalSearch = ''
  filters.value.providerGlobalSearch = ''
  filters.value.speciesGlobalSearch = ''
  filters.value.proteinGlobalSearch = ''
  filters.value.tagGlobalSearch = ''
  filters.value.memberGlobalSearch = ''
  filters.value.panelGlobalSearch = ''
  filters.value.validationGlobalSearch = ''
  filters.value.lotGlobalSearch = ''
  filters.value.conjugateGlobalSearch = ''

  function setFilter(key: string, value: any) {
    filters.value[key] = value
  }

  function reset(keys?: string[]) {
    const allKeys = keys ?? [
      ...globalFilterConfig.map(cfg => cfg.key),
      'cloneGlobalSearch',
      'collectionGlobalSearch',
      'storageGlobalSearch',
      'providerGlobalSearch',
      'speciesGlobalSearch',
      'proteinGlobalSearch',
      'tagGlobalSearch',
      'memberGlobalSearch',
      'panelGlobalSearch',
      'validationGlobalSearch',
      'lotGlobalSearch',
      'conjugateGlobalSearch',
    ]
    allKeys.forEach(key => {
      if (key.endsWith('GlobalSearch')) {
        filters.value[key] = ''
        return
      }
      const type = globalFilterConfig.find(cfg => cfg.key === key)?.type
      filters.value[key] = type === 'select' ? [] : ''
    })
  }

  return {
    filters,
    setFilter,
    reset,
  }
})
