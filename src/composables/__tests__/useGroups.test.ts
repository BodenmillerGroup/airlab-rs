import { describe, it, expect, vi } from "vitest"
import { ref } from "vue"
import { useGroups } from "../useGroups"

import { useGroupStore } from "@/stores/group"
import { useMemberStore } from "@/stores/member"
import { useMainStore } from "@/stores/main"
import { useFilterStore } from "@/stores/useFilterStore"

describe("useGroups", () => {
  it("loads groups and builds joined GroupView", async () => {
    // --- arrange (stores) ---
    const groupStore = useGroupStore()
    const memberStore = useMemberStore()
    const mainStore = useMainStore()
    const filterStore = useFilterStore()

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()

    vi.spyOn(groupStore, "listGroups", "get").mockReturnValue([
      { id: 2, name: "G2", createdAt: "2024-01-02" },
      { id: 1, name: "G1", createdAt: "2024-01-01" },
    ] as any)

    const loadListSpy = vi.spyOn(groupStore, "loadListQuery").mockResolvedValue([
      { id: 2 },
      { id: 1 },
    ] as any)
    vi.spyOn(groupStore, "reloadListQuery").mockResolvedValue([] as any)

    vi.spyOn(memberStore, "getByGroupId").mockImplementation(
      ((groupId: number) => {
        if (groupId === 1) {
          return [
            { id: 10, role: "admin", userId: 5, groupId: 1 },
            { id: 11, role: "user", userId: 6, groupId: 1 },
          ]
        }
        if (groupId === 2) {
          return [
            { id: 20, role: "user", userId: 5, groupId: 2 },
          ]
        }
        return []
      }) as any
    )



    filterStore.filters = {} as any

    // --- act ---
    const userId = ref(5)
    const groups = useGroups(userId)

    // wait until async watcher finishes
    await Promise.resolve()
    await Promise.resolve()

    // --- assert ---
    expect(loadListSpy).toHaveBeenCalledOnce()

    expect(groups.loading.value).toBe(false)

    expect(groups.items.value).toEqual([
      {
        id: 1,
        name: "G1",
        createdAt: "2024-01-01",
        members: [
          { id: 10, role: "admin", userId: 5 },
          { id: 11, role: "user", userId: 6 },
        ],
      },
      {
        id: 2,
        name: "G2",
        createdAt: "2024-01-02",
        members: [
          { id: 20, role: "user", userId: 5 },
        ],
      },
    ])
  })
})
