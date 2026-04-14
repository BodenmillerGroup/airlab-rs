import { describe, it, expect, vi, beforeEach } from "vitest"

import { useTags } from "@/composables/useTags"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useTagStore } from "@/stores/tag"
import { useMainStore } from "@/stores/main"

describe("useTags", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads tags and builds TagView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const tagStore = useTagStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(tagStore, "page", "get").mockReturnValue(1)
    vi.spyOn(tagStore, "listTags", "get").mockReturnValue([
      {
        id: 6,
        groupId: 10,
        name: "Alexa-647",
        isMetal: false,
        isFluorophore: true,
        isEnzyme: false,
        isBiotin: false,
        isOther: false,
        status: 0,
        description: null,
        mw: null,
        emission: null,
        excitation: null,
        meta: {},
        createdAt: "now",
      },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(tagStore, "loadListQuery").mockResolvedValue([
      { id: 6 },
    ] as any)
    vi.spyOn(tagStore, "reloadListQuery").mockResolvedValue([] as any)

    const tags = useTags()
    await tags.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(tags.loading.value).toBe(false)
    expect(tags.items.value).toEqual([
      expect.objectContaining({
        id: 6,
        groupId: 10,
        name: "Alexa-647",
        isFluorophore: true,
      }),
    ])
  })
})
