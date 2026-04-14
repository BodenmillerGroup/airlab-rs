import { describe, it, expect, vi, beforeEach } from "vitest"

import { useValidationFiles } from "@/composables/useValidationFiles"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useValidationFileStore } from "@/stores/validation_file"
import { useMainStore } from "@/stores/main"

describe("useValidationFiles", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads validation files and builds ValidationFileView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const validationFileStore = useValidationFileStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(validationFileStore, "page", "get").mockReturnValue(1)
    vi.spyOn(validationFileStore, "listFiles", "get").mockReturnValue([
      { id: 101 },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(validationFileStore, "loadListQuery").mockResolvedValue([
      { id: 101 },
    ] as any)
    vi.spyOn(validationFileStore, "reloadListQuery").mockResolvedValue([] as any)

    const validationFiles = useValidationFiles()
    await validationFiles.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(validationFiles.loading.value).toBe(false)
    expect(validationFiles.items.value).toEqual([
      expect.objectContaining({
        id: 101,
      }),
    ])
  })
})
