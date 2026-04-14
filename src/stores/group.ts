import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { createFilters, rpc, sf } from '@/modules/json/api'
import { useMainStore } from '@/stores/main'
import { saveAs } from 'file-saver'
import { useMemberStore } from '@/stores/member'

import type { GroupDto, CreateGroupDto, UpdateGroupDto } from '@/modules/group/types'
import type { MemberDto } from '@/modules/member/types'
import { api } from '@/modules/group/api' // fallback for files

type Id = number

type GroupListQuery = {
  userId?: number
}

export const useGroupStore = defineStore('group', () => {
  // 📦 State
  const ids = ref<Id[]>([])
  const listIds = ref<Id[]>([])
  const entities = ref<Record<Id, GroupDto>>({})
  const activeGroupId = ref<Id | undefined>()
  const myMember = ref<MemberDto | null>(null)

  const page = ref(1)
  const limit = ref(50)
  const total = ref(0)
  const search = ref('')
  const loading = ref(false)

  const main = useMainStore()
  const memberStore = useMemberStore()
  const activeListQuery = ref<GroupListQuery | null>(null)

  // 🧠 Getters
  const groups = computed(() => ids.value.map(id => entities.value[id]))
  const listGroups = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((group): group is GroupDto => group !== undefined)
  )
  const getGroupById = (id: Id) => entities.value[id]
  const activeGroup = computed(() => (activeGroupId.value != null ? entities.value[activeGroupId.value] : undefined))
  const groupRole = computed(() => myMember.value?.role ?? 0)
  const isGroupAdmin = computed(() => groupRole.value >= 100)
  const allPanels = computed(() => Boolean(myMember.value?.allPanels))
  const hasActiveGroup = computed(() => activeGroupId.value != null)

  // 🧱 Mutations
  function setEntities(payload: GroupDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(group: GroupDto) {
    if (!ids.value.includes(group.id)) ids.value.push(group.id)
    entities.value[group.id] = group
    total.value = ids.value.length
  }

  function updateEntity(group: GroupDto) {
    entities.value[group.id] = group
  }

  function deleteEntity(id: Id) {
    ids.value = ids.value.filter(x => x !== id)
    delete entities.value[id]
    total.value = ids.value.length
  }

  function setActiveGroupId(id?: Id) {
    activeGroupId.value = id
  }

  function setMyMember(member: MemberDto | null) {
    myMember.value = member
  }

  function reset() {
    ids.value = []
    listIds.value = []
    entities.value = {}
    activeGroupId.value = undefined
    myMember.value = null
    page.value = 1
    limit.value = 50
    total.value = 0
    search.value = ''
    loading.value = false
    activeListQuery.value = null
  }

  function resetListQuery() {
    listIds.value = []
    activeListQuery.value = null
    loading.value = false
  }

  // 🚀 RPC Actions
  async function getGroups() {
    loading.value = true
    try {
      const res = await rpc<{ items: GroupDto[] }>({
        operation: 'Get',
        return_type: 'Group',
      })
      setEntities(res.items)
      return res.items
    } catch (error) {
      main.checkApiError(error)
    } finally {
      loading.value = false
    }
  }

  async function ensureListRelations(userId?: number) {
    if (typeof userId !== 'number') {
      return
    }

    const groupIds = listIds.value
    if (groupIds.length === 0) {
      return
    }

    await memberStore.loadListQuery({
      groupId: -1,
      filters: createFilters(
        sf('Member', 'group_id', 'in', groupIds),
        sf('Member', 'user_id', 'eq', userId),
        sf('Member', 'is_active', 'eq', true),
      ),
      order: { table: 'Member', field: 'id', direction: 'asc' },
    })
  }

  async function loadListQuery(query: GroupListQuery = {}) {
    activeListQuery.value = query
    loading.value = true
    try {
      const groups = await rpc<{ items: GroupDto[] }>({
        operation: 'Get',
        return_type: 'Group',
      })
      setEntities(groups.items)
      listIds.value = groups.items.map((group) => group.id)
      await ensureListRelations(query.userId)
      return listGroups.value
    } catch (error) {
      main.checkApiError(error)
      return []
    } finally {
      loading.value = false
    }
  }

  async function reloadListQuery() {
    if (!activeListQuery.value) {
      return []
    }

    return loadListQuery(activeListQuery.value)
  }

  async function getGroup(id: Id) {
    loading.value = true
    try {
      const group = await rpc<GroupDto>({
        operation: 'Get',
        return_type: 'Group',
        payload: id,
      })
      setEntity(group)
      return group
    } catch (error) {
      main.checkApiError(error)
    } finally {
      loading.value = false
    }
  }

  async function createGroup(payload: CreateGroupDto) {
    try {
      const group = await rpc<{ id?: Id } | GroupDto>({
        operation: 'Insert',
        return_type: 'Group',
        payload: payload,
      })
      const id = (group as GroupDto)?.id
      const entity = typeof id === 'number' ? await getGroup(id) : undefined
      main.addNotification({ content: 'Group successfully created', color: 'success' })
      return entity
    } catch (error) {
      main.checkApiError(error)
    }
  }

  async function updateGroup({ id, data }: { id: Id; data: UpdateGroupDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Group',
        id,
        payload: data,
      })
      const updated = await getGroup(id)
      main.addNotification({ content: 'Group successfully updated', color: 'success' })
      return updated
    } catch (error) {
      main.checkApiError(error)
    }
  }

  async function deleteGroup(id: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Group',
        id,
      })
      deleteEntity(id)
      main.addNotification({ content: 'Group successfully deleted', color: 'success' })
    } catch (error) {
      main.checkApiError(error)
    }
  }

  async function joinGroup(id: Id) {
    try {
      const result = await api.joinGroup(id)
      main.addNotification({
        content: 'Request to join the group submitted',
        color: 'success',
      })
      return result
    } catch (error) {
      main.checkApiError(error)
    }
  }

  async function getMyMember(groupId: Id) {
    loading.value = true
    try {
      const data = await api.getMyMember(groupId)
      setMyMember(data)
      return data
    } catch (error) {
      main.checkApiError(error)
    } finally {
      loading.value = false
    }
  }

  // 📤 File export / import via REST
  async function exportGroupData(id: Id, format: 'json' | 'csv') {
    try {
      const blob = await api.exportGroupData(id, format)
      saveAs(blob, `group_${id}.${format}`)
      main.addNotification({ content: 'Group data successfully exported', color: 'success' })
    } catch (error) {
      main.checkApiError(error)
    }
  }

  async function exportAllData(format: 'json' | 'csv') {
    try {
      const blob = await api.exportAllData(format)
      saveAs(blob, `airlab.${format}`)
      main.addNotification({ content: 'All data successfully exported', color: 'success' })
    } catch (error) {
      main.checkApiError(error)
    }
  }

  async function importGroupData(formData: FormData) {
    try {
      const group = await api.importGroupData(formData)
      setEntity(group)
      main.addNotification({ content: 'Group data successfully imported', color: 'success' })
      return group
    } catch (error) {
      main.checkApiError(error)
    }
  }

  async function importAllData(formData: FormData) {
    try {
      const groups = await api.importAllData(formData)
      setEntities(groups)
      main.addNotification({ content: 'All data successfully imported', color: 'success' })
      return groups
    } catch (error) {
      main.checkApiError(error)
    }
  }

  // 🔍 Pagination helpers
  function setSearch(value: string) {
    search.value = value
    page.value = 1
  }

  function setPage(value: number) {
    page.value = value
  }

  function setLimit(value: number) {
    limit.value = value
  }

  return {
    // State
    ids,
    listIds,
    entities,
    activeGroupId,
    myMember,
    page,
    limit,
    total,
    search,
    loading,

    // Getters
    groups,
    listGroups,
    getGroupById,
    activeGroup,
    groupRole,
    isGroupAdmin,
    allPanels,
    hasActiveGroup,

    // Mutations
    setEntity,
    updateEntity,
    deleteEntity,
    setActiveGroupId,
    setMyMember,
    reset,
    resetListQuery,

    // RPC actions
    getGroups,
    loadListQuery,
    reloadListQuery,
    getGroup,
    createGroup,
    updateGroup,
    deleteGroup,
    joinGroup,

    // Member
    getMyMember,

    // Export / Import
    exportGroupData,
    exportAllData,
    importGroupData,
    importAllData,

    // Pagination
    setSearch,
    setPage,
    setLimit,
  }
})
