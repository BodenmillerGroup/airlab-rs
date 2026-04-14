import { describe, it, expect, beforeEach, vi } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { useGroupStore } from "@/stores/group"
import { useMainStore } from "@/stores/main"
import { useMemberStore } from "@/stores/member"
import * as jsonApi from "@/modules/json/api"
import type { SearchFilter } from "@/modules/json/api"

describe("group store – state, mutations, getters", () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it("sets and retrieves entities correctly", () => {
    const store = useGroupStore()

    store.setEntity({ id: 1, name: "A" } as any)
    store.setEntity({ id: 2, name: "B" } as any)

    expect(store.ids).toEqual([1, 2])
    expect(store.groups.map(g => g.name)).toEqual(["A", "B"])
    expect(store.getGroupById(1)?.name).toBe("A")
  })

  it("updates and deletes entities", () => {
    const store = useGroupStore()

    store.setEntity({ id: 1, name: "Old" } as any)
    store.updateEntity({ id: 1, name: "New" } as any)

    expect(store.getGroupById(1)?.name).toBe("New")

    store.deleteEntity(1)

    expect(store.ids).toEqual([])
    expect(store.getGroupById(1)).toBeUndefined()
  })

  it("handles active group correctly", () => {
    const store = useGroupStore()

    store.setEntity({ id: 1, name: "A" } as any)
    store.setActiveGroupId(1)

    expect(store.hasActiveGroup).toBe(true)
    expect(store.activeGroup?.name).toBe("A")
  })

  it("computes role and admin flags", () => {
    const store = useGroupStore()

    store.setMyMember({ role: 50, allPanels: false } as any)
    expect(store.groupRole).toBe(50)
    expect(store.isGroupAdmin).toBe(false)

    store.setMyMember({ role: 100, allPanels: true } as any)
    expect(store.isGroupAdmin).toBe(true)
    expect(store.allPanels).toBe(true)
  })

  it("resets state", () => {
    const store = useGroupStore()

    store.setEntity({ id: 1, name: "A" } as any)
    store.setActiveGroupId(1)
    store.setMyMember({ role: 1 } as any)

    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
    expect(store.activeGroupId).toBeUndefined()
    expect(store.myMember).toBeNull()
  })
})

describe("group store – actions", () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.restoreAllMocks()
  })

  it("getGroups loads and stores groups", async () => {
    const rpcSpy = vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [
        { id: 1, name: "A" },
        { id: 2, name: "B" },
      ],
    } as any)

    const store = useGroupStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.getGroups()

    expect(rpcSpy).toHaveBeenCalled()
    expect(store.groups.map(g => g.name)).toEqual(["A", "B"])
    expect(store.loading).toBe(false)
    expect(result?.length).toBe(2)
  })

  it("getGroups handles errors", async () => {
    const rpcSpy = vi.spyOn(jsonApi, "rpc").mockRejectedValue(new Error("boom"))

    const store = useGroupStore()
    const main = useMainStore()
    const errSpy = vi.spyOn(main, "checkApiError").mockResolvedValue()

    await store.getGroups()

    expect(rpcSpy).toHaveBeenCalled()
    expect(errSpy).toHaveBeenCalled()
    expect(store.loading).toBe(false)
  })

  it("loadListQuery stores visible ids and ensures related memberships", async () => {
    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [
        { id: 1, name: "A" },
        { id: 2, name: "B" },
      ],
    } as any)

    const store = useGroupStore()
    const memberStore = useMemberStore()
    const main = useMainStore()

    vi.spyOn(main, "checkApiError").mockResolvedValue()
    const ensureMembersSpy = vi.spyOn(memberStore, "loadListQuery").mockResolvedValue([] as any)

    const result = await store.loadListQuery({ userId: 5 })

    expect(result.map((group) => group.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listGroups.map((group) => group.id)).toEqual([1, 2])
    expect(ensureMembersSpy).toHaveBeenCalledWith({
      groupId: -1,
      filters: [
        { table: "Member", field: "group_id", op: "in", value: [1, 2] },
        { table: "Member", field: "user_id", op: "eq", value: 5 },
        { table: "Member", field: "is_active", op: "eq", value: true },
      ] satisfies SearchFilter[],
      order: { table: "Member", field: "id", direction: "asc" },
    })
  })
})
