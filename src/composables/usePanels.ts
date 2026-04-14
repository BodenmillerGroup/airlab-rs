import { computed, watch } from 'vue'
import { unref, type MaybeRef } from "vue"
import { useFilterStore } from '@/stores/useFilterStore'
import { useGroupStore } from '@/stores/group'
import { usePanelStore } from '@/stores/panel'
import { usePanelElementStore } from '@/stores/panel_element'
import { useUserStore } from '@/stores/user'
import { useMemberStore } from '@/stores/member'
import { useMainStore } from '@/stores/main'

import { globalFilterConfig } from '@/filters/globalFilterConfig'
import { createFilters, createOrder, sf } from '@/modules/json/api'
import type { SearchFilterInput } from '@/modules/json/api'
import type { PanelView } from "@/modules/panel/types"


type UsePanelOptions = {
  conjugateId?: MaybeRef<number | undefined>
  showAllPanels?: MaybeRef<boolean | undefined>
  globalFilter?: MaybeRef<string | undefined>
  sortBy?: MaybeRef<Array<{ key: string; order?: 'asc' | 'desc' }>>
}

export function usePanels(options: UsePanelOptions = {}) {
  const filterStore = useFilterStore()
  const groupStore = useGroupStore()
  const panelStore = usePanelStore()
  const panelElementStore = usePanelElementStore()
  const userStore = useUserStore()
  const memberStore = useMemberStore()
  const mainStore = useMainStore()

  function buildOrder() {
    const sortBy = unref(options.sortBy)
    const first = Array.isArray(sortBy) ? sortBy[0] : undefined
    if (!first?.key) {
      return createOrder('Panel', 'id', 'desc')
    }

    const keyMap: Record<string, string> = {
      id: 'id',
      name: 'name',
      description: 'description',
      isFluorophore: 'is_fluorophore',
      isLocked: 'is_locked',
      application: 'application',
      user: 'name',
      updatedAt: 'updated_at',
    }

    const field = keyMap[first.key]
    if (!field) {
      return createOrder('Panel', 'id', 'desc')
    }

    const dir = first.order === 'asc' ? 'asc' : 'desc'
    return createOrder(first.key === 'user' ? 'User' : 'Panel', field, dir)
  }

  async function loadPanels() {
    try {
      const groupId = groupStore.activeGroupId
      if (typeof groupId !== "number") {
        panelStore.resetListQuery()
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


      const baseFilters: SearchFilterInput[] = [
        sf('Panel', 'group_id', 'eq', groupId),
      ]
      const conjugateId = unref(options.conjugateId)
      if (typeof conjugateId === "number") {
        const panelElements = await panelElementStore.getByConjugateId(conjugateId)

        const panelIds = [...new Set(
          panelElements
            .map((element) => element.panelId)
            .filter((id): id is number => typeof id === 'number')
        )]

        if (panelIds.length === 0) {
          panelStore.resetListQuery()
          return
        }

        baseFilters.push(sf('Panel', 'id', 'in', panelIds))
      }
      const finalFilters = createFilters(
        ...baseFilters,
        ...dynamicFilters
      )

      await panelStore.loadListQuery({
        groupId,
        filters: finalFilters,
        globalFilter: unref(options.globalFilter)?.trim() || undefined,
        order: buildOrder(),
        showAll: Boolean(unref(options.showAllPanels)),
      })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  const items = computed<PanelView[]>(() =>
    panelStore.listPanels.map((panel) => {
        const member = memberStore.getMemberById(panel.createdBy)
        const user = member ? userStore.getUserById(member.userId) : undefined
        return {
          id: panel.id,
          groupId: panel.groupId,
          createdBy: panel.createdBy,
          name: panel.name,
          description: panel.description,
          isFluorophore: panel.isFluorophore,
          isLocked: panel.isLocked,
          application: panel.application,
          meta: panel.meta,
          isArchived: panel.isArchived,
          createdAt: panel.createdAt,
          updatedAt: panel.updatedAt,
          userName: user?.name || '',
          userId: panel.createdBy,
        }
      })
  )

  watch(() => filterStore.filters, loadPanels, { deep: true })
  watch(() => panelStore.page, loadPanels, { immediate: true })
  watch(() => panelStore.limit, loadPanels)
  watch(() => options.conjugateId ? unref(options.conjugateId) : undefined, loadPanels)
  watch(() => options.showAllPanels ? unref(options.showAllPanels) : undefined, loadPanels)
  watch(() => options.globalFilter ? unref(options.globalFilter) : undefined, loadPanels)
  watch(() => options.sortBy ? unref(options.sortBy) : undefined, loadPanels, { deep: true })

  return {
    items,
    loading: computed(() => panelStore.loading),
    reload: panelStore.reloadListQuery,
  }
}
