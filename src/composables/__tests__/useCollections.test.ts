import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { ref, nextTick } from 'vue'
import { flushPromises } from '@vue/test-utils'

import { useCollections } from '@/composables/useCollections'
import { useCollectionStore } from '@/stores/collection'
import { useMemberStore } from '@/stores/member'
import { useUserStore } from '@/stores/user'

describe('useCollections', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('loads collections and resolves created by to user name', async () => {
    const collectionStore = useCollectionStore()
    const memberStore = useMemberStore()
    const userStore = useUserStore()

    vi.spyOn(collectionStore, 'page', 'get').mockReturnValue(1)
    vi.spyOn(collectionStore, 'limit', 'get').mockReturnValue(50)
    vi.spyOn(collectionStore, 'loading', 'get').mockReturnValue(false)
    vi.spyOn(collectionStore, 'listCollections', 'get').mockReturnValue([
      {
        id: 10,
        name: 'Collection A',
        description: 'Test',
        created_at: '2024-01-01T00:00:00Z',
        created_by: 21,
      },
    ] as any)

    const loadListSpy = vi.spyOn(collectionStore, 'loadListQuery').mockResolvedValue([
      {
        id: 10,
        name: 'Collection A',
        description: 'Test',
        created_at: '2024-01-01T00:00:00Z',
        created_by: 21,
      },
    ] as any)
    vi.spyOn(collectionStore, 'reloadListQuery').mockResolvedValue([] as any)

    const fetchMembersSpy = vi.spyOn(memberStore, 'fetchByIds').mockResolvedValue([] as any)
    vi.spyOn(memberStore, 'getMemberById').mockReturnValue({ id: 21, userId: 7 } as any)

    const fetchUsersSpy = vi.spyOn(userStore, 'fetchByIds').mockResolvedValue([] as any)
    vi.spyOn(userStore, 'getUserById').mockReturnValue({ id: 7, name: 'Test User' } as any)

    const collections = useCollections({ sortBy: ref([{ key: 'id', order: 'desc' }]) })
    await nextTick()
    await flushPromises()

    expect(loadListSpy).toHaveBeenCalled()
    expect(fetchMembersSpy).toHaveBeenCalledWith([21])
    expect(fetchUsersSpy).toHaveBeenCalledWith([7])
    expect(collections.items.value).toEqual([
      {
        id: 10,
        name: 'Collection A',
        description: 'Test',
        created_at: '2024-01-01T00:00:00Z',
        created_by: 21,
        createdByName: 'Test User',
      },
    ])
  })
})
