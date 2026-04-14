import { computed, watch } from 'vue'
import { unref, type MaybeRef } from 'vue'
import { useCollectionStore } from '@/stores/collection'
import { useMemberStore } from '@/stores/member'
import { useUserStore } from '@/stores/user'
import { useMainStore } from '@/stores/main'
import { createOrder } from '@/modules/json/api'

type UseCollectionsOptions = {
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useCollections(options: UseCollectionsOptions = {}) {
  const collectionStore = useCollectionStore()
  const memberStore = useMemberStore()
  const userStore = useUserStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Collection', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      name: 'name',
      description: 'description',
      created_at: 'created_at',
      created_by: 'created_by',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Collection', 'id', 'desc')
    }

    return createOrder('Collection', field, first.order === 'asc' ? 'asc' : 'desc')
  }

  async function loadCollections() {
    try {
      const collections = await collectionStore.loadListQuery({
        groupId: -1,
        filters: [],
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order: buildOrder(),
      })

      const createdByIds = (collections ?? [])
        .map((collection) => collection.created_by)
        .filter((id): id is number => typeof id === 'number')

      await memberStore.fetchByIds(createdByIds)

      const userIds = createdByIds
        .map((id) => memberStore.getMemberById(id)?.userId)
        .filter((id): id is number => typeof id === 'number')

      await userStore.fetchByIds(userIds)
    } catch (error) {
      if (typeof mainStore.checkApiError === 'function') {
        await mainStore.checkApiError(error)
      }
    }
  }

  watch(() => collectionStore.page, loadCollections, { immediate: true })
  watch(() => collectionStore.limit, loadCollections)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadCollections)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadCollections, { deep: true })

  return {
    items: computed(() =>
      collectionStore.listCollections.map((collection) => {
        const member = memberStore.getMemberById(collection.created_by)
        const user = member ? userStore.getUserById(member.userId) : undefined

        return {
          ...collection,
          createdByName: user?.name ?? '—',
        }
      })
    ),
    loading: computed(() => collectionStore.loading),
    reload: collectionStore.reloadListQuery,
  }
}
