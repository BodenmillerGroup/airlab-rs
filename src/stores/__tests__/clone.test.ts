import { describe, it, expect, beforeEach, vi } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { useCloneStore } from "@/stores/clone"
import { useMainStore } from "@/stores/main"
import { useProteinStore } from "@/stores/protein"
import { useSpeciesStore } from "@/stores/species"
import { useValidationStore } from "@/stores/validation"

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

describe("clone store – state & getters", () => {
  it("sets entities and selectors work", () => {
    const store = useCloneStore()

    store.fetchByIds // just touch export

    store.reset()

    store["ids"] = [1, 2] as any
    store["entities"] = {
      1: { id: 1, name: "A", groupId: 10 },
      2: { id: 2, name: "B", groupId: 20 },
    } as any

    expect(store.clones.length).toBe(2)
    expect(store.getClone(1)?.name).toBe("A")
    expect(store.getGroupClones(10)[0].id).toBe(1)
  })

  it("getByIds returns only existing clones", () => {
    const store = useCloneStore()

    store["ids"] = [1] as any
    store["entities"] = {
      1: { id: 1, name: "A", groupId: 1 },
    } as any

    const result = store.getByIds([1, 2])

    expect(result.length).toBe(1)
    expect(result[0].id).toBe(1)
  })
})

/* ---------------- CSV ---------------- */

describe("clone store – CSV export", () => {
  it("builds a valid CSV string", () => {
    const store = useCloneStore()

    const csv = store.getCsv([
      {
        id: 1,
        name: "CloneA",
        isotype: "IgG",
        epitope: "E1",
        isPhospho: true,
        isPolyclonal: false,
        protein: { name: "Prot" },
        species: { name: "Human" },
      },
    ] as any)

    expect(csv).toContain("Clone")
    expect(csv).toContain("CloneA")
    expect(csv).toContain("Prot")
    expect(csv).toContain("Human")
    expect(csv.split("\n").length).toBe(2)
  })
})

/* ---------------- cache fetch ---------------- */

describe("clone store – fetchByIds", () => {
  it("fetches only missing clones", async () => {
    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [{ id: 2, name: "B", groupId: 1 }],
    } as any)

    const store = useCloneStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    store["entities"] = { 1: { id: 1, name: "A" } } as any
    store["ids"] = [1] as any

    const result = await store.fetchByIds([1, 2, 2])

    expect(result.length).toBe(1)
    expect(store.getClone(2)?.name).toBe("B")
  })
})

/* ---------------- search orchestration ---------------- */

describe("clone store – search()", () => {
  it("searches, fetches missing clones and returns full objects", async () => {
    vi.spyOn(jsonApi, "rpcSearch").mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any)

    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [
        { id: 1, name: "A", groupId: 1 },
        { id: 2, name: "B", groupId: 1 },
      ],
    } as any)

    const store = useCloneStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.search([], { field: "id", dir: "asc" } as any)

    expect(result?.map(c => c.name)).toEqual(["A", "B"])
    expect(store.total).toBe(2)
    expect(store.loading).toBe(false)
  })

  it("loadListQuery stores visible ids and ensures related data in dependent stores", async () => {
    vi.spyOn(jsonApi, "rpcSearch").mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any)

    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [
        { id: 1, name: "A", groupId: 7, proteinId: 11, speciesId: 21 },
        { id: 2, name: "B", groupId: 7, proteinId: 12, speciesId: 22 },
      ],
    } as any)

    const store = useCloneStore()
    const proteinStore = useProteinStore()
    const speciesStore = useSpeciesStore()
    const validationStore = useValidationStore()
    const main = useMainStore()

    vi.spyOn(main, "checkApiError").mockResolvedValue()
    const ensureProteinSpy = vi.spyOn(proteinStore, "ensureByIds").mockResolvedValue([] as any)
    const ensureSpeciesSpy = vi.spyOn(speciesStore, "ensureByIds").mockResolvedValue([] as any)
    const ensureValidationSpy = vi.spyOn(validationStore, "ensureForCloneIds").mockResolvedValue([] as any)

    const result = await store.loadListQuery({
      groupId: 7,
      filters: [],
      order: { table: "Clone", field: "id", direction: "asc" },
    } as any)

    expect(result.map(c => c.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listClones.map(c => c.id)).toEqual([1, 2])
    expect(ensureProteinSpy).toHaveBeenCalledWith(7, [11, 12])
    expect(ensureSpeciesSpy).toHaveBeenCalledWith(7, [21, 22])
    expect(ensureValidationSpy).toHaveBeenCalledWith(7, [1, 2])
  })
})

/* ---------------- write paths ---------------- */

describe("clone store – write actions", () => {
  it("createClone inserts and notifies", async () => {
    vi.spyOn(jsonApi, "rpc").mockResolvedValue({ id: 9, name: "New", groupId: 1 } as any)

    const store = useCloneStore()
    const main = useMainStore()

    const notifSpy = vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    await store.createClone({} as any)

    expect(store.getClone(9)?.name).toBe("New")
    expect(notifSpy).toHaveBeenCalled()
  })

  it("deleteClone removes clone", async () => {
    vi.spyOn(jsonApi, "rpc").mockResolvedValue(undefined)

    const store = useCloneStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    store["entities"] = { 5: { id: 5, name: "X" } } as any
    store["ids"] = [5] as any

    await store.deleteClone(5)

    expect(store.getClone(5)).toBeUndefined()
  })

  it("updateClone patches local entity before refetch completes", async () => {
    const rpcSpy = vi.spyOn(jsonApi, "rpc")
    rpcSpy
      .mockResolvedValueOnce(undefined as any)
      .mockResolvedValueOnce({
        id: 5,
        name: "Updated",
        groupId: 1,
        proteinId: 10,
        speciesId: 20,
      } as any)

    const store = useCloneStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()
    vi.spyOn(main, "addNotification").mockImplementation(() => {})

    store["entities"] = {
      5: {
        id: 5,
        name: "Old",
        groupId: 1,
        proteinId: 1,
        speciesId: 2,
      },
    } as any
    store["ids"] = [5] as any

    const promise = store.updateClone({
      id: 5,
      data: {
        name: "Updated",
        proteinId: 10,
        speciesId: 20,
        epitope: "",
        isotype: "",
        isPhospho: false,
        isPolyclonal: false,
        reactivity: [],
        application: {},
      },
    } as any)

    expect(store.getClone(5)?.speciesId).toBe(20)
    expect(store.getClone(5)?.proteinId).toBe(10)

    await promise
  })
})
