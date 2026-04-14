import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

import { usePanelElementStore } from '@/stores/panel_element'
import { useMainStore } from '@/stores/main'
import { rpc, rpcSearch } from '@/modules/json/api'
import { useConjugateStore } from '@/stores/conjugate'
import { useLotStore } from '@/stores/lot'
import { useCloneStore } from '@/stores/clone'
import { useProteinStore } from '@/stores/protein'
import { useTagStore } from '@/stores/tag'
import { useValidationStore } from '@/stores/validation'

vi.mock('@/modules/json/api', () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

vi.mock('@/stores/main', () => ({
  useMainStore: vi.fn(),
}))

vi.mock('@/stores/conjugate', () => ({
  useConjugateStore: vi.fn(),
}))

vi.mock('@/stores/lot', () => ({
  useLotStore: vi.fn(),
}))

vi.mock('@/stores/clone', () => ({
  useCloneStore: vi.fn(),
}))

vi.mock('@/stores/protein', () => ({
  useProteinStore: vi.fn(),
}))

vi.mock('@/stores/tag', () => ({
  useTagStore: vi.fn(),
}))

vi.mock('@/stores/validation', () => ({
  useValidationStore: vi.fn(),
}))

describe('usePanelElementStore', () => {
  let store: ReturnType<typeof usePanelElementStore>
  let mainStoreMock: any
  let conjugateStoreMock: any
  let lotStoreMock: any
  let cloneStoreMock: any
  let proteinStoreMock: any
  let tagStoreMock: any
  let validationStoreMock: any

  const mockElements = [
    { id: 1, panelId: 10, name: 'A' },
    { id: 2, panelId: 10, name: 'B' },
    { id: 3, panelId: 20, name: 'C' },
  ] as any[]

  beforeEach(() => {
    setActivePinia(createPinia())

    mainStoreMock = {
      addNotification: vi.fn(),
      checkApiError: vi.fn(),
    }

    conjugateStoreMock = {
      fetchByIds: vi.fn(),
    }

    lotStoreMock = {
      fetchByIds: vi.fn(),
    }

    cloneStoreMock = {
      fetchByIds: vi.fn(),
      getClone: vi.fn(),
    }

    proteinStoreMock = {
      fetchByIds: vi.fn(),
    }

    tagStoreMock = {
      fetchByIds: vi.fn(),
    }

    validationStoreMock = {
      page: 3,
      setPage: vi.fn(),
      fetchByCloneIds: vi.fn(),
    }

      ; (useMainStore as any).mockReturnValue(mainStoreMock)
      ; (useConjugateStore as any).mockReturnValue(conjugateStoreMock)
      ; (useLotStore as any).mockReturnValue(lotStoreMock)
      ; (useCloneStore as any).mockReturnValue(cloneStoreMock)
      ; (useProteinStore as any).mockReturnValue(proteinStoreMock)
      ; (useTagStore as any).mockReturnValue(tagStoreMock)
      ; (useValidationStore as any).mockReturnValue(validationStoreMock)

    store = usePanelElementStore()
    vi.clearAllMocks()
  })

  // -------------------
  // State & mutations
  // -------------------

  it('sets entities correctly', () => {
    store.setEntities(mockElements)

    expect(store.ids).toHaveLength(3)
  })

  it('adds, updates and deletes entity', () => {
    store.addEntity(mockElements[0])
    store.updateEntity({ ...mockElements[0], name: 'X' })


    store.deleteEntity(1)
    expect(store.entities[1]).toBeUndefined()
    expect(store.ids).toHaveLength(0)
  })

  it('resets state', () => {
    store.setEntities(mockElements)
    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
    expect(store.total).toBe(0)
  })

  // -------------------
  // Getters
  // -------------------

  it('computes panelElements and hasPanelElement', () => {
    store.setEntities(mockElements)

    expect(store.panelElements).toHaveLength(3)
    expect(store.hasPanelElement(2)).toBe(true)
    expect(store.hasPanelElement(99)).toBe(false)
  })

  it('groups elements by panelId', () => {
    store.setEntities(mockElements)

    expect(store.byPanelId[10]).toHaveLength(2)
    expect(store.byPanelId[20]).toHaveLength(1)
  })

  // -------------------
  // RPC actions
  // -------------------

  it('getPanelElements fetches and stores data', async () => {
    ; (rpc as any).mockResolvedValue({
      items: mockElements,
      total: 3,
    })

    const res = await store.getPanelElements(10)

    expect(rpc).toHaveBeenCalledOnce()
    expect(store.ids).toHaveLength(3)
    expect(store.total).toBe(3)
    expect(res).toHaveLength(3)
  })

  it('loadListQuery stores visible ids across pages', async () => {
    ; (rpc as any)
      .mockResolvedValueOnce({
        items: [
          { id: 1, panelId: 10, conjugateId: 101, dilutionType: 1, concentration: 1.5 },
          { id: 2, panelId: 10, conjugateId: 102, dilutionType: 1, concentration: 2.5 },
        ],
        total: 3,
      })
      .mockResolvedValueOnce({
        items: [
          { id: 3, panelId: 10, conjugateId: 103, dilutionType: 2, concentration: 3.5 },
        ],
        total: 3,
      })

    conjugateStoreMock.fetchByIds.mockResolvedValue([
      { id: 101, lotId: 201, tagId: 301 },
      { id: 102, lotId: 202, tagId: 302 },
      { id: 103, lotId: 203, tagId: 303 },
    ])
    lotStoreMock.fetchByIds.mockResolvedValue([
      { id: 201, cloneId: 401 },
      { id: 202, cloneId: 402 },
      { id: 203, cloneId: 403 },
    ])
    cloneStoreMock.getClone
      .mockReturnValueOnce({ id: 401, proteinId: 501 })
      .mockReturnValueOnce({ id: 402, proteinId: 502 })
      .mockReturnValueOnce({ id: 403, proteinId: 503 })

    store.setLimit(2)

    const res = await store.loadListQuery({ groupId: 99, panelId: 10 })

    expect(rpc).toHaveBeenCalledTimes(2)
    expect(store.listIds).toEqual([1, 2, 3])
    expect(store.listPanelElements).toHaveLength(3)
    expect(conjugateStoreMock.fetchByIds).toHaveBeenCalledWith(99, [101, 102, 103])
    expect(lotStoreMock.fetchByIds).toHaveBeenCalledWith(99, [201, 202, 203])
    expect(tagStoreMock.fetchByIds).toHaveBeenCalledWith(99, [301, 302, 303])
    expect(cloneStoreMock.fetchByIds).toHaveBeenCalledWith([401, 402, 403])
    expect(proteinStoreMock.fetchByIds).toHaveBeenCalledWith(99, [501, 502, 503])
    expect(validationStoreMock.setPage).toHaveBeenNthCalledWith(1, 1)
    expect(validationStoreMock.fetchByCloneIds).toHaveBeenCalledWith(99, [401, 402, 403])
    expect(validationStoreMock.setPage).toHaveBeenNthCalledWith(2, 3)
    expect(res).toHaveLength(3)
  })

  it('reloadListQuery reruns the active query', async () => {
    ; (rpc as any).mockResolvedValue({
      items: [{ id: 1, panelId: 10, conjugateId: 101, dilutionType: 1, concentration: 1.5 }],
      total: 1,
    })

    conjugateStoreMock.fetchByIds.mockResolvedValue([])
    lotStoreMock.fetchByIds.mockResolvedValue([])

    await store.loadListQuery({ groupId: 99, panelId: 10 })
    await store.reloadListQuery()

    expect(rpc).toHaveBeenCalledTimes(2)
  })

  it('createPanelElement inserts and notifies', async () => {
    ; (rpc as any).mockResolvedValue(mockElements[0])

    const res = await store.createPanelElement({ panelId: 10, name: 'A' } as any)

    expect(store.entities[1]).toBeDefined()
    expect(mainStoreMock.addNotification).toHaveBeenCalled()
    expect(res.id).toBe(1)
  })

  it('updatePanelElement updates and notifies', async () => {
    store.setEntities([mockElements[0]])

      ; (rpc as any).mockResolvedValue({ ...mockElements[0], name: 'Updated' })

    const res = await store.updatePanelElement(1, { name: 'Updated' } as any)


    expect(mainStoreMock.addNotification).toHaveBeenCalled()
  })

  it('deletePanelElement removes and notifies', async () => {
    store.setEntities([mockElements[0]])
      ; (rpc as any).mockResolvedValue(undefined)

    await store.deletePanelElement(1)

    expect(store.entities[1]).toBeUndefined()
    expect(store.ids).toHaveLength(0)
    expect(mainStoreMock.addNotification).toHaveBeenCalled()
  })

  it('handles RPC error paths', async () => {
    ; (rpc as any).mockRejectedValue(new Error('boom'))
    ; (rpcSearch as any).mockRejectedValue(new Error('boom'))

    await store.getPanelElements(10)
    await store.loadListQuery({ groupId: 10, panelId: 10 })
    await store.createPanelElement({} as any)
    await store.updatePanelElement(1, {})
    await store.deletePanelElement(1)
    await store.search([], { field: 'id', direction: 'asc' } as any)

    expect(mainStoreMock.checkApiError).toHaveBeenCalled()
  })
})
