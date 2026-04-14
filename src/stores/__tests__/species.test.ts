import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

import { useSpeciesStore } from '@/stores/species'
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

describe('useSpeciesStore', () => {
  let store: ReturnType<typeof useSpeciesStore>
  let mainMock: any
  let groupMock: any

  const species = [
    { id: 1, name: 'Human', groupId: 10 },
    { id: 2, name: 'Mouse', groupId: 10 },
    { id: 3, name: 'Rat', groupId: 20 },
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

    store = useSpeciesStore()
    vi.clearAllMocks()
  })

  // -------------------
  // State & getters
  // -------------------

  it('sets entities and ids', () => {
    store.setEntities(species)

    expect(store.ids).toEqual([1, 2, 3])
    expect(store.entities[2].name).toBe('Mouse')
  })

  it('computes species list and nameMap', () => {
    store.setEntities(species)

    expect(store.species).toHaveLength(3)
    expect(store.nameMap[1]).toBe('Human')
  })

  it('hasSpecies works', () => {
    store.setEntities(species)

    expect(store.hasSpecies(1)).toBe(true)
    expect(store.hasSpecies(99)).toBe(false)
  })

  // -------------------
  // Mutations
  // -------------------


  it('resets store', () => {
    store.setEntities(species)

    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
    expect(store.page).toBe(1)
  })

  // -------------------
  // RPC actions
  // -------------------

  it('createSpecies inserts and notifies', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue(species[0])

    await store.createSpecies({ name: 'Human' } as any)

    expect(store.entities[1]).toBeDefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('getSpeciesById fetches and stores', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue(species[1])

    await store.getSpeciesById(2)

    expect(store.entities[2].name).toBe('Mouse')
  })

  it('updateSpecies updates and notifies', async () => {
    store.setEntities([species[0]])
      ; (jsonApi.rpc as any).mockResolvedValue({ ...species[0], name: 'Human v2' })

    await store.updateSpecies({ id: 1, data: { name: 'Human v2' } as any })

    expect(store.entities[1].name).toBe('Human v2')
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('deleteSpecies removes and notifies', async () => {
    store.setEntities([species[0]])
      ; (jsonApi.rpc as any).mockResolvedValue(undefined)

    await store.deleteSpecies(1)

    expect(store.entities[1]).toBeUndefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('fetchByIds only fetches missing', async () => {
    store.setEntities([species[0]])
      ; (jsonApi.rpc as any).mockResolvedValue({ items: [species[1]], total: 2 })

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
      items: [species[0], species[1]],
      total: 2,
    } as any)

    const result = await store.loadListQuery({
      groupId: 10,
      filters: [],
      order: { table: 'Species', field: 'id', direction: 'asc' },
    } as any)

    expect(result.map((item) => item.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listSpecies.map((item) => item.id)).toEqual([1, 2])
  })

  it('getGroupSpecies sets loading, entities and total', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: species, total: 3 })

    const p = store.getGroupSpecies(10)

    expect(store.loading).toBe(true)
    await p

    expect(store.loading).toBe(false)
    expect(store.total).toBe(3)
    expect(store.species).toHaveLength(3)
  })

  it('updatePage triggers refetch', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: species, total: 3 })

    await store.updatePage(2)

    expect(store.page).toBe(2)
    expect(jsonApi.rpc).toHaveBeenCalled()
  })

  it('updateSearch resets page and refetches', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: species, total: 3 })

    await store.updateSearch('Hu')

    expect(store.page).toBe(1)
    expect(jsonApi.rpc).toHaveBeenCalled()
  })

  it('getSpeciesClones returns clones', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue([{ id: 1 }, { id: 2 }])

    const res = await store.getSpeciesClones(1)

    expect(res).toHaveLength(2)
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

    await store.createSpecies({} as any)
    await store.getSpeciesById(1)
    await store.updateSpecies({ id: 1, data: {} as any })
    await store.deleteSpecies(1)
    await store.fetchByIds(1, [1])
    await store.getGroupSpecies(1)
    await store.getSpeciesClones(1)

    expect(mainMock.checkApiError).toHaveBeenCalled()
  })
})
