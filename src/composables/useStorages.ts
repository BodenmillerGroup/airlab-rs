import { computed, watch } from 'vue'
import { unref, type MaybeRef } from 'vue'
import { useStorageStore } from '@/stores/storage'
import { useMainStore } from '@/stores/main'
import { createOrder } from '@/modules/json/api'

type UseStoragesOptions = {
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useStorages(options: UseStoragesOptions = {}) {
  const storageStore = useStorageStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Storage', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      name: 'name',
      type: 'type',
      location: 'location',
      temperature_c: 'temperature_c',
      active: 'active',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Storage', 'id', 'desc')
    }

    return createOrder('Storage', field, first.order === 'asc' ? 'asc' : 'desc')
  }

  async function loadStorages() {
    try {
      await storageStore.loadListQuery({
        groupId: -1,
        filters: [],
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order: buildOrder(),
      })
    } catch (error) {
      if (typeof mainStore.checkApiError === 'function') {
        await mainStore.checkApiError(error)
      }
    }
  }

  watch(() => storageStore.page, loadStorages, { immediate: true })
  watch(() => storageStore.limit, loadStorages)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadStorages)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadStorages, { deep: true })

  return {
    items: computed(() => storageStore.listStorages),
    loading: computed(() => storageStore.loading),
    reload: storageStore.reloadListQuery,
  }
}
