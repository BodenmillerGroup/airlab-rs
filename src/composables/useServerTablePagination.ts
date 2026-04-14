import { computed, type ComputedRef, type Ref, type WritableComputedRef } from 'vue'

type UseServerTablePaginationOptions = {
  itemsPerPageOptions?: number[]
}

type UseServerTablePaginationResult = {
  tablePage: WritableComputedRef<number>
  tableItemsPerPage: WritableComputedRef<number>
  tableFooterProps: ComputedRef<{
    showFirstLastPage: boolean
    showCurrentPage: boolean
    itemsPerPageOptions: number[]
  }>
}

export function useServerTablePagination(
  page: Ref<number>,
  itemsPerPage: Ref<number>,
  options: UseServerTablePaginationOptions = {},
): UseServerTablePaginationResult {
  const itemsPerPageOptions = options.itemsPerPageOptions ?? [10, 25, 50, 100]

  const tablePage = computed<number>({
    get: () => page.value,
    set: (value) => {
      const nextPage = Number(value)
      if (Number.isInteger(nextPage) && nextPage > 0 && nextPage !== page.value) {
        page.value = nextPage
      }
    },
  })

  const tableItemsPerPage = computed<number>({
    get: () => itemsPerPage.value,
    set: (value) => {
      const nextItemsPerPage = Number(value)
      if (
        Number.isInteger(nextItemsPerPage) &&
        nextItemsPerPage > 0 &&
        nextItemsPerPage !== itemsPerPage.value
      ) {
        itemsPerPage.value = nextItemsPerPage
        page.value = 1
      }
    },
  })

  const tableFooterProps = computed(() => ({
    showFirstLastPage: false,
    showCurrentPage: true,
    itemsPerPageOptions,
  }))

  return {
    tablePage,
    tableItemsPerPage,
    tableFooterProps,
  }
}
