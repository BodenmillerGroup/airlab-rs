import { describe, it, expect, beforeEach, vi } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { useUserStore } from "@/stores/user"
import { useMainStore } from "@/stores/main"

/* ---------------- mocks ---------------- */

vi.mock("@/modules/user/api", () => ({
  api: {
    getUsers: vi.fn(),
    getUser: vi.fn(),
    createUser: vi.fn(),
    updateUser: vi.fn(),
    checkUserExists: vi.fn(),
    signUp: vi.fn(),
  },
}))

vi.mock("@/modules/json/api", () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}))

import { api } from "@/modules/user/api"
import * as jsonApi from "@/modules/json/api"

/* ---------------- setup ---------------- */

beforeEach(() => {
  setActivePinia(createPinia())
  vi.clearAllMocks()
})

/* ---------------- state & getters ---------------- */

describe("user store – state & getters", () => {
  it("sets entities and exposes getters", () => {
    const store = useUserStore()

    store.setEntities([
      { id: 1, name: "Alice", email: "a@test.com" },
      { id: 2, email: "b@test.com" },
    ] as any)

    expect(store.users.length).toBe(2)
    expect(store.getUserById(1)?.name).toBe("Alice")
    expect(store.hasUser(2)).toBe(true)

    expect(store.nameMap[1]).toBe("Alice")
    expect(store.nameMap[2]).toBe("b@test.com")
  })

  it("updates and resets correctly", () => {
    const store = useUserStore()

    store.setEntity({ id: 1, name: "Old" } as any)
    store.updateEntity({ id: 1, name: "New" } as any)

    expect(store.getUserById(1)?.name).toBe("New")

    store.reset()

    expect(store.ids).toEqual([])
    expect(store.entities).toEqual({})
  })

  it("ignores malformed user payloads without numeric ids", () => {
    const store = useUserStore()

    store.setEntity({ name: "Broken" } as any)
    store.addEntity({ id: Number.NaN, name: "Broken 2" } as any)
    store.updateEntity({ id: undefined, name: "Broken 3" } as any)
    store.setEntities([
      { id: 1, name: "Valid" },
      { name: "Invalid" },
    ] as any)

    expect(store.ids).toEqual([1])
    expect(Object.keys(store.entities)).toEqual(["1"])
    expect(store.getUserById(1)?.name).toBe("Valid")
  })
})

/* ---------------- API reads ---------------- */

describe("user store – API reads", () => {
  it("loadListQuery stores visible ids", async () => {
    vi.spyOn(jsonApi, "rpcSearch").mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any)

    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [
        { id: 1, name: "A" },
        { id: 2, name: "B" },
      ],
    } as any)

    const store = useUserStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.loadListQuery({
      groupId: 7,
      filters: [],
      order: { table: "User", field: "id", direction: "asc" },
    } as any)

    expect(result.map((user) => user.id)).toEqual([1, 2])
    expect(store.listIds).toEqual([1, 2])
    expect(store.listUsers.map((user) => user.id)).toEqual([1, 2])
  })

  it("getUsers loads and stores users", async () => {
    vi.spyOn(api, "getUsers").mockResolvedValue([
      { id: 1, name: "A" },
      { id: 2, name: "B" },
    ] as any)

    const store = useUserStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.getUsers()

    expect(store.users.length).toBe(2)
    expect(result?.[0].name).toBe("A")
    expect(store.loading).toBe(false)
  })

  it("getUser loads and stores a user via RPC", async () => {
    vi.spyOn(api, "getUser").mockResolvedValue({
      id: 12,
      name: "Fetched",
      email: "fetched@test.com",
    } as any)

    const store = useUserStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.getUser(12)

    expect(result?.id).toBe(12)
    expect(store.getUserById(12)?.name).toBe("Fetched")
  })

  it("checkUserExists returns boolean", async () => {
    vi.spyOn(api, "checkUserExists").mockResolvedValue({ exists: true } as any)

    const store = useUserStore()
    const main = useMainStore()
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const exists = await store.checkUserExists("x@test.com")

    expect(exists).toBe(true)
  })
})

/* ---------------- API writes ---------------- */

describe("user store – API writes", () => {
  it("createUser inserts and notifies", async () => {
    vi.spyOn(api, "createUser").mockResolvedValue({ id: 5, name: "New" } as any)
    vi.spyOn(api, "getUser").mockResolvedValue({ id: 5, name: "New" } as any)

    const store = useUserStore()
    const main = useMainStore()
    const notifSpy = vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    await store.createUser({} as any)

    expect(store.getUserById(5)?.name).toBe("New")
    expect(notifSpy).toHaveBeenCalled()
  })

  it("signUp triggers logout routing and notification", async () => {
    vi.spyOn(api, "signUp").mockResolvedValue(undefined)

    const store = useUserStore()
    const main = useMainStore()

    const routeSpy = vi.spyOn(main, "routeLogOut").mockImplementation(() => { })
    const notifSpy = vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    await store.signUp({} as any)

    expect(routeSpy).toHaveBeenCalled()
    expect(notifSpy).toHaveBeenCalled()
  })

  it("updateUser uses RPC update and refreshes the user", async () => {
    const rpcSpy = vi.spyOn(jsonApi, "rpc").mockResolvedValue(undefined as any)
    vi.spyOn(api, "getUser").mockResolvedValue({
      id: 302,
      name: "Updated User",
      email: "updated@test.com",
      isAdmin: false,
      isActive: true,
    } as any)

    const store = useUserStore()
    const main = useMainStore()
    const notifSpy = vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    const result = await store.updateUser({
      id: 302,
      data: { name: "Updated User", email: "updated@test.com", isAdmin: false, isActive: true } as any,
    })

    expect(rpcSpy).toHaveBeenNthCalledWith(1, expect.objectContaining({
      operation: "Update",
      return_type: "User",
      id: 302,
      payload: { name: "Updated User", email: "updated@test.com", is_admin: false, is_active: true },
    }))
    expect(result?.id).toBe(302)
    expect(store.getUserById(302)?.name).toBe("Updated User")
    expect(notifSpy).toHaveBeenCalled()
  })

  it("updateUser omits empty passwords from the RPC payload", async () => {
    const rpcSpy = vi.spyOn(jsonApi, "rpc").mockResolvedValue(undefined as any)
    vi.spyOn(api, "getUser").mockResolvedValue({
      id: 302,
      name: "Updated User",
      email: "updated@test.com",
      isAdmin: true,
      isActive: false,
    } as any)

    const store = useUserStore()
    const main = useMainStore()
    vi.spyOn(main, "addNotification").mockImplementation(() => { })
    vi.spyOn(main, "checkApiError").mockResolvedValue()

    await store.updateUser({
      id: 302,
      data: { name: "Updated User", email: "updated@test.com", isAdmin: true, isActive: false, password: "" } as any,
    })

    expect(rpcSpy).toHaveBeenCalledWith(expect.objectContaining({
      payload: { name: "Updated User", email: "updated@test.com", is_admin: true, is_active: false },
    }))
  })
})
