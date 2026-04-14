import { describe, it, expect, beforeEach, vi } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { useSettingsStore } from "@/stores/settings"

beforeEach(() => {
  setActivePinia(createPinia())
})

describe("settings store", () => {
  it("clears all caches when resetSettings is called", async () => {
    const deleteMock = vi.fn()
    const keysMock = vi.fn().mockResolvedValue(["a", "b", "c"])

      // mock global caches API
      ; (globalThis as any).caches = {
        keys: keysMock,
        delete: deleteMock,
      }

    const store = useSettingsStore()
    await store.resetSettings()

    expect(keysMock).toHaveBeenCalled()
    expect(deleteMock).toHaveBeenCalledTimes(3)
    expect(deleteMock).toHaveBeenCalledWith("a")
    expect(deleteMock).toHaveBeenCalledWith("b")
    expect(deleteMock).toHaveBeenCalledWith("c")
  })
})
