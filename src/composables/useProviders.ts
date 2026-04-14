import { computed, watch } from 'vue'
import { unref, type MaybeRef } from "vue"
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useProviderStore } from '@/stores/provider'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { ProviderView } from "@/modules/provider/types"

type UseProvidersOptions = {
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useProviders(options: UseProvidersOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const providerStore = useProviderStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Provider', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      name: 'name',
      description: 'description',
      url: 'url',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Provider', 'id', 'desc')
    }

    const dir = first.order === 'asc' ? 'asc' : 'desc'
    return createOrder('Provider', field, dir)
  }

  async function loadProviders() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        providerStore.resetListQuery()
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
        sf('Provider', 'group_id', 'eq', groupId),
        ...dynamicFilters
      )

      const order = buildOrder()
      await providerStore.loadListQuery({
        groupId,
        filters: finalFilters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<ProviderView[]>(() =>
    providerStore.listProviders.map(provider => ({
        id: provider.id,
        groupId: provider.groupId,
        name: provider.name,
        description: provider.description,
        url: provider.url,
        meta: provider.meta,
        createdAt: provider.createdAt,
      }))
  )

  watch(() => filterStore.filters, loadProviders, { deep: true })
  watch(() => providerStore.page, loadProviders, { immediate: true })
  watch(() => providerStore.limit, loadProviders)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadProviders)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadProviders, { deep: true })

  return {
    items,
    loading: computed(() => providerStore.loading),
    reload: providerStore.reloadListQuery,
  }
}
