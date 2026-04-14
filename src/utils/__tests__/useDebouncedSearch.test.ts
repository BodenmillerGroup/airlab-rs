import { describe, it, expect, vi, beforeEach, afterEach } from "vitest"
import { useDebouncedSearch } from "@/utils/useDebouncedSearch"
import { nextTick } from "vue"

describe("useDebouncedSearch", () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  it("fetches after debounce delay", async () => {
    const fetchSpy = vi.fn().mockResolvedValue(["a", "b"])

    const { doSearch, items } = useDebouncedSearch(fetchSpy, 300)

    doSearch("abc")

    expect(fetchSpy).not.toHaveBeenCalled()

    await vi.advanceTimersByTimeAsync(300)
    await nextTick()

    expect(fetchSpy).toHaveBeenCalledOnce()
    expect(fetchSpy).toHaveBeenCalledWith("abc")
    expect(items.value).toEqual(["a", "b"])
  })

  it("sets loading while fetching", async () => {
    const fetchSpy = vi.fn(
      () => new Promise(resolve => setTimeout(() => resolve(["x"]), 100))
    )

    const { doSearch, loading } = useDebouncedSearch(fetchSpy as any, 300)


    doSearch("abc")

    await vi.advanceTimersByTimeAsync(300)
    await nextTick()

    expect(loading.value).toBe(true)

    await vi.advanceTimersByTimeAsync(100)
    await nextTick()

    expect(loading.value).toBe(false)
  })

  it("cancels previous call if query changes", async () => {
    const fetchSpy = vi.fn().mockResolvedValue([])

    const { doSearch } = useDebouncedSearch(fetchSpy, 300)

    doSearch("a")
    await vi.advanceTimersByTimeAsync(150)

    doSearch("b")
    await vi.advanceTimersByTimeAsync(300)
    await nextTick()

    expect(fetchSpy).toHaveBeenCalledOnce()
    expect(fetchSpy).toHaveBeenCalledWith("b")
  })

  it("clears items immediately if query is empty", async () => {
    const fetchSpy = vi.fn().mockResolvedValue(["x"])

    const { doSearch, items } = useDebouncedSearch(fetchSpy, 300)

    doSearch("abc")
    await vi.advanceTimersByTimeAsync(300)
    await nextTick()

    expect(items.value.length).toBe(1)

    doSearch("")
    await nextTick()

    expect(items.value.length).toBe(0)
  })

  it("handles fetch errors gracefully", async () => {
    const fetchSpy = vi.fn().mockRejectedValue(new Error("boom"))

    const { doSearch, loading, items } = useDebouncedSearch(fetchSpy, 300)

    doSearch("abc")

    await vi.advanceTimersByTimeAsync(300)
    await nextTick()

    expect(fetchSpy).toHaveBeenCalledOnce()
    expect(loading.value).toBe(false)
    expect(items.value).toEqual([])
  })
})

