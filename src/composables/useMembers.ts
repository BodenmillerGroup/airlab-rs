import { computed, watch } from 'vue'
import { unref, type MaybeRef } from "vue"
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useMemberStore } from '@/stores/member'
import { useUserStore } from '@/stores/user'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { MemberView } from "@/modules/member/types"

type UseMembersOptions = {
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function useMembers(options: UseMembersOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const memberStore = useMemberStore()
  const userStore = useUserStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Member', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      userName: 'name',
      role: 'role',
      isActive: 'is_active',
      allPanels: 'all_panels',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Member', 'id', 'desc')
    }

    const dir = first.order === 'asc' ? 'asc' : 'desc'
    return createOrder(first.key === 'userName' ? 'User' : 'Member', field, dir)
  }

  async function loadMembers() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        memberStore.resetListQuery()
        return
      }

      const dynamicFilters = globalFilterConfig
        .filter(cfg => {
          const val = filterStore.filters[cfg.key]
          return val !== '' && !(Array.isArray(val) && val.length === 0)
        })
        .map((cfg): SearchFilterInput => sf(
          cfg.table,
          cfg.field,
          cfg.op,
          filterStore.filters[cfg.key],
        ))

      const finalFilters = createFilters(
        sf('Member', 'group_id', 'eq', groupId),
        ...dynamicFilters
      )

      const order = buildOrder()
      await memberStore.loadListQuery({
        groupId,
        filters: finalFilters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<MemberView[]>(() =>
    memberStore.listMembers.map((member) => {
        const user = userStore.getUserById(member.userId)

        return {
          id: member.id,
          groupId: member.groupId,
          userId: member.userId,
          userName: user.name,
          role: member.role,
          activationKey: member.activationKey,
          isActive: member.isActive,
          allPanels: member.allPanels,
          createdAt: member.createdAt,
          updatedAt: member.updatedAt,
        }
      })
  )

  watch(() => filterStore.filters, loadMembers, { deep: true })
  watch(() => memberStore.page, loadMembers, { immediate: true })
  watch(() => memberStore.limit, loadMembers)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadMembers)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadMembers, { deep: true })

  return {
    items,
    loading: computed(() => memberStore.loading),
    reload: memberStore.reloadListQuery,
  }
}
