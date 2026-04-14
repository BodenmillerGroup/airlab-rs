import { computed, unref, watch, type MaybeRef } from 'vue'
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useCloneStore } from '@/stores/clone'
import { useConjugateStore } from '@/stores/conjugate'
import { useLotStore } from '@/stores/lot'
import { useProteinStore } from '@/stores/protein'
import { useSpeciesStore } from '@/stores/species'
import { useValidationStore } from '@/stores/validation'
import { useMainStore } from '@/stores/main'
import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { CloneView } from '@/modules/clone/types'
import { applicationToString, validationStatusToString } from '@/utils/converters'

type UseCloneOptions = {
  conjugateId?: MaybeRef<number | undefined>
  proteinId?: MaybeRef<number | undefined>
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useClones(options: UseCloneOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const cloneStore = useCloneStore()
  const conjugateStore = useConjugateStore()
  const lotStore = useLotStore()
  const proteinStore = useProteinStore()
  const speciesStore = useSpeciesStore()
  const validationStore = useValidationStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Clone', 'id', 'desc')
    }

    const keyMap: Record<string, { table: 'Clone' | 'Protein' | 'Species' | 'Validation'; field: string }> = {
      id: { table: 'Clone', field: 'id' },
      protein: { table: 'Protein', field: 'name' },
      name: { table: 'Clone', field: 'name' },
      species: { table: 'Species', field: 'name' },
      isPhospho: { table: 'Clone', field: 'is_phospho' },
      application: { table: 'Clone', field: 'application' },
      validations: { table: 'Validation', field: 'application' },
    }

    const orderTarget = keyMap[first.key]
    if (!orderTarget) {
      return createOrder('Clone', 'id', 'desc')
    }

    return createOrder(
      orderTarget.table,
      orderTarget.field,
      first.order === 'asc' ? 'asc' : 'desc',
    )
  }

  async function loadClones() {
    const groupId = groupStore.activeGroupId
    if (typeof groupId !== 'number') {
      cloneStore.resetListQuery()
      return
    }

    try {
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
        sf('Clone', 'group_id', 'eq', groupId),
      ]

      const conjugateId = unref(options.conjugateId)
      if (typeof conjugateId === 'number') {
        let conjugate = conjugateStore.getConjugate(conjugateId)
        if (!conjugate) {
          conjugate = (await conjugateStore.fetchByIds(groupId, [conjugateId]))[0]
        }

        const lotId = conjugate?.lotId
        if (typeof lotId !== 'number') {
          cloneStore.resetListQuery()
          return
        }

        let lot = lotStore.getLotById(lotId)
        if (!lot) {
          lot = (await lotStore.fetchByIds(groupId, [lotId], true))[0]
        }

        const cloneId = lot?.cloneId
        if (typeof cloneId !== 'number') {
          cloneStore.resetListQuery()
          return
        }

        baseFilters.push(sf('Clone', 'id', 'eq', cloneId))
      }

      const proteinId = unref(options.proteinId)
      if (typeof proteinId === 'number') {
        baseFilters.push(sf('Clone', 'protein_id', 'eq', proteinId))
      }

      const filters = createFilters(...baseFilters, ...dynamicFilters)
      await cloneStore.loadListQuery({
        groupId,
        filters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order: buildOrder(),
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<CloneView[]>(() => {
    void cloneStore.listClones
    void proteinStore.revision
    void speciesStore.revision
    void validationStore.cloneValidationMap

    return cloneStore.listClones.flatMap((clone) => {
      const protein = proteinStore.getProtein(clone.proteinId)
      const species = speciesStore.getSpecies(clone.speciesId)
      const validations = validationStore.cloneValidationMap[clone.id] || []

      return [{
        id: clone.id,
        name: clone.name,
        groupId: clone.groupId,
        proteinName: protein?.name ?? '—',
        proteinId: clone.proteinId,
        speciesName: species?.name ?? '—',
        isPolyclonal: clone.isPolyclonal,
        isPhospho: clone.isPhospho,
        createdBy: clone.createdBy,
        speciesId: clone.speciesId,
        isotype: clone.isotype,
        epitope: clone.epitope,
        reactivity: clone.reactivity,
        application: clone.application,
        isArchived: clone.isArchived,
        createdAt: clone.createdAt,
        updatedAt: clone.updatedAt,
        validations: validations.map(v => ({
          id: v.id,
          application: applicationToString(v.application),
          status: validationStatusToString(v.status),
        })),
      }]
    })
  })

  watch(
    [
      () => filterStore.filters,
      () => cloneStore.page,
      () => cloneStore.limit,
      () => options.globalFilter ? unref(options.globalFilter) : undefined,
      () => options.sortBy ? unref(options.sortBy) : undefined,
      () => options.conjugateId ? unref(options.conjugateId) : undefined,
      () => options.proteinId ? unref(options.proteinId) : undefined,
    ],
    loadClones,
    { deep: true, immediate: true }
  )

  return {
    items,
    loading: computed(() => cloneStore.loading),
    reload: cloneStore.reloadListQuery,
  }
}
