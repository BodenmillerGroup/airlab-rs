import { describe, it, expect, vi, beforeEach } from "vitest"

import { useSpecies } from "@/composables/useSpecies"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useSpeciesStore } from "@/stores/species"
import { useMainStore } from "@/stores/main"

describe("useSpecies", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads species and builds SpeciesView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const speciesStore = useSpeciesStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(speciesStore, "page", "get").mockReturnValue(1)
    vi.spyOn(speciesStore, "listSpecies", "get").mockReturnValue([
      {
        id: 8,
        groupId: 10,
        name: "Human",
        acronym: "Hs",
        meta: {},
        createdAt: "now",
      },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(speciesStore, "loadListQuery").mockResolvedValue([
      { id: 8 },
    ] as any)
    vi.spyOn(speciesStore, "reloadListQuery").mockResolvedValue([] as any)

    const species = useSpecies()
    await species.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(species.loading.value).toBe(false)
    expect(species.items.value).toEqual([
      expect.objectContaining({
        id: 8,
        groupId: 10,
        name: "Human",
        acronym: "Hs",
      }),
    ])
  })
})
