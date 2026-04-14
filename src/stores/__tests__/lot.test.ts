import { describe, it, expect, beforeEach, vi } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { useLotStore } from "@/stores/lot"
import { useMainStore } from "@/stores/main"
import { useCloneStore } from "@/stores/clone"
import { useProviderStore } from "@/stores/provider"
import { useCollectionStore } from "@/stores/collection"
import { useValidationStore } from "@/stores/validation"
import { LotStatus } from "@/modules/lot/LotStatus"

/* ---------------- mocks ---------------- */

vi.mock("@/modules/json/api", () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

import * as jsonApi from "@/modules/json/api"

/* ---------------- setup ---------------- */

beforeEach(() => {
  setActivePinia(createPinia())
  vi.clearAllMocks()
})

/* ---------------- state & getters ---------------- */

describe("lot store – state & getters", () => {
  it("sets and retrieves lots", () => {
    const store = useLotStore()

    store.setEntity({ id: 1, name: "L1" } as any)
    store.setEntity({ id: 2, name: "L2" } as any)

    expect(store.lots.length).toBe(2)
    expect(store.getLotById(1)?.name).toBe("L1")
  })
})

/* ---------------- CSV ---------------- */

describe("lot store – CSV export", () => {
  it("builds valid CSV", () => {
    const store = useLotStore()

    const csv = store.getCsv([
      {
        id: 1,
        name: "LotA",
        number: "N1",
        reference: "R1",
        price: 10,
        status: LotStatus.Finished,
        clone: { name: "CloneA" },
        provider: { name: "Prov" },
      },
    ] as any)

    expect(csv).toContain("LotA")
    expect(csv).toContain("CloneA")
    expect(csv).toContain("Prov")
    expect(csv.split("\n").length).toBe(2)
  })
})

/* ---------------- fetchByIds ---------------- */

describe("lot store – fetchByIds", () => {
  it("fetches only missing lots", async () => {
    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [{ id: 2, name: "B" }],
      total: 2,
    } as any)

    const store = useLotStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    store.setEntity({ id: 1, name: "A" } as any)

    const result = await store.fetchByIds(1, [1, 2, 2])

    expect(result.length).toBe(1)
    expect(store.getLotById(2)?.name).toBe("B")
  })
})

/* ---------------- group queries ---------------- */

describe("lot store – group queries", () => {
  it("loadListQuery stores visible ids and ensures related data in dependent stores", async () => {
    vi.spyOn(jsonApi, "rpcSearch").mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any)

    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [
        { id: 1, groupId: 7, cloneId: 11, providerId: 21, name: "Lot A" },
        { id: 2, groupId: 7, cloneId: 12, providerId: 22, name: "Lot B" },
      ],
      total: 2,
    } as any)

    const store = useLotStore()
    const cloneStore = useCloneStore()
    const providerStore = useProviderStore()
    const collectionStore = useCollectionStore()
    const validationStore = useValidationStore()
    const main = useMainStore()

    vi.spyOn(main, "checkApiError").mockResolvedValue()
    const ensureClonesSpy = vi.spyOn(cloneStore, "fetchByIds").mockResolvedValue([] as any)
    const ensureProvidersSpy = vi.spyOn(providerStore, "fetchByIds").mockResolvedValue([] as any)
    const getCollectionsSpy = vi.spyOn(collectionStore, "getCollections").mockResolvedValue([] as any)
    const ensureValidationsSpy = vi.spyOn(validationStore, "fetchByCloneIds").mockResolvedValue([] as any)

    const result = await store.loadListQuery({
      groupId: 7,
      filters: [],
      order: { table: "Lot", field: "id", direction: "asc" },
    } as any)

    expect(result.map((lot) => lot.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listLots.map((lot) => lot.id)).toEqual([1, 2])
    expect(ensureClonesSpy).toHaveBeenCalledWith([11, 12])
    expect(ensureProvidersSpy).toHaveBeenCalledWith(7, [21, 22])
    expect(ensureValidationsSpy).toHaveBeenCalledWith(7, [11, 12])
    expect(getCollectionsSpy).toHaveBeenCalled()
  })

  it("getGroupLots loads and stores lots", async () => {
    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [{ id: 10, name: "L" }],
      total: 1,
    } as any)

    const store = useLotStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.getGroupLots(5)

    expect(result?.length).toBe(1)
    expect(store.getLotById(10)?.name).toBe("L")
    expect(store.total).toBe(1)
  })

  it("getRecentOrders calls getGroupLots with Requested status", async () => {
    const spy = vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [],
      total: 0,
    } as any)

    const store = useLotStore()
    await store.getRecentOrders(3)

    expect(spy).toHaveBeenCalled()
  })
})

/* ---------------- write paths ---------------- */

describe("lot store – write actions", () => {
  it("createLot inserts and notifies", async () => {
    vi.spyOn(jsonApi, "rpc")
      .mockResolvedValueOnce({ id: 7, name: "New" } as any)
      .mockResolvedValueOnce({
        items: [{ id: 7, name: "New" }],
        total: 1,
      } as any)

    const store = useLotStore()
    const main = useMainStore()

    const notifSpy = vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    await store.createLot({} as any)

    expect(store.getLotById(7)?.name).toBe("New")
    expect(notifSpy).toHaveBeenCalled()
  })

  it("deleteLot removes and notifies", async () => {
    vi.spyOn(jsonApi, "rpc").mockResolvedValue(undefined)

    const store = useLotStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    store.setEntity({ id: 9, name: "X" } as any)

    await store.deleteLot(9)

    expect(store.getLotById(9)).toBeUndefined()
  })

  it("updateLotStatus updates and notifies", async () => {
    vi.spyOn(jsonApi, "rpc")
      .mockResolvedValueOnce(undefined as any)
      .mockResolvedValueOnce({
        items: [{ id: 3, name: "L", status: LotStatus.Low }],
        total: 1,
      } as any)

    const store = useLotStore()
    const main = useMainStore()

    const notifSpy = vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    store.setEntity({ id: 3, name: "L", status: LotStatus.Requested } as any)
    await store.updateLotStatus(3, { status: LotStatus.Low } as any)

    expect(store.getLotById(3)?.status).toBe(LotStatus.Low)
    expect(notifSpy).toHaveBeenCalled()
  })

  it("updateLotStatus maps lotNumber to number in the RPC payload", async () => {
    const rpcSpy = vi.spyOn(jsonApi, "rpc")
      .mockResolvedValueOnce(undefined as any)
      .mockResolvedValueOnce({
        items: [{ id: 4, name: "L", status: LotStatus.Stock, number: "LOT-42" }],
        total: 1,
      } as any)

    const store = useLotStore()
    const main = useMainStore()

    vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    store.setEntity({ id: 4, name: "L", status: LotStatus.Ordered, number: "Pending" } as any)
    await store.updateLotStatus(4, { status: LotStatus.Stock, lotNumber: "LOT-42" } as any)

    expect(rpcSpy).toHaveBeenNthCalledWith(1, expect.objectContaining({
      operation: "Update",
      return_type: "Lot",
      id: 4,
      payload: { status: LotStatus.Stock, number: "LOT-42" },
    }))
  })
})
