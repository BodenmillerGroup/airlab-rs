import { computed, ref, watch, type Ref } from 'vue'
import { useGroupStore } from '@/stores/group'
import { useConjugateStore } from '@/stores/conjugate'
import { useLotStore } from '@/stores/lot'
import { useCloneStore } from '@/stores/clone'
import { useProteinStore } from '@/stores/protein'
import { useTagStore } from '@/stores/tag'
import { useValidationStore } from '@/stores/validation'

import type { ConjugateDto, ConjugateView } from '@/modules/conjugate/types'

export function useSelectedConjugateViews(
  selectedTagConjugates: Ref<Map<number, Set<ConjugateDto>>>
) {
  const groupStore = useGroupStore()
  const conjugateStore = useConjugateStore()
  const lotStore = useLotStore()
  const cloneStore = useCloneStore()
  const proteinStore = useProteinStore()
  const tagStore = useTagStore()
  const validationStore = useValidationStore()

  const loading = ref(false)

  function flattenSelected() {
    return [...selectedTagConjugates.value.values()].flatMap((set) => [...set])
  }

  async function ensureRelatedData() {
    const groupId = groupStore.activeGroupId
    if (typeof groupId !== 'number') return

    const selected = flattenSelected()
    if (selected.length === 0) return

    loading.value = true
    try {
      const lotIds = [...new Set(selected.map((c) => c.lotId))]
      const tagIds = [...new Set(selected.map((c) => c.tagId))]

      await Promise.all([
        lotStore.fetchByIds(groupId, lotIds),
        tagStore.fetchByIds(groupId, tagIds),
      ])

      const cloneIds = [...new Set(
        lotIds
          .map((id) => lotStore.getLot(id)?.cloneId)
          .filter((id): id is number => typeof id === 'number')
      )]
      await cloneStore.fetchByIds(cloneIds)

      const proteinIds = [...new Set(
        cloneIds
          .map((id) => cloneStore.getClone(id)?.proteinId)
          .filter((id): id is number => typeof id === 'number')
      )]
      await proteinStore.fetchByIds(groupId, proteinIds)

      await validationStore.fetchByCloneIds(groupId, cloneIds)
    } finally {
      loading.value = false
    }
  }

  watch(selectedTagConjugates, ensureRelatedData, { deep: true })
  watch(() => groupStore.activeGroupId, ensureRelatedData)

  const conjugateViewMap = computed(() => {
    const map = new Map<number, ConjugateView>()
    const selected = flattenSelected()

    for (const conjugate of selected) {
      const lot = lotStore.getLot(conjugate.lotId)
      const clone = lot ? cloneStore.getClone(lot.cloneId) : undefined
      const protein = clone ? proteinStore.getProtein(clone.proteinId) : undefined
      const tag = tagStore.getTag(conjugate.tagId)
      const validations = clone ? (validationStore.cloneValidationMap[clone.id] ?? []) : []

      const view: ConjugateView = {
        id: conjugate.id,
        groupId: conjugate.groupId,
        createdBy: conjugate.createdBy,
        labeledBy: conjugate.labeledBy,
        finishedBy: conjugate.finishedBy,
        lotId: conjugate.lotId,
        tagId: conjugate.tagId,
        storageId: conjugate.storageId,
        status: conjugate.status,
        tubeNumber: conjugate.tubeNumber,
        concentration: conjugate.concentration,
        description: conjugate.description,
        isArchived: conjugate.isArchived,
        meta: conjugate.meta,
        labeledAt: conjugate.labeledAt,
        createdAt: conjugate.createdAt,
        updatedAt: conjugate.updatedAt,
        customId: conjugate.customId,
        tagName: tag?.name ?? '—',
        tagMw: tag?.mw ?? null,
        proteinName: protein?.name ?? '—',
        proteinId: clone?.proteinId ?? 0,
        cloneName: clone?.name ?? '—',
        cloneId: lot?.cloneId ?? 0,
        lotName: lot?.number ?? '—',
        userName: '—',
        userId: conjugate.labeledBy,
        validations,
      }

      map.set(conjugate.id, view)
    }

    return map
  })

  const selectedTagConjugateViews = computed(() => {
    const map = new Map<number, Set<ConjugateView>>()
    selectedTagConjugates.value.forEach((set, tagId) => {
      const viewSet = new Set<ConjugateView>()
      set.forEach((conjugate) => {
        const view = conjugateViewMap.value.get(conjugate.id)
        if (view) viewSet.add(view)
      })
      map.set(tagId, viewSet)
    })
    return map
  })

  return {
    conjugateViewMap,
    selectedTagConjugateViews,
    loading,
  }
}
