import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

import { useTagStore } from '@/stores/tag'
import { useMainStore } from '@/stores/main'
import * as jsonApi from '@/modules/json/api'

vi.mock('@/modules/json/api', () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

vi.mock('@/stores/main', () => ({
  useMainStore: vi.fn(),
}))

describe('useTagStore', () => {
  let store: ReturnType<typeof useTagStore>
  let mainMock: any

  const tags = [
    { id: 1, name: 'Y89', groupId: 10 },
    { id: 2, name: 'Nd150', groupId: 10 },
    { id: 3, name: 'Sm154', groupId: 20 },
  ] as any[]

  beforeEach(() => {
    setActivePinia(createPinia())

    mainMock = {
      addNotification: vi.fn(),
      checkApiError: vi.fn(),
    }

      ; (useMainStore as any).mockReturnValue(mainMock)

    store = useTagStore()
    vi.clearAllMocks()
  })

  // -------------------
  // State & getters
  // -------------------

  it('sets entities and ids', () => {
    store.setEntities(tags)

    expect(store.ids).toEqual([1, 2, 3])
    expect(store.entities[2].name).toBe('Nd150')
  })

  it('computes tags list and nameMap', () => {
    store.setEntities(tags)

    expect(store.tags).toHaveLength(3)
    expect(store.nameMap[1]).toBe('Y89')
  })

  it('hasTag works', () => {
    store.setEntities(tags)

    expect(store.hasTag(1)).toBe(true)
    expect(store.hasTag(99)).toBe(false)
  })

  // -------------------
  // Mutations
  // -------------------


  it('resets store', () => {
    store.setEntities(tags)

    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
    expect(store.page).toBe(1)
  })

  // -------------------
  // RPC actions
  // -------------------

  it('createTag inserts and notifies', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue(tags[0])

    const res = await store.createTag({ name: 'Y89' } as any)

    expect(store.entities[1]).toBeDefined()
    expect(res.id).toBe(1)
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('getTagById fetches and stores', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue(tags[1])

    const res = await store.getTagById(2)

    expect(store.entities[2].name).toBe('Nd150')
    expect(res.id).toBe(2)
  })

  it('updateTag updates and notifies', async () => {
    store.setEntities([tags[0]])
      ; (jsonApi.rpc as any).mockResolvedValue({ ...tags[0], name: 'Y89-new' })

    const res = await store.updateTag({ id: 1, data: { name: 'Y89-new' } as any })

    expect(store.entities[1].name).toBe('Y89-new')
    expect(res.name).toBe('Y89-new')
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('fetchByIds only fetches missing', async () => {
    store.setEntities([tags[0]])
      ; (jsonApi.rpc as any).mockResolvedValue({ items: [tags[1]], total: 2 })

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
      items: [tags[0], tags[1]],
      total: 2,
    } as any)

    const result = await store.loadListQuery({
      groupId: 10,
      filters: [],
      order: { table: 'Tag', field: 'id', direction: 'asc' },
    } as any)

    expect(result.map((tag) => tag.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listTags.map((tag) => tag.id)).toEqual([1, 2])
  })

  it('loadListQuery hydrates explicit ids on later pages without paginating them away', async () => {
    store.page = 2

    vi.spyOn(jsonApi, 'rpcSearch').mockResolvedValue({
      items: [2, 3],
      search_total: 3,
    } as any)

    const rpcSpy = vi.spyOn(jsonApi, 'rpc').mockResolvedValue({
      items: [tags[1], tags[2]],
      total: 2,
    } as any)

    const result = await store.loadListQuery({
      groupId: 10,
      filters: [],
      order: { table: 'Tag', field: 'id', direction: 'asc' },
    } as any)

    expect(rpcSpy).toHaveBeenCalledWith(expect.objectContaining({
      filters: [{ field: 'id', op: 'in', value: [2, 3] }],
      limit: 2,
      page: 1,
    }))
    expect(result.map((tag) => tag.id)).toEqual([2, 3])
  })

  it('getGroupTags sets loading, entities and total', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: tags, total: 3 })

    const p = store.getGroupTags(10)

    expect(store.loading).toBe(true)
    await p

    expect(store.loading).toBe(false)
    expect(store.total).toBe(3)
    expect(store.tags).toHaveLength(3)
  })

  // -------------------
  // UI helpers
  // -------------------


  it('setPage and setLimit update state', () => {
    store.setPage(7)
    store.setLimit(123)

    expect(store.page).toBe(7)
    expect(store.limit).toBe(123)
  })

  // -------------------
  // Error handling
  // -------------------

  it('handles API errors', async () => {
    ; (jsonApi.rpc as any).mockRejectedValue(new Error('boom'))

    await store.createTag({} as any)
    await store.getTagById(1)
    await store.updateTag({ id: 1, data: {} as any })
    await store.fetchByIds(1, [1])
    await store.getGroupTags(1)

    expect(mainMock.checkApiError).toHaveBeenCalled()
  })
})
