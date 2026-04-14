import { computed, watch } from 'vue'
import { unref, type MaybeRef } from "vue"
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useLotStore } from '@/stores/lot'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { ReturnType, SearchFilterInput } from '@/modules/json/api'
import type { LotView } from "@/modules/lot/types"

type UseLotsOptions = {
  cloneId?: MaybeRef<number | undefined>
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useLots(options: UseLotsOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const lotStore = useLotStore()
  const mainStore = useMainStore()


  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Lot', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      name: 'name',
      number: 'number',
      reference: 'reference',
      provider: 'name',
      clone: 'name',
      collection: 'name',
      status: 'status',
      validations: 'application',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Lot', 'id', 'desc')
    }

    const dir = first.order === 'asc' ? 'asc' : 'desc'
    const table: ReturnType =
      first.key === 'provider' ? 'Provider' :
      first.key === 'clone' ? 'Clone' :
      first.key === 'collection' ? 'Collection' :
      first.key === 'validations' ? 'Validation' :
      'Lot'
    return createOrder(table, field, dir)
  }

  async function loadLots() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        lotStore.resetListQuery()
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

      const baseFilters: SearchFilterInput[] = [
        sf('Lot', 'group_id', 'eq', groupId),
      ]
      const cloneId = unref(options.cloneId)
      if (typeof cloneId === "number") {
        baseFilters.push(sf('Lot', 'clone_id', 'eq', cloneId))
      }
      const finalFilters = createFilters(
        ...baseFilters,
        ...dynamicFilters
      )

      await lotStore.loadListQuery({
        groupId,
        filters: finalFilters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order: buildOrder(),
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<LotView[]>(() =>
    lotStore.listLots.map((lot) => ({
      id: lot.id,
      groupId: lot.groupId,
      createdBy: lot.createdBy,
      cloneId: lot.cloneId,
      providerId: lot.providerId,
      storageId: lot.storageId ?? null,
      collectionId: lot.collectionId ?? null,
      name: lot.name,
      reference: lot.reference,
      requestedBy: lot.requestedBy,
      approvedBy: lot.approvedBy,
      orderedBy: lot.orderedBy,
      receivedBy: lot.receivedBy,
      finishedBy: lot.finishedBy,
      number: lot.number,
      status: lot.status,
      purpose: lot.purpose,
      url: lot.url,
      price: lot.price,
      note: lot.note,
      requestedAt: lot.requestedAt,
      approvedAt: lot.approvedAt,
      orderedAt: lot.orderedAt,
      receivedAt: lot.receivedAt,
      finishedAt: lot.finishedAt,
      isArchived: lot.isArchived,
      meta: lot.meta,
      createdAt: lot.createdAt,
      updatedAt: lot.updatedAt,
    }))
  )

  watch(() => filterStore.filters, loadLots, { deep: true })
  watch(() => lotStore.page, loadLots, { immediate: true })
  watch(() => lotStore.limit, loadLots)
  watch(() => options.cloneId ? unref(options.cloneId) : undefined, loadLots)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadLots)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadLots, { deep: true })

  return {
    items,
    loading: computed(() => lotStore.loading),
    reload: lotStore.reloadListQuery,
  }
}
