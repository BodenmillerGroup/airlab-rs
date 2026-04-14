import { describe, it, expect, vi, beforeEach } from "vitest"

import { useProviders } from "@/composables/useProviders"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useProviderStore } from "@/stores/provider"
import { useMainStore } from "@/stores/main"

describe("useProviders", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads providers and builds ProviderView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const providerStore = useProviderStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(providerStore, "page", "get").mockReturnValue(1)
    vi.spyOn(providerStore, "listProviders", "get").mockReturnValue([
      {
        id: 2,
        groupId: 10,
        name: "Sigma",
        description: "Chemicals",
        url: "",
        meta: {},
        createdAt: "now",
      },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(providerStore, "loadListQuery").mockResolvedValue([
      { id: 2 },
    ] as any)
    vi.spyOn(providerStore, "reloadListQuery").mockResolvedValue([] as any)

    const providers = useProviders()
    await providers.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(providers.loading.value).toBe(false)
    expect(providers.items.value).toEqual([
      expect.objectContaining({
        id: 2,
        groupId: 10,
        name: "Sigma",
        description: "Chemicals",
      }),
    ])
  })
})
