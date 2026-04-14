import { computed, watch } from 'vue'
import { useGroupStore } from '@/stores/group'
import { useMemberStore } from '@/stores/member'
import { useMainStore } from '@/stores/main'
import { useFilterStore } from '@/stores/useFilterStore'

import type { Ref } from 'vue';
import type { GroupView } from '@/modules/group/types'

export function useGroups(userId: Ref<number | undefined>) {
  const groupStore = useGroupStore()
  const memberStore = useMemberStore()
  const mainStore = useMainStore()
  const filterStore = useFilterStore()

  async function loadGroups() {
    try {
      await groupStore.loadListQuery({
        userId: userId.value,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<GroupView[]>(() =>
    [...groupStore.listGroups]
      .sort((a, b) => a.id - b.id)
      .map((group) => {
      const members = memberStore.getByGroupId(group.id)

      return {
        id: group.id,
        name: group.name,
        institution: group.institution,
        url: group.url,
        isOpen: group.isOpen,
        meta: group.meta,
        createdAt: group.createdAt,
        members: members.filter(m => m.groupId === group.id).map(m => ({
          id: m.id,
          role: m.role,
          userId: m.userId,
        })),
      }
      })
  )

  watch(
    () => filterStore.filters,
    () => {
      loadGroups()
    },
    { deep: true, immediate: true }
  )

  return {
    items,
    loading: computed(() => groupStore.loading),
    reload: groupStore.reloadListQuery,
  }
}
