import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { ref, nextTick, effectScope } from 'vue'
import { flushPromises } from '@vue/test-utils'

import { usePanelElements } from '@/composables/usePanelElements'

const loadListQueryMock = vi.fn()
const resetListQueryMock = vi.fn()

const panelElementState = {
  listPanelElements: [] as any[],
  loading: false,
  loadListQuery: loadListQueryMock,
  resetListQuery: resetListQueryMock,
}

const conjugateState = {
  getConjugate: vi.fn(),
}

const lotState = {
  getLot: vi.fn(),
}

const cloneState = {
  getClone: vi.fn(),
}

const proteinState = {
  getProtein: vi.fn(),
}

const tagState = {
  getTag: vi.fn(),
}

const validationState = {
  cloneValidationMap: {} as Record<number, any[]>,
}

vi.mock('@/stores/panel_element', () => ({
  usePanelElementStore: () => panelElementState,
}))

vi.mock('@/stores/conjugate', () => ({
  useConjugateStore: () => conjugateState,
}))

vi.mock('@/stores/lot', () => ({
  useLotStore: () => lotState,
}))

vi.mock('@/stores/clone', () => ({
  useCloneStore: () => cloneState,
}))

vi.mock('@/stores/protein', () => ({
  useProteinStore: () => proteinState,
}))

vi.mock('@/stores/tag', () => ({
  useTagStore: () => tagState,
}))

vi.mock('@/stores/validation', () => ({
  useValidationStore: () => validationState,
}))

describe('usePanelElements', () => {
  beforeEach(() => {
    setActivePinia(createPinia())

    panelElementState.listPanelElements = [
      { id: 31, panelId: 5, conjugateId: 12, dilutionType: 2, concentration: 1.5 },
    ]
    panelElementState.loading = false

    loadListQueryMock.mockReset()
    resetListQueryMock.mockReset()

    conjugateState.getConjugate.mockReset()
    lotState.getLot.mockReset()
    cloneState.getClone.mockReset()
    proteinState.getProtein.mockReset()
    tagState.getTag.mockReset()

    loadListQueryMock.mockResolvedValue(panelElementState.listPanelElements)

    conjugateState.getConjugate.mockReturnValue({
      id: 12,
      lotId: 22,
      tagId: 32,
      tubeNumber: 42,
      status: 1,
      concentration: 9.5,
    })
    lotState.getLot.mockReturnValue({ id: 22, cloneId: 52, number: 'LOT-22' })
    cloneState.getClone.mockReturnValue({ id: 52, proteinId: 62, name: 'Clone 52' })
    proteinState.getProtein.mockReturnValue({ id: 62, name: 'Protein 62' })
    tagState.getTag.mockReturnValue({ id: 32, name: 'Tag 32', mw: 50 })
    validationState.cloneValidationMap = { 52: [{ id: 72 }] as any[] }
  })

  it('loads panel elements and builds PanelElementView', async () => {
    const scope = effectScope()
    const api = scope.run(() => usePanelElements(ref(123), ref(123)))!

    await nextTick()
    await flushPromises()
    loadListQueryMock.mockClear()

    await api.reload()
    await flushPromises()

    expect(loadListQueryMock).toHaveBeenCalledWith({ groupId: 123, panelId: 123 })
    expect(api.loading.value).toBe(false)
    expect(api.items.value).toEqual([
      {
        id: 12,
        conjugateId: 12,
        tubeNumber: 42,
        status: 1,
        finishedBy: 0,
        tagId: 32,
        tagName: 'Tag 32',
        tagMw: 50,
        lotId: 22,
        lotNumber: 'LOT-22',
        cloneId: 52,
        cloneName: 'Clone 52',
        proteinId: 62,
        proteinName: 'Protein 62',
        validations: [{ id: 72 }],
        concentration: 9.5,
        actualConcentration: 1.5,
        dilutionType: 2,
        pipet: 0,
      },
    ])

    scope.stop()
  })

  it('resets the list query if panelId or groupId is missing', async () => {
    const scope = effectScope()
    scope.run(() => usePanelElements(ref<number | null>(null), ref<number | null>(null)))

    await nextTick()

    expect(resetListQueryMock).toHaveBeenCalled()
    expect(loadListQueryMock).not.toHaveBeenCalled()

    scope.stop()
  })
})
