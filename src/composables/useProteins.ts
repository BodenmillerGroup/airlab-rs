import { computed, watch } from 'vue'
import { unref, type MaybeRef } from "vue"
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useProteinStore } from '@/stores/protein'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { ProteinView } from "@/modules/protein/types"

type UseProteinsOptions = {
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useProteins(options: UseProteinsOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const proteinStore = useProteinStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Protein', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      name: 'name',
      description: 'description',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Protein', 'id', 'desc')
    }

    const dir = first.order === 'asc' ? 'asc' : 'desc'
    return createOrder('Protein', field, dir)
  }

  async function loadProteins() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        proteinStore.resetListQuery()
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
        sf('Protein', 'group_id', 'eq', groupId),
        ...dynamicFilters
      )

      const order = buildOrder()
      await proteinStore.loadListQuery({
        groupId,
        filters: finalFilters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<ProteinView[]>(() =>
    proteinStore.listProteins.map((protein) => ({
      id: protein.id,
      groupId: protein.groupId,
      createdBy: protein.createdBy,
      name: protein.name,
      description: protein.description,
      meta: protein.meta,
      createdAt: protein.createdAt,
    }))
  )

  watch(() => filterStore.filters, loadProteins, { deep: true })
  watch(() => proteinStore.page, loadProteins, { immediate: true })
  watch(() => proteinStore.limit, loadProteins)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadProteins)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadProteins, { deep: true })

  return {
    items,
    loading: computed(() => proteinStore.loading),
    reload: proteinStore.reloadListQuery,
  }
}
