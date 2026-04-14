import { ref, watch } from "vue"

export function useDebouncedSearch<T>(
  fetchFn: (query: string) => Promise<T[]>,
  delay = 300
) {
  const items = ref<T[]>([])
  const loading = ref(false)
  const search = ref("")
  let timeout: ReturnType<typeof setTimeout>

  watch(items, (newItems) => {
    if (search.value && newItems.length > 0) {
      search.value += ' '
      search.value = search.value.trim()
    }
  })
  watch(search, (query) => {
    clearTimeout(timeout)

    if (!query) {
      items.value = []
      return
    }

    timeout = setTimeout(async () => {
      loading.value = true
      try {
        const result = await fetchFn(query)
        items.value = result || []
      } catch (e) {
        console.error("Debounced search error", e)
      } finally {
        loading.value = false
      }
    }, delay)
  })

  const doSearch = (val: string) => {
    search.value = val
  }

  return { items, loading, search, doSearch }
}
