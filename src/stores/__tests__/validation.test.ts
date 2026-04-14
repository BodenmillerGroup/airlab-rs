import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

import { useValidationStore } from '@/stores/validation'
import { useMainStore } from '@/stores/main'
import { useCloneStore } from '@/stores/clone'
import { useProteinStore } from '@/stores/protein'
import { useConjugateStore } from '@/stores/conjugate'
import { useSpeciesStore } from '@/stores/species'
import { useUserStore } from '@/stores/user'
import { useMemberStore } from '@/stores/member'
import { useLotStore } from '@/stores/lot'

vi.mock('@/modules/json/api', () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

vi.mock('@/modules/validation_file/api', () => ({
  api: {
    uploadValidationFile: vi.fn(),
  },
}))

import * as jsonApi from '@/modules/json/api'
import { api as validationFileApi } from '@/modules/validation_file/api'

vi.mock('@/stores/main', () => ({
  useMainStore: vi.fn(),
}))

describe('useValidationStore', () => {
  let store: ReturnType<typeof useValidationStore>
  let mainMock: any

  const validations = [
    { id: 1, name: 'v1', cloneId: 10, lotId: 100 },
    { id: 2, name: 'v2', cloneId: 10, lotId: 100 },
    { id: 3, name: 'v3', cloneId: 20, lotId: 200 },
  ] as any[]

  beforeEach(() => {
    setActivePinia(createPinia())

    mainMock = {
      addNotification: vi.fn(),
      checkApiError: vi.fn(),
    }

      ; (useMainStore as any).mockReturnValue(mainMock)

    store = useValidationStore()
    vi.clearAllMocks()
  })

  // --------------------
  // State & getters
  // --------------------

  it('sets entities and ids', () => {
    store.setEntities(validations)

    expect(store.ids).toEqual([1, 2, 3])
  })

  it('computes validations list', () => {
    store.setEntities(validations)

    expect(store.validations).toHaveLength(3)
    expect(store.validations[0].id).toBe(1)
  })

  it('hasValidation works', () => {
    store.setEntities(validations)

    expect(store.hasValidation(1)).toBe(true)
    expect(store.hasValidation(999)).toBe(false)
  })

  it('groups by clone id', () => {
    store.setEntities(validations)

    expect(store.cloneValidationMap[10]).toHaveLength(2)
    expect(store.cloneValidationMap[20][0].id).toBe(3)
  })

  it('groups by lot id', () => {
    store.setEntities(validations)

    expect(store.lotValidationMap[100]).toHaveLength(2)
    expect(store.lotValidationMap[200][0].id).toBe(3)
  })

  // --------------------
  // Mutations
  // --------------------

  it('adds entity', () => {
    store.addEntity(validations[0])

    expect(store.ids).toEqual([1])
    expect(store.entities[1]).toBeDefined()
    expect(store.total).toBe(1)
  })

  it('deletes entity', async () => {
    store.setEntities(validations)
      ; (jsonApi.rpc as any).mockResolvedValue(undefined)

    await store.deleteValidation(2)

    expect(store.entities[2]).toBeUndefined()
    expect(store.ids).toEqual([1, 3])
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('resets store', () => {
    store.setEntities(validations)

    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
    expect(store.page).toBe(1)
    expect(store.loading).toBe(false)
  })

  // --------------------
  // RPC actions
  // --------------------

  it('creates validation', async () => {
    ; (jsonApi.rpc as any)
      .mockResolvedValueOnce({ id: 1 })
      .mockResolvedValueOnce({ items: [validations[0]], total: 1 })

    const res = await store.createValidation({} as any)

    expect(res.id).toBe(1)
    expect(store.entities[1]).toBeDefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('fetches by clone ids', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: validations, total: 3 })

    const res = await store.fetchByCloneIds(1, [10, 20])

    expect(res).toHaveLength(3)
    expect(store.total).toBe(3)
  })

  it('uses an elevated single-page limit for clone relation hydration', async () => {
    const relationItems = Array.from({ length: 8 }, (_, index) => ({
      id: index + 1,
      cloneId: 10,
      lotId: 100,
    }))

    ; (jsonApi.rpc as any).mockResolvedValue({ items: relationItems, total: 8 })

    const res = await store.fetchByCloneIds(1, [10, 20])

    expect(jsonApi.rpc).toHaveBeenCalledTimes(1)
    expect((jsonApi.rpc as any).mock.calls[0][0].limit).toBe(200)
    expect((jsonApi.rpc as any).mock.calls[0][0].page).toBe(1)
    expect(res).toHaveLength(8)
    expect(store.total).toBe(8)
    expect(store.entities[8]).toBeDefined()
  })

  it('reuses cached clone validation fetches', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: validations, total: 3 })

    await store.fetchByCloneIds(1, [10, 20])
    await store.fetchByCloneIds(1, [10, 20])

    expect(jsonApi.rpc).toHaveBeenCalledTimes(1)
  })

  it('does not cache clone ids when relation hydration is truncated', async () => {
    const relationItems = Array.from({ length: 200 }, (_, index) => ({
      id: index + 1,
      cloneId: 10,
      lotId: 100,
    }))

    ; (jsonApi.rpc as any).mockResolvedValue({ items: relationItems, total: 250 })

    await store.fetchByCloneIds(1, [10])
    await store.fetchByCloneIds(1, [10])

    expect(jsonApi.rpc).toHaveBeenCalledTimes(2)
  })

  it('loadListQuery stores visible ids and ensures related data in dependent stores', async () => {
    vi.spyOn(jsonApi, 'rpcSearch').mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any)

    vi.spyOn(jsonApi, 'rpc').mockResolvedValue({
      items: [
        { id: 1, groupId: 7, cloneId: 11, conjugateId: 21, speciesId: 31, lotId: 41, createdBy: 51 },
        { id: 2, groupId: 7, cloneId: 12, conjugateId: 22, speciesId: 32, lotId: 42, createdBy: 52 },
      ],
    } as any)

    const cloneStore = useCloneStore()
    const proteinStore = useProteinStore()
    const conjugateStore = useConjugateStore()
    const speciesStore = useSpeciesStore()
    const userStore = useUserStore()
    const memberStore = useMemberStore()
    const lotStore = useLotStore()

    vi.spyOn(cloneStore, 'fetchByIds').mockResolvedValue([] as any)
    vi.spyOn(cloneStore, 'getClone').mockImplementation((id: number) =>
      id === 11 ? ({ id: 11, proteinId: 61 } as any) : ({ id: 12, proteinId: 62 } as any)
    )
    vi.spyOn(conjugateStore, 'fetchByIds').mockResolvedValue([] as any)
    vi.spyOn(speciesStore, 'fetchByIds').mockResolvedValue([] as any)
    vi.spyOn(lotStore, 'fetchByIds').mockResolvedValue([] as any)
    vi.spyOn(memberStore, 'fetchByIds').mockResolvedValue([] as any)
    vi.spyOn(memberStore, 'getMemberById').mockImplementation((id: number) =>
      id === 51 ? ({ id: 51, userId: 71 } as any) : ({ id: 52, userId: 72 } as any)
    )

    const ensureProteinsSpy = vi.spyOn(proteinStore, 'fetchByIds').mockResolvedValue([] as any)
    const ensureUsersSpy = vi.spyOn(userStore, 'fetchByIds').mockResolvedValue([] as any)

    const result = await store.loadListQuery({
      groupId: 7,
      filters: [],
      order: { table: 'Validation', field: 'id', direction: 'asc' },
    } as any)

    expect(result.map((validation) => validation.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listValidations.map((validation) => validation.id)).toEqual([1, 2])
    expect(ensureUsersSpy).toHaveBeenCalledWith([71, 72])
    expect(ensureProteinsSpy).toHaveBeenCalledWith(7, [61, 62])
  })

  it('gets group validations', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue({ items: validations, total: 3 })

    const p = store.getGroupValidations(1)
    expect(store.loading).toBe(true)

    const res = await p

    expect(store.loading).toBe(false)
    expect(res).toHaveLength(3)
  })

  it('updates validation', async () => {
    ; (jsonApi.rpc as any)
      .mockResolvedValueOnce(undefined)
      .mockResolvedValueOnce({ items: [validations[0]], total: 1 })

    const res = await store.updateValidation({ id: 1, data: {} as any })

    expect(store.entities[1]).toBeDefined()
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('archives validation', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue(validations[0])

    await store.updateValidationArchiveState({
      id: 1,
      data: { state: true } as any,
    })

    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  // --------------------
  // File actions
  // --------------------

  it('uploads validation file', async () => {
    ; (validationFileApi.uploadValidationFile as any).mockResolvedValue({
      id: 9,
      validationId: 1,
      createdBy: 2,
      hash: 'abc',
      extension: 'pdf',
    })

    const res = await store.uploadValidationFile({
      validationId: 1,
      formData: new FormData(),
    })

    expect(res).toEqual(expect.objectContaining({
      id: 9,
      validationId: 1,
      extension: 'pdf',
    }))
    expect(validationFileApi.uploadValidationFile).toHaveBeenCalledWith(1, expect.any(FormData))
    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  it('deletes validation file', async () => {
    ; (jsonApi.rpc as any).mockResolvedValue(undefined)

    await store.deleteValidationFile(1)

    expect(mainMock.addNotification).toHaveBeenCalled()
  })

  // --------------------
  // UI helpers
  // --------------------

  it('setSearch resets page', () => {
    store.page = 5

    store.setSearch('abc')

    expect(store.page).toBe(1)
    expect(store.searchstr).toBe('abc')
  })

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

    await store.createValidation({} as any)
    await store.getValidationById(1)
    await store.fetchByCloneIds(1, [1])
    await store.getGroupValidations(1)

    expect(mainMock.checkApiError).toHaveBeenCalled()
  })
})
