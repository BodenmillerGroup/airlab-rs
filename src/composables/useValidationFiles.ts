import { computed, watch } from 'vue'
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useValidationFileStore } from '@/stores/validation_file'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { ValidationFileView } from "@/modules/validation_file/types"

export function useValidationFiles() {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const validation_fileStore = useValidationFileStore()
  const mainStore = useMainStore()

  async function loadValidationFiles() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        validation_fileStore.resetListQuery()
        return
      }

      const dynamicFilters = globalFilterConfig
        .filter(cfg => {
          const val = filterStore.filters[cfg.key]
          return val !== '' && !(Array.isArray(val) && val.length === 0)
        })
        .map((cfg): SearchFilterInput => sf(
          cfg.table,
          cfg.field,
          cfg.op,
          filterStore.filters[cfg.key],
        ))

      const finalFilters = createFilters(
        sf('ValidationFile', 'group_id', 'eq', groupId),
        ...dynamicFilters
      )

      const order = createOrder('ValidationFile', 'id', 'desc')
      await validation_fileStore.loadListQuery({
        groupId,
        filters: finalFilters,
        order,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<ValidationFileView[]>(() =>
    validation_fileStore.listFiles.map(validation_file => ({
        id: validation_file.id,
      } as any))
  )

  watch(() => filterStore.filters, loadValidationFiles, { deep: true })
  watch(() => validation_fileStore.page, loadValidationFiles, { immediate: true })
  watch(() => validation_fileStore.limit, loadValidationFiles)

  return {
    items,
    loading: computed(() => validation_fileStore.loading),
    reload: validation_fileStore.reloadListQuery,
  }
}
