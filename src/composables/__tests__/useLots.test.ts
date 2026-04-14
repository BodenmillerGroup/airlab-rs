import { describe, it, expect, vi, beforeEach } from "vitest"

import { useLots } from "@/composables/useLots"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useLotStore } from "@/stores/lot"
import { useMainStore } from "@/stores/main"

describe("useLots", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads lots and builds LotView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const lotStore = useLotStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(lotStore, "page", "get").mockReturnValue(1)
    vi.spyOn(lotStore, "listLots", "get").mockReturnValue([
      {
        id: 1,
        groupId: 10,
        createdBy: 1,
        cloneId: 2,
        providerId: 3,
        storageId: 4,
        collectionId: null,
        name: "Lot A",
        reference: "",
        requestedBy: 0,
        approvedBy: 0,
        orderedBy: 0,
        receivedBy: 0,
        finishedBy: 0,
        number: "L-1",
        status: 2,
        purpose: "",
        url: "",
        price: "",
        note: "",
        requestedAt: "",
        approvedAt: "",
        orderedAt: "",
        receivedAt: "",
        finishedAt: "",
        isArchived: false,
        meta: {},
        createdAt: "now",
        updatedAt: "now",
      },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(lotStore, "loadListQuery").mockResolvedValue([
      { id: 1 },
    ] as any)
    vi.spyOn(lotStore, "reloadListQuery").mockResolvedValue([] as any)

    const lots = useLots()
    await lots.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(lots.loading.value).toBe(false)
    expect(lots.items.value).toEqual([
      expect.objectContaining({
        id: 1,
        groupId: 10,
        name: "Lot A",
        number: "L-1",
        storageId: 4,
        status: 2,
      }),
    ])
  })
})
