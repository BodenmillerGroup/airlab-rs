import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

import { useProteinStore } from '@/stores/protein'
import { useMainStore } from '@/stores/main'
import { useGroupStore } from '@/stores/group'
import * as jsonApi from '@/modules/json/api'

vi.mock('@/modules/json/api', () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

vi.mock('@/stores/main', () => ({
  useMainStore: vi.fn(),
}))

vi.mock('@/stores/group', () => ({
  useGroupStore: vi.fn(),
}))

describe('useProteinStore', () => {
  let store: ReturnType<typeof useProteinStore>
  let mainMock: any
  let groupMock: any

  const proteins = [
    { id: 1, name: 'P53', groupId: 10 },
    { id: 2, name: 'CD3', groupId: 10 },
    { id: 3, name: 'CD19', groupId: 20 },
  ] as any[]

  beforeEach(() => {
    setActivePinia(createPinia())

    mainMock = {
      addNotification: vi.fn(),
      checkApiError: vi.fn(),
    }

    groupMock = {
      activeGroupId: 10,
    }

      ; (useMainStore as any).mockReturnValue(mainMock)
      ; (useGroupStore as any).mockReturnValue(groupMock)

    store = useProteinStore()
    vi.clearAllMocks()
  })

  // -------------------
  // State & getters
  // -------------------

  it('sets entities and ids', () => {
    store.setEntities(proteins)

    expect(store.ids).toEqual([1, 2, 3])
    expect(store.entities[1].name).toBe('P53')
  })

  it('computes proteins and nameMap', () => {
    store.setEntities(proteins)

    expect(store.proteins).toHaveLength(3)
    expect(store.nameMap[1]).toBe('P53')
  })

  it('filters proteins by group', () => {
    store.setEntities(proteins)

    const res = store.getGroupProteins(10)

    expect(res).toHaveLength(2)
    expect(res.every(p => p.groupId === 10)).toBe(true)
  })

  // -------------------
  // Mutations
  // -------------------


  it('resets store', () => {
    store.setEntities(proteins)

    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
    expect(store.page).toBe(1)
    expect(store.searchstr).toBe('')
  })

  // -------------------
  // RPC actions
  // -------------------

  it('createProtein inserts and notifies', async () => {
    ; (jsonApi.rpc as any)
      .mockResolvedValueOnce({ id: proteins[0].id })
      .mockResolvedValueOnce({ items: [proteins[0]], total: 1 })

    await store.createProtein({ name: 'P53' } as any)

    expect(store.entities[1]).toBeDefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('updateProtein updates and notifies', async () => {
    store.setEntities([proteins[0]])
      ; (jsonApi.rpc as any)
        .mockResolvedValueOnce(undefined)
        .mockResolvedValueOnce({ items: [{ ...proteins[0], name: 'X' }], total: 1 })

    await store.updateProtein({ id: 1, data: { name: 'X' } as any })

    expect(store.entities[1].name).toBe('X')
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('deleteProtein removes and notifies', async () => {
    store.setEntities([proteins[0]])
      ; (jsonApi.rpc as any).mockResolvedValue(undefined)

    await store.deleteProtein(1)

    expect(store.entities[1]).toBeUndefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('fetchByIds only fetches missing', async () => {
    store.setEntities([proteins[0]])
      ; (jsonApi.rpc as any).mockResolvedValue({ items: [proteins[1]], total: 2 })

    const res = await store.fetchByIds(10, [1, 2])

    expect(jsonApi.rpc).toHaveBeenCalledOnce()
    expect(res).toHaveLength(1)
    expect(store.entities[2]).toBeDefined()
  })

  it('loadListQuery stores visible ids', async () => {
    vi.spyOn(jsonApi, 'rpcSearch').mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any)

    vi.spyOn(jsonApi, 'rpc').mockResolvedValue({
      items: [proteins[0], proteins[1]],
      total: 2,
    } as any)

    const result = await store.loadListQuery({
      groupId: 10,
      filters: [],
      order: { table: 'Protein', field: 'id', direction: 'asc' },
    } as any)

    expect(result.map((protein) => protein.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listProteins.map((protein) => protein.id)).toEqual([1, 2])
  })

  it('loadListQuery hydrates explicit ids on later pages without paginating them away', async () => {
    store.page = 2

    vi.spyOn(jsonApi, 'rpcSearch').mockResolvedValue({
      items: [2, 3],
      search_total: 3,
    } as any)

    const rpcSpy = vi.spyOn(jsonApi, 'rpc').mockResolvedValue({
      items: [proteins[1], proteins[2]],
      total: 2,
    } as any)

    const result = await store.loadListQuery({
      groupId: 10,
      filters: [],
      order: { table: 'Protein', field: 'id', direction: 'asc' },
    } as any)

    expect(rpcSpy).toHaveBeenCalledWith(expect.objectContaining({
      filters: [{ field: 'id', op: 'in', value: [2, 3] }],
      limit: 2,
      page: 1,
    }))
    expect(result.map((protein) => protein.id)).toEqual([2, 3])
  })

  it('fetchGroupProteins sets loading and total', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: proteins, total: 3 })

    const promise = store.fetchGroupProteins(10)

    expect(store.loading).toBe(true)

    await promise

    expect(store.loading).toBe(false)
    expect(store.total).toBe(3)
    expect(store.ids).toHaveLength(3)
  })

  it('updatePage triggers fetch using group store', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: proteins, total: 3 })

    await store.updatePage(2)

    expect(store.page).toBe(2)
    expect(jsonApi.rpc).toHaveBeenCalled()
  })

  it('updateSearch resets page and triggers fetch', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: proteins, total: 3 })

    await store.updateSearch('CD')

    expect(store.page).toBe(1)
    expect(store.searchstr).toBe('CD')
    expect(jsonApi.rpc).toHaveBeenCalled()
  })

  it('getProteinClones returns clones', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue([{ id: 1 }, { id: 2 }])

    const res = await store.getProteinClones(1)

    expect(res).toHaveLength(2)
    expect(jsonApi.rpc).toHaveBeenCalledOnce()
  })

  // -------------------
  // Error handling
  // -------------------

  it('handles API errors', async () => {
    ; (jsonApi.rpc as any).mockRejectedValue(new Error('boom'))

    await store.createProtein({} as any)
    await store.updateProtein({ id: 1, data: {} as any })
    await store.deleteProtein(1)
    await store.fetchByIds(1, [1])
    await store.fetchGroupProteins(1)
    await store.getProteinClones(1)

    expect(mainMock.checkApiError).toHaveBeenCalled()
  })
})
