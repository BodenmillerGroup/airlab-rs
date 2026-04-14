import { computed, watch } from 'vue'
import { unref, type MaybeRef } from "vue"
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useConjugateStore } from '@/stores/conjugate'
import { useLotStore } from '@/stores/lot'
import { useCloneStore } from '@/stores/clone'
import { useProteinStore } from '@/stores/protein'
import { useUserStore } from '@/stores/user'
import { useMemberStore } from '@/stores/member'
import { useTagStore } from '@/stores/tag'
import { useValidationStore } from '@/stores/validation'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { ConjugateView } from "@/modules/conjugate/types"

type UseConjugateOptions = {
  lotId?: MaybeRef<number | undefined>
  tagId?: MaybeRef<number | undefined>
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useConjugates(options: UseConjugateOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const conjugateStore = useConjugateStore()
  const lotStore = useLotStore()
  const cloneStore = useCloneStore()
  const proteinStore = useProteinStore()
  const userStore = useUserStore()
  const memberStore = useMemberStore()
  const mainStore = useMainStore()
  const tagStore = useTagStore()
  const validationStore = useValidationStore()

  const cloneValidationMap = computed(() => validationStore.cloneValidationMap)

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Conjugate', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      tubeNumber: 'tube_number',
      protein: 'name',
      clone: 'name',
      lot: 'name',
      tag: 'name',
      tagMw: 'mw',
      user: 'name',
      concentration: 'concentration',
      status: 'status',
      validations: 'application',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Conjugate', 'id', 'desc')
    }

    const dir = first.order === 'asc' ? 'asc' : 'desc'
    const table =
      first.key === 'protein' ? 'Protein' :
      first.key === 'clone' ? 'Clone' :
      first.key === 'lot' ? 'Lot' :
      first.key === 'tag' || first.key === 'tagMw' ? 'Tag' :
      first.key === 'user' ? 'User' :
      first.key === 'validations' ? 'Validation' :
      'Conjugate'
    return createOrder(table, field, dir)
  }

  async function loadConjugates() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        conjugateStore.resetListQuery()
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
        sf('Conjugate', 'group_id', 'eq', groupId),
      ]
      const lotId = unref(options.lotId)
      if (typeof lotId === "number") {
        baseFilters.push(sf('Conjugate', 'lot_id', 'eq', lotId))
      }
      const tagId = unref(options.tagId)
      if (typeof tagId === "number") {
        baseFilters.push(sf('Conjugate', 'tag_id', 'eq', tagId))
      }
      const finalFilters = createFilters(
        ...baseFilters,
        ...dynamicFilters
      )

      const order = buildOrder()
      await conjugateStore.loadListQuery({
        groupId,
        filters: finalFilters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<ConjugateView[]>(() =>
    conjugateStore.listConjugates.flatMap((conjugate) => {
      const lot = lotStore.getLot(conjugate.lotId)
      const member = memberStore.getMemberById(conjugate.labeledBy)
      const user = member ? userStore.getUserById(member.userId) : undefined
      const clone = lot ? cloneStore.getClone(lot.cloneId) : undefined
      const protein = clone ? proteinStore.getProtein(clone.proteinId) : undefined
      const tag = tagStore.getTag(conjugate.tagId)
      if (!tag) {
        return []
      }

      const validations = clone ? (cloneValidationMap.value[clone.id] ?? []) : []
      return [{
        id: conjugate.id,
        groupId: conjugate.groupId,
        createdBy: conjugate.createdBy,
        labeledBy: conjugate.labeledBy,
        finishedBy: conjugate.finishedBy,
        lotId: conjugate.lotId,
        tagId: conjugate.tagId,
        storageId: conjugate.storageId,
        tagName: tag.name,
        tagMw: tag.mw,
        status: conjugate.status,
        tubeNumber: conjugate.tubeNumber,
        concentration: conjugate.concentration,
        description: conjugate.description,
        proteinName: protein?.name ?? '—',
        proteinId: clone?.proteinId ?? 0,
        cloneName: clone?.name ?? '—',
        cloneId: lot?.cloneId ?? 0,
        lotName: lot?.number ?? '—',
        isArchived: conjugate.isArchived,
        userName: user?.name ?? '—',
        userId: member?.userId ?? conjugate.labeledBy ?? 0,
        meta: conjugate.meta,
        labeledAt: conjugate.labeledAt,
        createdAt: conjugate.createdAt,
        updatedAt: conjugate.updatedAt,
        customId: conjugate.customId,
        validations,
      }]
    })
  )

  watch(() => filterStore.filters, loadConjugates, { deep: true })
  watch(() => conjugateStore.page, loadConjugates, { immediate: true })
  watch(() => conjugateStore.limit, loadConjugates)
  watch(() => options.lotId ? unref(options.lotId) : undefined, loadConjugates)
  watch(() => options.tagId ? unref(options.tagId) : undefined, loadConjugates)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadConjugates)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadConjugates, { deep: true })

  return {
    items,
    loading: computed(() => conjugateStore.loading),
    reload: conjugateStore.reloadListQuery,
  }
}
