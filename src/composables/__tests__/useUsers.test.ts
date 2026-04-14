import { describe, it, expect, vi, beforeEach } from "vitest"

import { useUsers } from "@/composables/useUsers"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useUserStore } from "@/stores/user"
import { useMainStore } from "@/stores/main"

describe("useUsers", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads users and builds UserView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const userStore = useUserStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(userStore, "page", "get").mockReturnValue(1)
    vi.spyOn(userStore, "listUsers", "get").mockReturnValue([
      {
        id: 11,
        email: "test@example.com",
        name: "Test User",
        isActive: true,
        isAdmin: false,
        mfaEnabled: true,
        createdAt: "now",
        updatedAt: "now",
      },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(userStore, "loadListQuery").mockResolvedValue([
      { id: 11 },
    ] as any)
    vi.spyOn(userStore, "reloadListQuery").mockResolvedValue([] as any)

    const users = useUsers()
    await users.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(users.loading.value).toBe(false)
    expect(users.items.value).toEqual([
      expect.objectContaining({
        id: 11,
        email: "test@example.com",
        name: "Test User",
        isActive: true,
        isAdmin: false,
        mfaEnabled: true,
      }),
    ])
  })
})
