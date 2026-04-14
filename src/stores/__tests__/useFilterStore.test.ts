import { describe, it, expect, beforeEach } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { useFilterStore } from "@/stores/useFilterStore"
import { globalFilterConfig } from "@/filters/globalFilterConfig"

beforeEach(() => {
  setActivePinia(createPinia())
})

describe("filter store", () => {
  it("initializes filters from globalFilterConfig", () => {
    const store = useFilterStore()

    for (const cfg of globalFilterConfig) {
      if (cfg.type === "select") {
        expect(store.filters[cfg.key]).toEqual([])
      } else {
        expect(store.filters[cfg.key]).toBe("")
      }
    }
  })

  it("sets filters", () => {
    const store = useFilterStore()

    store.setFilter("test_key", 123)

    expect(store.filters["test_key"]).toBe(123)
  })

  it("resets all filters", () => {
    const store = useFilterStore()

    // dirty everything
    for (const cfg of globalFilterConfig) {
      store.filters[cfg.key] = "DIRTY"
    }

    store.reset()

    for (const cfg of globalFilterConfig) {
      if (cfg.type === "select") {
        expect(store.filters[cfg.key]).toEqual([])
      } else {
        expect(store.filters[cfg.key]).toBe("")
      }
    }
  })

  it("resets only selected keys", () => {
    const store = useFilterStore()

    const [first, second] = globalFilterConfig

    store.filters[first.key] = "DIRTY"
    store.filters[second.key] = "DIRTY"

    store.reset([first.key])

    if (first.type === "select") {
      expect(store.filters[first.key]).toEqual([])
    } else {
      expect(store.filters[first.key]).toBe("")
    }

    expect(store.filters[second.key]).toBe("DIRTY")
  })
})
