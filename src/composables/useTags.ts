import { computed, watch } from 'vue'
import { unref, type MaybeRef } from "vue"
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useTagStore } from '@/stores/tag'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { TagView } from "@/modules/tag/types"

type UseTagsOptions = {
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useTags(options: UseTagsOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const tagStore = useTagStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Tag', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      name: 'name',
      description: 'description',
      isMetal: 'is_metal',
      isFluorophore: 'is_fluorophore',
      isEnzyme: 'is_enzyme',
      isBiotin: 'is_biotin',
      isOther: 'is_other',
      mw: 'mw',
      emission: 'emission',
      excitation: 'excitation',
      status: 'status',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Tag', 'id', 'desc')
    }

    const dir = first.order === 'asc' ? 'asc' : 'desc'
    return createOrder('Tag', field, dir)
  }

  async function loadTags() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        tagStore.resetListQuery()
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
        sf('Tag', 'group_id', 'eq', groupId),
        ...dynamicFilters
      )

      const order = buildOrder()
      await tagStore.loadListQuery({
        groupId,
        filters: finalFilters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<TagView[]>(() =>
    tagStore.listTags.map((tag) => ({
      id: tag.id,
      groupId: tag.groupId,
      name: tag.name,
      isMetal: tag.isMetal,
      isFluorophore: tag.isFluorophore,
      isEnzyme: tag.isEnzyme,
      isBiotin: tag.isBiotin,
      isOther: tag.isOther,
      status: tag.status,
      description: tag.description,
      mw: tag.mw,
      emission: tag.emission,
      excitation: tag.excitation,
      meta: tag.meta,
      createdAt: tag.createdAt,
    }))
  )

  watch(() => filterStore.filters, loadTags, { deep: true })
  watch(() => tagStore.page, loadTags, { immediate: true })
  watch(() => tagStore.limit, loadTags)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadTags)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadTags, { deep: true })

  return {
    items,
    loading: computed(() => tagStore.loading),
    reload: tagStore.reloadListQuery,
  }
}
