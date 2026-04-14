import { describe, it, expect, beforeEach, vi } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { useMemberStore } from "@/stores/member"
import { useMainStore } from "@/stores/main"
import { useUserStore } from "@/stores/user"

/* ---------------- mocks ---------------- */

vi.mock("@/modules/json/api", () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

vi.mock("@/modules/member/api", () => ({
  api: {
    getGroupMembers: vi.fn(),
    getMember: vi.fn(),
    createMember: vi.fn(),
    updateMember: vi.fn(),
    deleteMember: vi.fn(),
  },
}))

import * as jsonApi from "@/modules/json/api"
import { api } from "@/modules/member/api"

/* ---------------- setup ---------------- */

beforeEach(() => {
  setActivePinia(createPinia())
  vi.clearAllMocks()
})

/* ---------------- state & getters ---------------- */

describe("member store – state & getters", () => {
  it("sets entities and exposes getters", () => {
    const store = useMemberStore()

    store.setEntities([
      { id: 1, name: "A", groupId: 10 },
      { id: 2, name: "B", groupId: 20 },
      { id: 3, name: "C", groupId: 10 },
    ] as any)

    expect(store.members.length).toBe(3)
    expect(store.hasMember(1)).toBe(true)
    expect(store.getMembersForGroup(10).map(m => m.id)).toEqual([1, 3])
    expect(store.getByGroupId(20)[0].id).toBe(2)
  })

  it("deletes and resets correctly", () => {
    const store = useMemberStore()

    store.setEntities([{ id: 1 }, { id: 2 }] as any)
    store.deleteEntity(1)

    expect(store.ids).toEqual([2])

    store.reset()
    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
  })
})

/* ---------------- cache-aware fetch ---------------- */

describe("member store – fetchByIds", () => {
  it("fetches only missing members", async () => {
    const rpcSpy = vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [{ id: 2, name: "B" }],
    } as any)

    const store = useMemberStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    store.setEntities([{ id: 1, name: "A" }] as any)

    const result = await store.fetchByIds([1, 2, 2])

    expect(rpcSpy).toHaveBeenCalled()
    expect(result?.length).toBe(1)
  })

  it("force refreshes cached members when requested", async () => {
    const rpcSpy = vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [{ id: 1, name: "A2", role: 100, allPanels: false }],
    } as any)

    const store = useMemberStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    store.setEntities([{ id: 1, name: "A1", role: 10, allPanels: true }] as any)

    const result = await store.fetchByIds([1], true)

    expect(rpcSpy).toHaveBeenCalledWith(expect.objectContaining({
      operation: "Get",
      return_type: "Member",
      filters: [{ field: "id", op: "in", value: [1] }],
    }))
    expect(result?.[0]?.role).toBe(100)
    expect(store.getMemberById(1)?.allPanels).toBe(false)
  })
})

/* ---------------- search orchestration ---------------- */

