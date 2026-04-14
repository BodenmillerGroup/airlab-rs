import { describe, it, expect, vi, beforeEach } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { ref } from "vue"

import { useTagConjugates } from "@/composables/useTagConjugates"

import { useGroupStore } from "@/stores/group"
import { useConjugateStore } from "@/stores/conjugate"
import { useLotStore } from "@/stores/lot"
import { useCloneStore } from "@/stores/clone"
import { useProteinStore } from "@/stores/protein"
import { useTagStore } from "@/stores/tag"
import { useValidationStore } from "@/stores/validation"
import { useSpeciesStore } from "@/stores/species"
import { useMainStore } from "@/stores/main"

describe("useTagConjugates", () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.restoreAllMocks()
  })

  it("loads tag conjugates and builds TagConjugateView", async () => {
    const groupStore = useGroupStore()
    const conjugateStore = useConjugateStore()
    const lotStore = useLotStore()
    const cloneStore = useCloneStore()
    const proteinStore = useProteinStore()
    const tagStore = useTagStore()
    const validationStore = useValidationStore()
    const speciesStore = useSpeciesStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(conjugateStore, "page", "get").mockReturnValue(1)
    vi.spyOn(conjugateStore, "listConjugates", "get").mockReturnValue([
      {
        id: 7,
        groupId: 10,
        tagId: 5,
        lotId: 3,
        tubeNumber: 12,
        status: 2,
        concentration: 1,
        description: null,
        customId: "c-7",
      },
    ] as any)

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(conjugateStore, "loadListQuery").mockResolvedValue([
      { id: 7 },
    ] as any)
    vi.spyOn(conjugateStore, "reloadListQuery").mockResolvedValue([] as any)

    vi.spyOn(lotStore, "getLot").mockReturnValue({ id: 3, cloneId: 1, number: "L-1" } as any)
    vi.spyOn(cloneStore, "getClone").mockReturnValue({
      id: 1,
      proteinId: 11,
      name: "Clone-1",
      reactivity: [21],
    } as any)
    vi.spyOn(proteinStore, "getProtein").mockReturnValue({ id: 11, name: "Protein-1" } as any)
    vi.spyOn(tagStore, "getTag").mockReturnValue({ id: 5, name: "Tag", mw: 42 } as any)
    vi.spyOn(validationStore, "cloneValidationMap", "get").mockReturnValue({
      1: [{ id: 900 }],
    } as any)
    vi.spyOn(speciesStore, "fetchByIds").mockResolvedValue([] as any)

    const tagId = ref(5)
    const api = useTagConjugates({ tagId })
    await api.reload()

    expect(loadListSpy).toHaveBeenCalled()
    expect(api.loading.value).toBe(false)
    expect(api.items.value).toEqual([
      expect.objectContaining({
        id: 7,
        tagId: 5,
        tagName: "Tag",
        lotNumber: "L-1",
        cloneName: "Clone-1",
        proteinName: "Protein-1",
      }),
    ])
  })
})
