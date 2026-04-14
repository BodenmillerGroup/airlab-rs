import { computed, watch } from 'vue'
import { unref, type MaybeRef } from "vue"
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useValidationStore } from '@/stores/validation'
import { useCloneStore } from '@/stores/clone'
import { useProteinStore } from '@/stores/protein'
import { useConjugateStore } from '@/stores/conjugate'
import { useSpeciesStore } from '@/stores/species'
import { useUserStore } from '@/stores/user'
import { useMemberStore } from '@/stores/member'
import { useLotStore } from '@/stores/lot'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { ReturnType, SearchFilterInput } from '@/modules/json/api'
import type { ValidationView } from "@/modules/validation/types"

type UseValidationsOptions = {
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useValidations(options: UseValidationsOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const validationStore = useValidationStore()
  const cloneStore = useCloneStore()
  const proteinStore = useProteinStore()
  const conjugateStore = useConjugateStore()
  const speciesStore = useSpeciesStore()
  const lotStore = useLotStore()
  const userStore = useUserStore()
  const memberStore = useMemberStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Validation', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      proteinName: 'name',
      cloneName: 'name',
      lotNumber: 'number',
      speciesName: 'name',
      tubeNumber: 'tube_number',
      id: 'id',
      application: 'application',
      status: 'status',
      antigenRetrievalType: 'antigen_retrieval_type',
      userName: 'name',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Validation', 'id', 'desc')
    }

    const dir = first.order === 'asc' ? 'asc' : 'desc'
    const table: ReturnType =
      first.key === 'proteinName' ? 'Protein' :
      first.key === 'cloneName' ? 'Clone' :
      first.key === 'lotNumber' ? 'Lot' :
      first.key === 'speciesName' ? 'Species' :
      first.key === 'tubeNumber' ? 'Conjugate' :
      first.key === 'userName' ? 'User' :
      'Validation'
    return createOrder(table, field, dir)
  }

  async function loadValidations() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        validationStore.resetListQuery()
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
        sf('Validation', 'group_id', 'eq', groupId),
        ...dynamicFilters
      )

      const order = buildOrder()
      await validationStore.loadListQuery({
        groupId,
        filters: finalFilters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<ValidationView[]>(() =>
    validationStore.listValidations.map((validation) => {
      const clone = cloneStore.getClone(validation.cloneId)
      const protein = clone ? proteinStore.getProtein(clone.proteinId) : undefined
      const lot = lotStore.getLot(validation.lotId)
      const species = speciesStore.getSpecies(validation.speciesId)
      const conjugate = conjugateStore.getConjugate(validation.conjugateId)
      const member = memberStore.getMemberById(validation.createdBy)
      const user = member ? userStore.getUserById(member.userId) : undefined

      return {
        id: validation.id,
        groupId: validation.groupId,
        createdBy: validation.createdBy,
        cloneId: validation.cloneId,
        cloneName: clone?.name ?? '-',
        proteinId: protein?.id ?? 0,
        proteinName: protein?.name ?? '-',
        lotId: validation.lotId,
        lotName: lot?.name ?? '-',
        userId: user?.id ?? 0,
        userName: user?.name ?? '-',
        lotNumber: lot?.number ?? '—',
        conjugateId: validation.conjugateId,
        tubeNumber: conjugate?.tubeNumber ?? '',
        speciesId: validation.speciesId,
        speciesName: species?.name ?? '-',
        application: validation.application,
        positiveControl: validation.positiveControl,
        negativeControl: validation.negativeControl,
        incubationConditions: validation.incubationConditions,
        concentration: validation.concentration,
        concentrationUnit: validation.concentrationUnit,
        tissue: validation.tissue,
        fixation: validation.fixation,
        fixationNotes: validation.fixationNotes,
        notes: validation.notes,
        status: validation.status,
        antigenRetrievalType: validation.antigenRetrievalType,
        antigenRetrievalTime: validation.antigenRetrievalTime,
        antigenRetrievalTemperature: validation.antigenRetrievalTemperature,
        saponin: validation.saponin,
        saponinConcentration: validation.saponinConcentration,
        methanolTreatment: validation.methanolTreatment,
        methanolTreatmentConcentration: validation.methanolTreatmentConcentration,
        surfaceStaining: validation.surfaceStaining,
        surfaceStainingConcentration: validation.surfaceStainingConcentration,
        meta: validation.meta,
        isArchived: validation.isArchived,
        validationFiles: [],
        createdAt: validation.createdAt,
        updatedAt: validation.updatedAt,
      }
    })
  )

  watch(() => filterStore.filters, loadValidations, { deep: true })
  watch(() => validationStore.page, loadValidations, { immediate: true })
  watch(() => validationStore.limit, loadValidations)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadValidations)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadValidations, { deep: true })

  return {
    items,
    loading: computed(() => validationStore.loading),
    reload: validationStore.reloadListQuery,
  }
}
