import { computed, watch } from 'vue'
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { useUserStore } from '@/stores/user'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { UserView } from "@/modules/user/types"

export function useUsers() {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const userStore = useUserStore()
  const mainStore = useMainStore()

  async function loadUsers() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        userStore.resetListQuery()
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
        sf('User', 'group_id', 'eq', groupId),
        ...dynamicFilters
      )

      const order = createOrder('User', 'id', 'desc')
      await userStore.loadListQuery({
        groupId,
        filters: finalFilters,
        order,
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<UserView[]>(() =>
    userStore.listUsers.map(user => ({
        id: user.id,
        email: user.email,
        name: user.name,
        isActive: user.isActive,
        isAdmin: user.isAdmin,
        mfaEnabled: user.mfaEnabled,
        createdAt: user.createdAt,
        updatedAt: user.updatedAt,
      }))
  )

  watch(() => filterStore.filters, loadUsers, { deep: true })
  watch(() => userStore.page, loadUsers, { immediate: true })
  watch(() => userStore.limit, loadUsers)

  return {
    items,
    loading: computed(() => userStore.loading),
    reload: userStore.reloadListQuery,
  }
}
