import { describe, it, expect, vi, beforeEach } from "vitest"

import { useValidations } from "@/composables/useValidations"

import { useFilterStore } from "@/stores/useFilterStore"
import { useGroupStore } from "@/stores/group"
import { useValidationStore } from "@/stores/validation"
import { useCloneStore } from "@/stores/clone"
import { useProteinStore } from "@/stores/protein"
import { useConjugateStore } from "@/stores/conjugate"
import { useSpeciesStore } from "@/stores/species"
import { useUserStore } from "@/stores/user"
import { useMemberStore } from "@/stores/member"
import { useLotStore } from "@/stores/lot"
import { useMainStore } from "@/stores/main"

describe("useValidations", () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it("loads validations and builds ValidationView", async () => {
    const filterStore = useFilterStore()
    const groupStore = useGroupStore()
    const validationStore = useValidationStore()
    const cloneStore = useCloneStore()
    const proteinStore = useProteinStore()
    const conjugateStore = useConjugateStore()
    const speciesStore = useSpeciesStore()
    const userStore = useUserStore()
    const memberStore = useMemberStore()
    const lotStore = useLotStore()
    const mainStore = useMainStore()

    vi.spyOn(groupStore, "activeGroupId", "get").mockReturnValue(10)
    vi.spyOn(validationStore, "page", "get").mockReturnValue(1)
    vi.spyOn(validationStore, "listValidations", "get").mockReturnValue([
      {
        id: 21,
        groupId: 10,
        createdBy: 51,
        cloneId: 5,
        lotId: 3,
        conjugateId: 4,
        speciesId: 6,
        application: "IHC",
        positiveControl: "",
        negativeControl: "",
        incubationConditions: "",
        concentration: "",
        concentrationUnit: "",
        tissue: "",
        fixation: "",
        fixationNotes: "",
        notes: "",
        status: 1,
        antigenRetrievalType: "",
        antigenRetrievalTime: "",
        antigenRetrievalTemperature: "",
        saponin: "",
        saponinConcentration: "",
        methanolTreatment: "",
        methanolTreatmentConcentration: "",
        surfaceStaining: "",
        surfaceStainingConcentration: "",
        meta: {},
        isArchived: false,
        createdAt: "now",
        updatedAt: "now",
      },
    ] as any)

    filterStore.filters = {} as any

    vi.spyOn(mainStore, "checkApiError").mockResolvedValue()
    const loadListSpy = vi.spyOn(validationStore, "loadListQuery").mockResolvedValue([
      { id: 21 },
    ] as any)
    vi.spyOn(validationStore, "reloadListQuery").mockResolvedValue([] as any)

    vi.spyOn(cloneStore, "getClone").mockReturnValue({ id: 5, name: "Clone", proteinId: 1 } as any)
    vi.spyOn(proteinStore, "getProtein").mockReturnValue({ id: 1, name: "Protein" } as any)
    vi.spyOn(conjugateStore, "getConjugate").mockReturnValue({ id: 4, tubeNumber: "T-1" } as any)
    vi.spyOn(speciesStore, "getSpecies").mockReturnValue({ id: 6, name: "Human" } as any)
    vi.spyOn(lotStore, "getLot").mockReturnValue({ id: 3, name: "Lot", number: "L-1" } as any)
    vi.spyOn(memberStore, "getMemberById").mockReturnValue({ id: 51, userId: 2 } as any)
    vi.spyOn(userStore, "getUserById").mockReturnValue({ id: 2, name: "Test User" } as any)

    const validations = useValidations()
    await validations.reload()

    expect(loadListSpy.mock.calls.length).toBeGreaterThan(0)
    expect(validations.loading.value).toBe(false)
    expect(validations.items.value).toEqual([
      expect.objectContaining({
        id: 21,
        groupId: 10,
        cloneId: 5,
        proteinName: "Protein",
        lotName: "Lot",
        lotNumber: "L-1",
        userName: "Test User",
        speciesName: "Human",
        tubeNumber: "T-1",
      }),
    ])
  })
})
