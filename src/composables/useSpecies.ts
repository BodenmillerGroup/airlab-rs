import { computed, watch } from 'vue'
import { unref, type MaybeRef } from "vue"
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useSpeciesStore } from '@/stores/species'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { SpeciesView } from "@/modules/species/types"

type UseSpeciesOptions = {
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useSpecies(options: UseSpeciesOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const speciesStore = useSpeciesStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Species', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      name: 'name',
      acronym: 'acronym',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Species', 'id', 'desc')
    }

    const dir = first.order === 'asc' ? 'asc' : 'desc'
    return createOrder('Species', field, dir)
  }

  async function loadSpecies() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        speciesStore.resetListQuery()
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
        sf('Species', 'group_id', 'eq', groupId),
        ...dynamicFilters
      )

      const order = buildOrder()
      await speciesStore.loadListQuery({
        groupId,
        filters: finalFilters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<SpeciesView[]>(() =>
    speciesStore.listSpecies.map(species => ({
        id: species.id,
        groupId: species.groupId,
        name: species.name,
        acronym: species.acronym,
        meta: species.meta,
        createdAt: species.createdAt,
      }))
  )

  watch(() => filterStore.filters, loadSpecies, { deep: true })
  watch(() => speciesStore.page, loadSpecies, { immediate: true })
  watch(() => speciesStore.limit, loadSpecies)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadSpecies)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadSpecies, { deep: true })

  return {
    items,
    loading: computed(() => speciesStore.loading),
    reload: speciesStore.reloadListQuery,
  }
}
