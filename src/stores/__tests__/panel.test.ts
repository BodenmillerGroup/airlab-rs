import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

import { usePanelStore } from '@/stores/panel'
import { useMainStore } from '@/stores/main'
import { useUserStore } from '@/stores/user'
import { useMemberStore } from '@/stores/member'

vi.mock('@/modules/json/api', () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

import * as jsonApi from '@/modules/json/api'

vi.mock('@/stores/main', () => ({
  useMainStore: vi.fn(),
}))

describe('usePanelStore', () => {
  let store: ReturnType<typeof usePanelStore>
  let mainMock: any

  const mockPanels = [
    { id: 1, name: 'P1', groupId: 10, createdBy: 31 },
    { id: 2, name: 'P2', groupId: 10, createdBy: 32 },
    { id: 3, name: 'P3', groupId: 20, createdBy: 33 },
  ] as any[]

  beforeEach(() => {
    setActivePinia(createPinia())

    mainMock = {
      addNotification: vi.fn(),
      checkApiError: vi.fn(),
    }

    ;(useMainStore as any).mockReturnValue(mainMock)

    store = usePanelStore()
    vi.clearAllMocks()
  })

  it('sets entities and ids', () => {
    store.setEntities(mockPanels)

    expect(store.ids).toHaveLength(3)
    expect(store.entities[1].name).toBe('P1')
  })

  it('adds entity', () => {
    store.addEntity(mockPanels[0])
    expect(store.ids[0]).toBe(1)
  })

  it('sets active panel tag', () => {
    store.setActivePanelTag(5)
    expect(store.activePanelTagId).toBe(5)

    store.setActivePanelTag(null)
    expect(store.activePanelTagId).toBe(null)
  })

  it('resets state', () => {
    store.setEntities(mockPanels)
    store.setActivePanelTag(99)

    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
    expect(store.activePanelTagId).toBe(null)
    expect(store.total).toBe(0)
  })

  it('computes panels and hasPanel', () => {
    store.setEntities(mockPanels)

    expect(store.panels).toHaveLength(3)
    expect(store.hasPanel(2)).toBe(true)
    expect(store.hasPanel(99)).toBe(false)
  })

  it('loadListQuery stores visible ids and ensures related data in dependent stores', async () => {
    vi.spyOn(jsonApi, 'rpcSearch').mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any)

    vi.spyOn(jsonApi, 'rpc').mockResolvedValue({
      items: [
        { id: 1, name: 'P1', groupId: 7, createdBy: 31 },
        { id: 2, name: 'P2', groupId: 7, createdBy: 32 },
      ],
    } as any)

    const userStore = useUserStore()
    const memberStore = useMemberStore()

    vi.spyOn(memberStore, 'fetchByIds').mockResolvedValue([] as any)
    vi.spyOn(memberStore, 'getMemberById').mockImplementation((id: number) =>
      id === 31 ? ({ id: 31, userId: 41 } as any) : ({ id: 32, userId: 42 } as any)
    )
    const ensureUsersSpy = vi.spyOn(userStore, 'fetchByIds').mockResolvedValue([] as any)

    const result = await store.loadListQuery({
      groupId: 7,
      filters: [],
      order: { table: 'Panel', field: 'id', direction: 'asc' },
    } as any)

    expect(result.map((panel) => panel.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listPanels.map((panel) => panel.id)).toEqual([1, 2])
    expect(ensureUsersSpy).toHaveBeenCalledWith([41, 42])
  })

  it('createPanel inserts and notifies', async () => {
    ;(jsonApi.rpc as any)
      .mockResolvedValueOnce({ id: 1 })
      .mockResolvedValueOnce({ items: [mockPanels[0]], total: 1 })

    const res = await store.createPanel({ name: 'P1' } as any)

    expect(store.entities[1]).toBeDefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
    expect(res.id).toBe(1)
  })

  it('duplicatePanel inserts and notifies', async () => {
    ;(jsonApi.rpc as any).mockResolvedValue(mockPanels[1])

    const res = await store.duplicatePanel({ id: 1, data: {} as any })

    expect(store.entities[2]).toBeDefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
    expect(res.id).toBe(2)
  })

  it('getPanelById fetches and sets loading', async () => {
    ;(jsonApi.rpc as any).mockResolvedValue({ items: [mockPanels[0]], total: 1 })

    const promise = store.getPanelById(1)

    expect(store.loading).toBe(true)

    const res = await promise

    expect(store.loading).toBe(false)
    expect(store.entities[1]).toBeDefined()
    expect(res.id).toBe(1)
  })

  it('updatePanel updates and notifies', async () => {
    store.setEntities([mockPanels[0]])
    ;(jsonApi.rpc as any)
      .mockResolvedValueOnce(undefined)
      .mockResolvedValueOnce({ items: [{ ...mockPanels[0], name: 'X' }], total: 1 })

    const res = await store.updatePanel({ id: 1, data: { name: 'X' } as any })

    expect(store.entities[1].name).toBe('X')
    expect(mainMock.addNotification).toHaveBeenCalled()
    expect(res.name).toBe('X')
  })

  it('updatePanelArchiveState updates and notifies', async () => {
    store.setEntities([mockPanels[0]])
    ;(jsonApi.rpc as any).mockResolvedValue(mockPanels[0])

    await store.updatePanelArchiveState({ id: 1, data: { state: true } as any })

    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('deletePanel removes and notifies', async () => {
    store.setEntities([mockPanels[0]])
    ;(jsonApi.rpc as any).mockResolvedValue(undefined)

    await store.deletePanel(1)

    expect(store.entities[1]).toBeUndefined()
    expect(store.ids).toHaveLength(0)
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('getGroupPanels fetches, sets total and loading', async () => {
    ;(jsonApi.rpc as any).mockResolvedValue({ items: mockPanels, total: 3 })

    const promise = store.getGroupPanels(10)

    expect(store.loading).toBe(true)

    const res = await promise

    expect(store.loading).toBe(false)
    expect(store.total).toBe(3)
    expect(store.ids).toHaveLength(3)
    expect(res).toHaveLength(3)
  })

  it('getPanelElements returns elements', async () => {
    ;(jsonApi.rpc as any).mockResolvedValue([{ id: 1 }, { id: 2 }])

    const res = await store.getPanelElements(1)

    expect(jsonApi.rpc).toHaveBeenCalledOnce()
    expect(res).toHaveLength(2)
  })

  it('handles API errors', async () => {
    ;(jsonApi.rpc as any).mockRejectedValue(new Error('boom'))

    await store.createPanel({} as any)
    await store.duplicatePanel({ id: 1, data: {} as any })
    await store.getPanelById(1)
    await store.updatePanel({ id: 1, data: {} as any })
    await store.updatePanelArchiveState({ id: 1, data: { state: true } as any })
    await store.deletePanel(1)
    await store.getGroupPanels(1)
    await store.getPanelElements(1)

    expect(mainMock.checkApiError).toHaveBeenCalled()
  })
})
