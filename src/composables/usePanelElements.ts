import { computed, watch, type Ref } from 'vue'
import { usePanelElementStore } from '@/stores/panel_element'
import { useConjugateStore } from '@/stores/conjugate'
import { useLotStore } from '@/stores/lot'
import { useProteinStore } from '@/stores/protein'
import { useCloneStore } from '@/stores/clone'
import { useTagStore } from '@/stores/tag'
import { useValidationStore } from '@/stores/validation'
import type { PanelElementView } from '@/modules/panel_element/types'

export function usePanelElements(
  panelId: Ref<number | null>,
  groupId: Ref<number | null>,
) {
  const panelElementStore = usePanelElementStore()
  const conjugateStore = useConjugateStore()
  const lotStore = useLotStore()
  const proteinStore = useProteinStore()
  const cloneStore = useCloneStore()
  const tagStore = useTagStore()
  const validationStore = useValidationStore()

  async function load() {
    if (typeof panelId?.value !== 'number') {
      panelElementStore.resetListQuery()
      return
    }
    if (typeof groupId?.value !== 'number') {
      panelElementStore.resetListQuery()
      return
    }

    await panelElementStore.loadListQuery({
      groupId: groupId.value,
      panelId: panelId.value,
    })
  }

  const items = computed<PanelElementView[]>(() => {
    const validationsByClone = validationStore.cloneValidationMap

    return panelElementStore.listPanelElements.flatMap((el) => {
      const conjugate = conjugateStore.getConjugate(el.conjugateId)
      if (!conjugate) return []

      const lot = lotStore.getLot(conjugate.lotId)
      if (!lot) return []

      const clone = cloneStore.getClone(lot.cloneId)
      if (!clone) return []

      const protein = proteinStore.getProtein(clone.proteinId)
      if (!protein) return []

      const tag = tagStore.getTag(conjugate.tagId)

      return [{
        id: conjugate.id,
        conjugateId: conjugate.id,
        tubeNumber: conjugate.tubeNumber,
        status: conjugate.status,
        finishedBy: 0,
        tagId: tag?.id ?? 0,
        tagName: tag?.name ?? '—',
        tagMw: tag?.mw ?? null,
        lotId: lot.id,
        lotNumber: lot.number,
        cloneId: clone.id,
        cloneName: clone.name,
        proteinId: protein.id,
        proteinName: protein.name,
        validations: validationsByClone[clone.id] ?? [],
        concentration: conjugate.concentration,
        actualConcentration: el.concentration,
        dilutionType: el.dilutionType,
        pipet: 0,
      }]
    })
  })

  watch([panelId, groupId], load, { immediate: true })

  return {
    items,
    loading: computed(() => panelElementStore.loading),
    reload: load,
  }
}
