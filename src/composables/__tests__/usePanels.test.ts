import { describe, it, expect, vi, beforeEach } from "vitest"
import { nextTick } from "vue"

import { usePanels } from "@/composables/usePanels"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { usePanelStore } from "@/stores/panel"
import { usePanelElementStore } from "@/stores/panel_element"
import { useUserStore } from "@/stores/user"
import { useMemberStore } from "@/stores/member"
import { useMainStore } from "@/stores/main"

describe("usePanels", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads panels and builds PanelView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const panelStore = usePanelStore()
    const userStore = useUserStore()
    const memberStore = useMemberStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(panelStore, "page", "get").mockReturnValue(1)
    vi.spyOn(panelStore, "listPanels", "get").mockReturnValue([
      {
        id: 3,
        groupId: 10,
        createdBy: 21,
        name: "My Panel",
        description: "Test panel",
        isFluorophore: false,
        isLocked: false,
        application: 0,
        meta: {},
        isArchived: false,
        createdAt: "now",
        updatedAt: "now",
      },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(panelStore, "loadListQuery").mockResolvedValue([
      { id: 3 },
    ] as any)
    vi.spyOn(panelStore, "reloadListQuery").mockResolvedValue([] as any)

    vi.spyOn(memberStore, "getMemberById").mockReturnValue({ id: 21, userId: 1 } as any)
    vi.spyOn(userStore, "getUserById").mockReturnValue({ id: 1, name: "Test User" } as any)

    const panels = usePanels()
    await panels.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(panels.loading.value).toBe(false)
    expect(panels.items.value).toEqual([
      expect.objectContaining({
        id: 3,
        groupId: 10,
        name: "My Panel",
        description: "Test panel",
        userName: "Test User",
      }),
    ])
  })

  it("filters panels by conjugate through panel elements", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const panelStore = usePanelStore()
    const panelElementStore = usePanelElementStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(panelStore, "page", "get").mockReturnValue(1)
    vi.spyOn(panelStore, "listPanels", "get").mockReturnValue([] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const panelElementLookupSpy = vi.spyOn(panelElementStore, "getByConjugateId").mockResolvedValue([
      { id: 101, panelId: 7, conjugateId: 55 },
      { id: 102, panelId: 9, conjugateId: 55 },
    ] as any)
    const loadListSpy = vi.spyOn(panelStore, "loadListQuery").mockResolvedValue([] as any)
    vi.spyOn(panelStore, "reloadListQuery").mockResolvedValue([] as any)

    usePanels({ conjugateId: 55 })
    await nextTick()
    await Promise.resolve()

    expect(panelElementLookupSpy).toHaveBeenCalledWith(55)
    expect(loadListSpy).toHaveBeenCalledWith(expect.objectContaining({
      groupId: 10,
      filters: expect.arrayContaining([
        expect.objectContaining({
          table: "Panel",
          field: "group_id",
          op: "eq",
          value: 10,
        }),
        expect.objectContaining({
          table: "Panel",
          field: "id",
          op: "in",
          value: [7, 9],
        }),
      ]),
    }))
  })
})
