import { describe, it, expect, vi, beforeEach } from "vitest"
import { useClones } from "../useClones"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useCloneStore } from "@/stores/clone"
import { useConjugateStore } from "@/stores/conjugate"
import { useLotStore } from "@/stores/lot"
import { useProteinStore } from "@/stores/protein"
import { useSpeciesStore } from "@/stores/species"
import { useValidationStore } from "@/stores/validation"
import { useMainStore } from "@/stores/main"
import { nextTick } from "vue"

describe("useClones", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads clones and builds CloneView list", async () => {
    // --------------------
    // Arrange (STORES FIRST)
    // --------------------
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const cloneStore = useCloneStore()
    const proteinStore = useProteinStore()
    const speciesStore = useSpeciesStore()
    const validationStore = useValidationStore()
    const mainStore = useMainStore()

    // --- getters / computed ---
    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(1)
    vi.spyOn(cloneStore, "page", "get").mockReturnValue(1)
    vi.spyOn(cloneStore, "listClones", "get").mockReturnValue([
      {
        id: 10,
        name: "Clone A",
        groupId: 1,
        proteinId: 100,
        speciesId: 200,
        isPolyclonal: false,
        isPhospho: true,
        createdBy: "lars",
        isotype: "IgG",
        epitope: "X",
        reactivity: "human",
        application: "WB",
        isArchived: false,
        createdAt: "2024-01-01",
        updatedAt: "2024-01-02",
      },
    ] as any)
    vi.spyOn(proteinStore, "revision", "get").mockReturnValue(0)
    vi.spyOn(speciesStore, "revision", "get").mockReturnValue(0)

    vi.spyOn(validationStore, "cloneValidationMap", "get").mockReturnValue({
      10: [{ id: 500, application: "WB", status: "approved" }],
    } as any)

    // --- state ---
    filterStore.filters = {} as any

    // --- actions ---
    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()

    const loadListSpy = vi.spyOn(cloneStore, "loadListQuery").mockResolvedValue([
      {
        id: 10,
        name: "Clone A",
        groupId: 1,
        proteinId: 100,
        speciesId: 200,
        isPolyclonal: false,
        isPhospho: true,
        createdBy: "lars",
        isotype: "IgG",
        epitope: "X",
        reactivity: "human",
        application: "WB",
        isArchived: false,
        createdAt: "2024-01-01",
        updatedAt: "2024-01-02",
      },
    ] as any)
    vi.spyOn(cloneStore, "reloadListQuery").mockResolvedValue([] as any)

    vi.spyOn(proteinStore, "getProtein").mockReturnValue({ id: 100, name: "TP53" } as any)
    vi.spyOn(speciesStore, "getSpecies").mockReturnValue({ id: 200, name: "Human" } as any)

    // --------------------
    // Act (ONLY AFTER spies exist)
    // --------------------
    const clones = useClones()
    await clones.reload()


    // --------------------
    // Assert
    // --------------------
    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)

    expect(clones.loading.value).toBe(false)

    expect(clones.items.value).toEqual([
      expect.objectContaining({
        id: 10,
        name: "Clone A",
        proteinName: "TP53",
        speciesName: "Human",
        validations: [
          expect.objectContaining({ id: 500 }),
        ],
      }),
    ])
  })

  it("resolves clone filter from conjugate via lot", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const cloneStore = useCloneStore()
    const conjugateStore = useConjugateStore()
    const lotStore = useLotStore()
    const proteinStore = useProteinStore()
    const speciesStore = useSpeciesStore()
    const validationStore = useValidationStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(1)
    vi.spyOn(cloneStore, "page", "get").mockReturnValue(1)
    vi.spyOn(proteinStore, "revision", "get").mockReturnValue(0)
    vi.spyOn(speciesStore, "revision", "get").mockReturnValue(0)
    vi.spyOn(validationStore, "cloneValidationMap", "get").mockReturnValue({} as any)
    vi.spyOn(cloneStore, "listClones", "get").mockReturnValue([] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    vi.spyOn(conjugateStore, "getConjugate").mockReturnValue(undefined)
    const fetchConjugatesSpy = vi.spyOn(conjugateStore, "fetchByIds").mockResolvedValue([
      { id: 55, lotId: 8 },
    ] as any)
    const fetchLotsSpy = vi.spyOn(lotStore, "fetchByIds").mockResolvedValue([
      { id: 8, cloneId: 10 },
    ] as any)
    const loadListSpy = vi.spyOn(cloneStore, "loadListQuery").mockResolvedValue([] as any)
    vi.spyOn(cloneStore, "reloadListQuery").mockResolvedValue([] as any)

    useClones({ conjugateId: 55 })
    await nextTick()
    await Promise.resolve()

    expect(fetchConjugatesSpy).toHaveBeenCalledWith(1, [55])
    expect(fetchLotsSpy).toHaveBeenCalledWith(1, [8], true)
    expect(loadListSpy).toHaveBeenCalledWith(expect.objectContaining({
      groupId: 1,
      filters: expect.arrayContaining([
        expect.objectContaining({
          table: "Clone",
          field: "group_id",
          op: "eq",
          value: 1,
        }),
        expect.objectContaining({
          table: "Clone",
          field: "id",
          op: "eq",
          value: 10,
        }),
      ]),
    }))
  })
})
