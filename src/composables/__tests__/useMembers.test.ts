import { describe, it, expect, vi, beforeEach } from "vitest"

import { useMembers } from "@/composables/useMembers"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useMemberStore } from "@/stores/member"
import { useUserStore } from "@/stores/user"
import { useMainStore } from "@/stores/main"

describe("useMembers", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads members and builds MemberView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const memberStore = useMemberStore()
    const userStore = useUserStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(memberStore, "page", "get").mockReturnValue(1)
    vi.spyOn(memberStore, "listMembers", "get").mockReturnValue([
      {
        id: 5,
        groupId: 10,
        userId: 42,
        role: "admin",
        activationKey: "",
        isActive: true,
        allPanels: false,
        createdAt: "now",
        updatedAt: "now",
      },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(memberStore, "loadListQuery").mockResolvedValue([
      { id: 5 },
    ] as any)
    vi.spyOn(memberStore, "reloadListQuery").mockResolvedValue([] as any)
    vi.spyOn(userStore, "getUserById").mockReturnValue({ id: 42, name: "Test User" } as any)

    const members = useMembers()
    await members.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(members.loading.value).toBe(false)
    expect(members.items.value).toEqual([
      expect.objectContaining({
        id: 5,
        groupId: 10,
        userId: 42,
        userName: "Test User",
        role: "admin",
        isActive: true,
      }),
    ])
  })
})
