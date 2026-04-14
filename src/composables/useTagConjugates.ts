import { computed, watch } from 'vue'
import { unref, type MaybeRef } from 'vue'
import { useGroupStore } from '@/stores/group'
import { useConjugateStore } from '@/stores/conjugate'
import { useLotStore } from '@/stores/lot'
import { useCloneStore } from '@/stores/clone'
import { useProteinStore } from '@/stores/protein'
import { useTagStore } from '@/stores/tag'
import { useValidationStore } from '@/stores/validation'
import { useSpeciesStore } from '@/stores/species'
import { useMainStore } from '@/stores/main'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { ConjugateDto } from '@/modules/conjugate/types'
import type { ValidationDto } from '@/modules/validation/types'

export type TagConjugateView = {
  id: number
  tagId: number
  tagName: string
  tagMw: number | null
  tubeNumber: number
  status: number
  lotId: number
  lotNumber: string
  cloneId: number
  cloneName: string
  proteinId: number
  proteinName: string
  reactivity: number[]
  validations: ValidationDto[]
  concentration: number
  description: string | null
  customId: string
  conjugate: ConjugateDto
}

type UseTagConjugateOptions = {
  tagId: MaybeRef<number | undefined>
}

export function useTagConjugates(options: UseTagConjugateOptions) {
  const groupStore = useGroupStore()
  const conjugateStore = useConjugateStore()
  const lotStore = useLotStore()
  const cloneStore = useCloneStore()
  const proteinStore = useProteinStore()
  const tagStore = useTagStore()
  const validationStore = useValidationStore()
  const speciesStore = useSpeciesStore()
  const mainStore = useMainStore()

  async function load() {
    const groupId = groupStore.activeGroupId
    const tagId = unref(options.tagId)
    if (typeof groupId !== 'number' || typeof tagId !== 'number') {
      conjugateStore.resetListQuery()
      return
    }

    try {
      const filters = [
        sf('Conjugate', 'group_id', 'eq', groupId),
        sf('Conjugate', 'tag_id', 'eq', tagId),
      ]
      const order = createOrder('Conjugate', 'id', 'desc')
      await conjugateStore.loadListQuery({
        groupId,
        filters: createFilters(...filters),
        order,
      })

      const cloneIds = [...new Set(
        conjugateStore.listConjugates
          .map((conjugate) => lotStore.getLot(conjugate.lotId)?.cloneId)
          .filter((id): id is number => typeof id === 'number')
      )]

      const reactivityIds = [...new Set(
        cloneIds.flatMap(id => cloneStore.getClone(id)?.reactivity ?? [])
      )]
      await speciesStore.fetchByIds(groupId, reactivityIds)
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<TagConjugateView[]>(() => {
    const cloneValidationMap = validationStore.cloneValidationMap

    return conjugateStore.listConjugates.flatMap(conjugate => {
      const lot = lotStore.getLot(conjugate.lotId)
      if (!lot) return []

      const clone = cloneStore.getClone(lot.cloneId)
      if (!clone) return []

      const protein = proteinStore.getProtein(clone.proteinId)
      const tag = tagStore.getTag(conjugate.tagId)

      return [{
        id: conjugate.id,
        tagId: conjugate.tagId,
        tagName: tag?.name ?? '—',
        tagMw: tag?.mw ?? null,
        tubeNumber: conjugate.tubeNumber,
        status: conjugate.status,
        lotId: lot.id,
        lotNumber: lot.number,
        cloneId: clone.id,
        cloneName: clone.name,
        proteinId: clone.proteinId,
        proteinName: protein?.name ?? '—',
        reactivity: clone.reactivity ?? [],
        validations: cloneValidationMap[clone.id] ?? [],
        concentration: conjugate.concentration,
        description: conjugate.description,
        customId: conjugate.customId,
        conjugate,
      }]
    })
  })

  watch(() => groupStore.activeGroupId, load, { immediate: true })
  watch(() => unref(options.tagId), load)
  watch(() => conjugateStore.page, load)

  return {
    items,
    loading: computed(() => conjugateStore.loading),
    reload: conjugateStore.reloadListQuery,
  }
}
