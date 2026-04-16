import { describe, it, expect, vi, beforeEach } from "vitest"
import { useConjugates } from "../useConjugates"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useConjugateStore } from "@/stores/conjugate"
import { useLotStore } from "@/stores/lot"
import { useCollectionStore } from "@/stores/collection"
import { useCloneStore } from "@/stores/clone"
import { useProteinStore } from "@/stores/protein"
import { useUserStore } from "@/stores/user"
import { useMemberStore } from "@/stores/member"
import { useTagStore } from "@/stores/tag"
import { useValidationStore } from "@/stores/validation"
import { useMainStore } from "@/stores/main"

describe("useConjugates", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads conjugates and builds ConjugateView list", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const conjugateStore = useConjugateStore()
    const lotStore = useLotStore()
    const collectionStore = useCollectionStore()
    const cloneStore = useCloneStore()
    const proteinStore = useProteinStore()
    const userStore = useUserStore()
    const memberStore = useMemberStore()
    const tagStore = useTagStore()
    const validationStore = useValidationStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(conjugateStore, "page", "get").mockReturnValue(1)
    vi.spyOn(conjugateStore, "listConjugates", "get").mockReturnValue([
      {
        id: 7,
        groupId: 10,
        createdBy: 1,
        labeledBy: 1,
        finishedBy: 0,
        lotId: 3,
        tagId: 5,
        storageId: null,
        status: 2,
        tubeNumber: "T-12",
        concentration: 1,
        description: null,
        isArchived: false,
        meta: {},
        labeledAt: "now",
        createdAt: "now",
        updatedAt: "now",
        customId: "c-7",
      },
    ] as any)

    vi.spyOn(validationStore, "cloneValidationMap", "get").mockReturnValue({
      1: [{ id: 900, application: "WB", status: "approved" }],
    } as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(conjugateStore, "loadListQuery").mockResolvedValue([
      { id: 7 },
    ] as any)
    vi.spyOn(conjugateStore, "reloadListQuery").mockResolvedValue([] as any)

    vi.spyOn(lotStore, "getLot").mockReturnValue({ id: 3, cloneId: 1, collectionId: 12, number: "L-1", status: 6 } as any)
    vi.spyOn(collectionStore, "getCollection").mockReturnValue({ id: 12, name: "Collection A" } as any)
    vi.spyOn(memberStore, "getMemberById").mockReturnValue({ id: 1, userId: 2 } as any)
    vi.spyOn(userStore, "getUserById").mockReturnValue({ id: 2, name: "Test User" } as any)
    vi.spyOn(cloneStore, "getClone").mockReturnValue({ id: 1, proteinId: 11, name: "Clone-1" } as any)
    vi.spyOn(proteinStore, "getProtein").mockReturnValue({ id: 11, name: "Protein-1" } as any)
    vi.spyOn(tagStore, "getTag").mockReturnValue({ id: 5, name: "Tag", mw: 42 } as any)

    const conjugates = useConjugates()
    await conjugates.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(conjugates.loading.value).toBe(false)
    expect(conjugates.items.value).toEqual([
      expect.objectContaining({
        id: 7,
        groupId: 10,
        tagName: "Tag",
        lotName: "L-1",
        lotCollectionId: 12,
        lotCollectionName: "Collection A",
        cloneName: "Clone-1",
        proteinName: "Protein-1",
        userName: "Test User",
        status: 2,
        validations: [expect.objectContaining({ id: 900 })],
      }),
    ])
  })

  it("passes the tube number filter into the search query", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const conjugateStore = useConjugateStore()
    const validationStore = useValidationStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(conjugateStore, "page", "get").mockReturnValue(1)
    vi.spyOn(conjugateStore, "limit", "get").mockReturnValue(25)
    vi.spyOn(conjugateStore, "listConjugates", "get").mockReturnValue([] as any)
    vi.spyOn(validationStore, "cloneValidationMap", "get").mockReturnValue({} as any)

    filterStore.filters = {
      ...filterStore.filters,
      tubeNumber: 42,
    } as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(conjugateStore, "loadListQuery").mockResolvedValue([] as any)
    vi.spyOn(conjugateStore, "reloadListQuery").mockResolvedValue([] as any)

    useConjugates()

    expect(loadListSpy).toHaveBeenCalled()
    expect(loadListSpy).toHaveBeenLastCalledWith(expect.objectContaining({
      groupId: 10,
      filters: expect.arrayContaining([
        expect.objectContaining({
          table: "Conjugate",
          field: "group_id",
          op: "eq",
          value: 10,
        }),
        expect.objectContaining({
          table: "Conjugate",
          field: "tube_number",
          op: "eq",
          value: 42,
        }),
      ]),
    }))
  })

  it("passes the lot collection filter into the search query", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const conjugateStore = useConjugateStore()
    const validationStore = useValidationStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(conjugateStore, "page", "get").mockReturnValue(1)
    vi.spyOn(conjugateStore, "limit", "get").mockReturnValue(25)
    vi.spyOn(conjugateStore, "listConjugates", "get").mockReturnValue([] as any)
    vi.spyOn(validationStore, "cloneValidationMap", "get").mockReturnValue({} as any)

    filterStore.filters = {
      ...filterStore.filters,
      lotCollectionName: "Archive",
    } as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(conjugateStore, "loadListQuery").mockResolvedValue([] as any)
    vi.spyOn(conjugateStore, "reloadListQuery").mockResolvedValue([] as any)

    useConjugates()

    expect(loadListSpy).toHaveBeenCalled()
    expect(loadListSpy).toHaveBeenLastCalledWith(expect.objectContaining({
      groupId: 10,
      filters: expect.arrayContaining([
        expect.objectContaining({
          table: "Conjugate",
          field: "group_id",
          op: "eq",
          value: 10,
        }),
        expect.objectContaining({
          table: "Collection",
          field: "name",
          op: "contains",
          value: "Archive",
        }),
      ]),
    }))
  })
})
