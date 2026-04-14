import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

import { useValidationFileStore } from '@/stores/validation_file'
import { useMainStore } from '@/stores/main'
import * as jsonApi from '@/modules/json/api'

vi.mock('@/modules/json/api', () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

vi.mock('@/stores/main', () => ({
  useMainStore: vi.fn(),
}))

describe('useValidationFileStore', () => {
  let store: ReturnType<typeof useValidationFileStore>
  let mainMock: any

  const files = [
    { id: 1, name: 'file1.pdf', validationId: 10 },
    { id: 2, name: 'file2.csv', validationId: 10 },
    { id: 3, name: 'file3.png', validationId: 20 },
  ] as any[]

  beforeEach(() => {
    setActivePinia(createPinia())

    mainMock = {
      addNotification: vi.fn(),
      checkApiError: vi.fn(),
    }

      ; (useMainStore as any).mockReturnValue(mainMock)

    store = useValidationFileStore()
    vi.clearAllMocks()
  })

  // --------------------
  // State & getters
  // --------------------

  it('sets entities and ids', () => {
    store.setEntities(files)

    expect(store.ids).toEqual([1, 2, 3])
    expect(store.entities[2].name).toBe('file2.csv')
  })

  it('computes files list', () => {
    store.setEntities(files)

    expect(store.files).toHaveLength(3)
    expect(store.files[0].id).toBe(1)
  })

  it('hasFile works', () => {
    store.setEntities(files)

    expect(store.hasFile(1)).toBe(true)
    expect(store.hasFile(99)).toBe(false)
  })

  it('groups by validation id', () => {
    store.setEntities(files)

    expect(store.byValidationId[10]).toHaveLength(2)
    expect(store.byValidationId[20][0].id).toBe(3)
  })

  // --------------------
  // Mutations
  // --------------------

  it('adds entity', () => {
    store.addEntity(files[0])

    expect(store.ids).toEqual([1])
    expect(store.entities[1]).toBeDefined()
    expect(store.total).toBe(1)
  })

  it('deletes entity', () => {
    store.setEntities(files)

    store.deleteEntity(2)

    expect(store.entities[2]).toBeUndefined()
    expect(store.ids).toEqual([1, 3])
    expect(store.total).toBe(2)
  })

  it('resets store', () => {
    store.setEntities(files)

    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
    expect(store.page).toBe(1)
    expect(store.loading).toBe(false)
  })

  // --------------------
  // RPC actions
  // --------------------

  it('getValidationFiles fetches and stores', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: files, total: 3 })

    const p = store.getValidationFiles(10)
    expect(store.loading).toBe(true)

    const res = await p

    expect(store.loading).toBe(false)
    expect(res).toHaveLength(3)
    expect(store.files).toHaveLength(3)
    expect(store.total).toBe(3)
  })

  it('loadListQuery stores visible ids', async () => {
    vi.spyOn(jsonApi, 'rpcSearch').mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any)

    vi.spyOn(jsonApi, 'rpc').mockResolvedValue({
      items: [files[0], files[1]],
    } as any)

    const result = await store.loadListQuery({
      groupId: 10,
      filters: [],
      order: { table: 'ValidationFile', field: 'id', direction: 'asc' },
    } as any)

    expect(result.map((file) => file.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listFiles.map((file) => file.id)).toEqual([1, 2])
  })

  it('deleteValidationFile removes and notifies', async () => {
    store.setEntities(files)
      ; (jsonApi.rpc as any).mockResolvedValue(undefined)

    await store.deleteValidationFile(1)

    expect(store.entities[1]).toBeUndefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  // --------------------
  // UI helpers
  // --------------------

  it('setPage and setLimit work', () => {
    store.setPage(7)
    store.setLimit(123)

    expect(store.page).toBe(7)
    expect(store.limit).toBe(123)
  })

  // --------------------
  // Error handling
  // --------------------

  it('handles API errors', async () => {
    ; (jsonApi.rpc as any).mockRejectedValue(new Error('boom'))

    await store.getValidationFiles(1)
    await store.deleteValidationFile(1)

    expect(mainMock.checkApiError).toHaveBeenCalled()
  })
})
