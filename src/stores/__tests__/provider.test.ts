import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

import { useProviderStore } from '@/stores/provider'
import { useMainStore } from '@/stores/main'
import * as jsonApi from '@/modules/json/api'

vi.mock('@/modules/json/api', () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

vi.mock('@/stores/main', () => ({
  useMainStore: vi.fn(),
}))

describe('useProviderStore', () => {
  let store: ReturnType<typeof useProviderStore>
  let mainMock: any

  const providers = [
    { id: 1, name: 'Sigma', groupId: 10 },
    { id: 2, name: 'Abcam', groupId: 10 },
    { id: 3, name: 'Thermo', groupId: 20 },
  ] as any[]

  beforeEach(() => {
    setActivePinia(createPinia())

    mainMock = {
      addNotification: vi.fn(),
      checkApiError: vi.fn(),
    }

      ; (useMainStore as any).mockReturnValue(mainMock)

    store = useProviderStore()
    vi.clearAllMocks()
  })

  // -------------------
  // State & getters
  // -------------------

  it('sets entities and ids', () => {
    store.setEntities(providers)

    expect(store.ids).toEqual([1, 2, 3])
    expect(store.entities[2].name).toBe('Abcam')
  })

  it('computes providers and nameMap', () => {
    store.setEntities(providers)

    expect(store.providers).toHaveLength(3)
    expect(store.nameMap[1]).toBe('Sigma')
  })

  it('hasProvider works', () => {
    store.setEntities(providers)

    expect(store.hasProvider(1)).toBe(true)
    expect(store.hasProvider(99)).toBe(false)
  })

  // -------------------
  // Mutations
  // -------------------


  it('resets store', () => {
    store.setEntities(providers)

    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
    expect(store.page).toBe(1)
  })

  // -------------------
  // RPC actions
  // -------------------

  it('createProvider inserts and notifies', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue(providers[0])

    const res = await store.createProvider({ name: 'Sigma' } as any)

    expect(store.entities[1]).toBeDefined()
    expect(res.id).toBe(1)
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('getProviderById fetches and sets loading', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue(providers[1])

    const p = store.getProviderById(2)

    expect(store.loading).toBe(true)
    await p

    expect(store.loading).toBe(false)
    expect(store.entities[2].name).toBe('Abcam')
  })

  it('updateProvider updates and notifies', async () => {
    store.setEntities([providers[0]])
      ; (jsonApi.rpc as any).mockResolvedValue({ ...providers[0], name: 'X' })

    await store.updateProvider({ id: 1, data: { name: 'X' } as any })

    expect(store.entities[1].name).toBe('X')
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('deleteProvider removes and notifies', async () => {
    store.setEntities([providers[0]])
      ; (jsonApi.rpc as any).mockResolvedValue(undefined)

    await store.deleteProvider(1)

    expect(store.entities[1]).toBeUndefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('fetchByIds only fetches missing', async () => {
    store.setEntities([providers[0]])
      ; (jsonApi.rpc as any).mockResolvedValue({ items: [providers[1]], total: 2 })

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
      items: [providers[0], providers[1]],
      total: 2,
    } as any)

    const main = useMainStore()
    vi.spyOn(main, 'checkApiError').mockResolvedValue()

    const result = await store.loadListQuery({
      groupId: 10,
      filters: [],
      order: { table: 'Provider', field: 'id', direction: 'asc' },
    } as any)

    expect(result.map((provider) => provider.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listProviders.map((provider) => provider.id)).toEqual([1, 2])
  })

  it('loadListQuery hydrates explicit ids on later pages without paginating them away', async () => {
    store.page = 2

    vi.spyOn(jsonApi, 'rpcSearch').mockResolvedValue({
      items: [2, 3],
      search_total: 3,
    } as any)

    const rpcSpy = vi.spyOn(jsonApi, 'rpc').mockResolvedValue({
      items: [providers[1], providers[2]],
      total: 2,
    } as any)

    const result = await store.loadListQuery({
      groupId: 10,
      filters: [],
      order: { table: 'Provider', field: 'id', direction: 'asc' },
    } as any)

    expect(rpcSpy).toHaveBeenCalledWith(expect.objectContaining({
      filters: [{ field: 'id', op: 'in', value: [2, 3] }],
      limit: 2,
      page: 1,
    }))
    expect(result.map((provider) => provider.id)).toEqual([2, 3])
  })

  it('getGroupProviders sets entities and total', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: providers, total: 3 })

    const p = store.getGroupProviders(10)

    expect(store.loading).toBe(true)
    await p

    expect(store.loading).toBe(false)
    expect(store.total).toBe(3)
    expect(store.providers).toHaveLength(3)
  })

  it('getProviderLots returns lots', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue([{ id: 1 }, { id: 2 }])

    const res = await store.getProviderLots(1)

    expect(res).toHaveLength(2)
    expect(jsonApi.rpc).toHaveBeenCalledOnce()
  })

  // -------------------
  // UI helpers
  // -------------------


  it('setPage and setLimit update state', () => {
    store.setPage(5)
    store.setLimit(99)

    expect(store.page).toBe(5)
    expect(store.limit).toBe(99)
  })

  // -------------------
  // Error handling
  // -------------------

  it('handles API errors', async () => {
    ; (jsonApi.rpc as any).mockRejectedValue(new Error('boom'))

    await store.createProvider({} as any)
    await store.getProviderById(1)
    await store.updateProvider({ id: 1, data: {} as any })
    await store.deleteProvider(1)
    await store.fetchByIds(1, [1])
    await store.getGroupProviders(1)
    await store.getProviderLots(1)

    expect(mainMock.checkApiError).toHaveBeenCalled()
  })
})