describe("member store – search()", () => {
  it("searches, fetches missing members and returns full objects", async () => {
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

    const store = useMemberStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.search([], { field: "id", dir: "asc" } as any)

    expect(store.total).toBe(2)
    expect(store.loading).toBe(false)
  })

  it("loadListQuery stores visible ids and ensures related users", async () => {
    vi.spyOn(jsonApi, "rpcSearch").mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any)

    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [
        { id: 1, groupId: 7, userId: 11, name: "A" },
        { id: 2, groupId: 7, userId: 12, name: "B" },
      ],
    } as any)

    const store = useMemberStore()
    const userStore = useUserStore()
    const main = useMainStore()

    vi.spyOn(main, "checkApiError").mockResolvedValue()
    const ensureUsersSpy = vi.spyOn(userStore, "fetchByIds").mockResolvedValue([] as any)

    const result = await store.loadListQuery({
      groupId: 7,
      filters: [],
      order: { table: "Member", field: "id", direction: "asc" },
    } as any)

    expect(result.map((member) => member.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listMembers.map((member) => member.id)).toEqual([1, 2])
    expect(ensureUsersSpy).toHaveBeenCalledWith([11, 12])
  })

  it("loadListQuery refreshes cached visible members", async () => {
    vi.spyOn(jsonApi, "rpcSearch").mockResolvedValue({
      items: [1],
      search_total: 1,
    } as any)

    const rpcSpy = vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [{ id: 1, groupId: 7, userId: 11, role: 100, allPanels: false }],
    } as any)

    const store = useMemberStore()
    const userStore = useUserStore()
    const main = useMainStore()

    store.setEntities([{ id: 1, groupId: 7, userId: 11, role: 10, allPanels: true }] as any)

    vi.spyOn(main, "checkApiError").mockResolvedValue()
    vi.spyOn(userStore, "fetchByIds").mockResolvedValue([] as any)

    await store.loadListQuery({
      groupId: 7,
      filters: [],
      order: { table: "Member", field: "id", direction: "asc" },
    } as any)

    expect(rpcSpy).toHaveBeenCalledWith(expect.objectContaining({
      operation: "Get",
      return_type: "Member",
      filters: [{ field: "id", op: "in", value: [1] }],
    }))
    expect(store.getMemberById(1)?.role).toBe(100)
    expect(store.getMemberById(1)?.allPanels).toBe(false)
  })
})

/* ---------------- API actions ---------------- */

describe("member store – API actions", () => {
  it("getMember loads and stores the member via RPC", async () => {
    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      id: 11,
      groupId: 1,
      userId: 21,
      role: 10,
      isActive: true,
      allPanels: false,
    } as any)

    const store = useMemberStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.getMember(11)

    expect(result?.id).toBe(11)
    expect(store.getMemberById(11)?.role).toBe(10)
  })

  it("getGroupMembers loads and stores members", async () => {
    vi.spyOn(api, "getGroupMembers").mockResolvedValue([
      { id: 1, name: "A", groupId: 1 },
    ] as any)

    const store = useMemberStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    await store.getGroupMembers(1)

    expect(store.members.length).toBe(1)
  })

  it("createMember inserts and notifies", async () => {
    vi.spyOn(api, "createMember").mockResolvedValue({ id: 3, name: "C" } as any)

    const store = useMemberStore()
    const main = useMainStore()
    const notifSpy = vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    await store.createMember({} as any)

    expect(notifSpy).toHaveBeenCalled()
  })

  it("updateMember uses RPC update and refreshes the member", async () => {
    const rpcSpy = vi.spyOn(jsonApi, "rpc")
      .mockResolvedValueOnce(undefined as any)
      .mockResolvedValueOnce({
        id: 302,
        groupId: 9,
        userId: 18,
        role: 100,
        isActive: true,
        allPanels: false,
      } as any)

    const store = useMemberStore()
    const main = useMainStore()
    const notifSpy = vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.updateMember({
      id: 302,
      data: { role: 100, isActive: true, allPanels: false },
    })

    expect(rpcSpy).toHaveBeenCalledWith(expect.objectContaining({
      operation: "Update",
      return_type: "Member",
      id: 302,
      payload: { role: 100, isActive: true, allPanels: false },
    }))
    expect(rpcSpy).toHaveBeenNthCalledWith(2, expect.objectContaining({
      operation: "Get",
      return_type: "Member",
      payload: 302,
    }))
    expect(result?.id).toBe(302)
    expect(store.getMemberById(302)?.role).toBe(100)
    expect(notifSpy).toHaveBeenCalled()
  })

  it("removeMember deletes and notifies", async () => {
    vi.spyOn(api, "deleteMember").mockResolvedValue(undefined)

    const store = useMemberStore()
    const main = useMainStore()
    const notifSpy = vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    store.setEntities([{ id: 9, name: "X" }] as any)

    await store.removeMember(9)

    expect(store.hasMember(9)).toBe(false)
    expect(notifSpy).toHaveBeenCalled()
  })
})
