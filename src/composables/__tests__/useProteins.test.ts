import { describe, it, expect, vi, beforeEach } from "vitest"

import { useProteins } from "@/composables/useProteins"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useProteinStore } from "@/stores/protein"
import { useMainStore } from "@/stores/main"

describe("useProteins", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads proteins and builds ProteinView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const proteinStore = useProteinStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(proteinStore, "page", "get").mockReturnValue(1)
    vi.spyOn(proteinStore, "listProteins", "get").mockReturnValue([
      {
        id: 4,
        groupId: 10,
        createdBy: 1,
        name: "Protein X",
        description: "Test protein",
        meta: {},
        createdAt: "now",
      },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(proteinStore, "loadListQuery").mockResolvedValue([
      { id: 4 },
    ] as any)
    vi.spyOn(proteinStore, "reloadListQuery").mockResolvedValue([] as any)

    const proteins = useProteins()
    await proteins.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(proteins.loading.value).toBe(false)
    expect(proteins.items.value).toEqual([
      expect.objectContaining({
        id: 4,
        groupId: 10,
        name: "Protein X",
        description: "Test protein",
      }),
    ])
  })
})
