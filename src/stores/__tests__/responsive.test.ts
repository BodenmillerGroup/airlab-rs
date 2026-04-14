import { describe, it, expect, beforeEach } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { useResponsiveStore } from "@/stores/responsive"

beforeEach(() => {
  setActivePinia(createPinia())
})

describe("responsive store", () => {
  it("updates width and height", () => {
    const store = useResponsiveStore()

    expect(store.width).toBe(0)
    expect(store.height).toBe(0)

    store.setResponsive({ width: 1200, height: 800 })

    expect(store.width).toBe(1200)
    expect(store.height).toBe(800)
  })
})
