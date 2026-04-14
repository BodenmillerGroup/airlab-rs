import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { api } from '@/modules/member/api'
import { useMainStore } from '@/stores/main'
import { useUserStore } from '@/stores/user'

import type {
  MemberDto,
  CreateMemberDto,
  UpdateMemberDto,
} from '@/modules/member/types'

type Id = number

type MemberListQuery = {
  groupId: number
  filters: SearchFilter[]
  order: SearchOrder
  globalFilter?: string
}

export const useMemberStore = defineStore('member', () => {
  // 🧪 State
  const ids = ref<Id[]>([])
  const listIds = ref<Id[]>([])
  const entities = ref<Record<Id, MemberDto>>({})
  const page = ref(1)
  const limit = ref(50)
  const total = ref(0)
  const loading = ref(false)

  const mainStore = useMainStore();
  const userStore = useUserStore()
  const activeListQuery = ref<MemberListQuery | null>(null)

  // 🧠 Getters
  const members = computed(() => ids.value.map(id => entities.value[id]))
  const listMembers = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((member): member is MemberDto => member !== undefined)
  )
  const getMemberById = (id: Id) => entities.value[id]
  const hasMember = (id: Id) => id in entities.value

  const getMembersForGroup = (groupId: Id): MemberDto[] =>
    ids.value
      .map(id => entities.value[id])
      .filter(m => m?.groupId === groupId) as MemberDto[]

  // 🔧 Mutations
  function setEntities(payload: MemberDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function deleteEntity(id: Id) {
    ids.value = ids.value.filter(x => x !== id)
    delete entities.value[id]
    total.value = ids.value.length
  }

  function getByGroupId(groupId: number) {
    return ids.value
      .map(id => entities.value[id])
      .filter((m): m is MemberDto => m?.groupId === groupId);
  }

  function reset() {
    ids.value = []
    listIds.value = []
    entities.value = {}
    page.value = 1
    limit.value = 50
    total.value = 0
    loading.value = false
    activeListQuery.value = null
  }

  function resetListQuery() {
    listIds.value = []
    activeListQuery.value = null
    loading.value = false
  }

  async function fetchByIds(itemIds: readonly number[], forceRefresh = false) {
    const uniqueIds = [...new Set(itemIds)].filter((x): x is number => Number.isInteger(x));

    const targetIds = forceRefresh
      ? uniqueIds
      : uniqueIds.filter(id => !(id in entities.value));
    if (targetIds.length === 0) return [];

    try {
      const res = await rpc<{ items: MemberDto[] }>({
        operation: 'Get',
        return_type: 'Member',
        filters: [{ field: 'id', op: 'in', value: targetIds }],
      });

      setEntities(res.items);
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function ensureListRelations(groupId: number, itemIds: readonly number[]) {
    if (groupId <= 0) {
      return
    }

    const visibleMembers = itemIds
      .map((id) => entities.value[id])
      .filter((member): member is MemberDto => member !== undefined)

    const userIds = visibleMembers
      .map((member) => member.userId)
      .filter((id): id is number => typeof id === 'number')

    await userStore.fetchByIds(userIds)
  }

  async function loadListQuery(query: MemberListQuery) {
    activeListQuery.value = query
    loading.value = true
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Member',
        filters: query.filters,
        limit: limit.value,
        page: page.value,
        order: query.order,
        global_filter: query.globalFilter,
      })

      if (items.length > 0) {
        // Member rows are edited in-place elsewhere; refresh visible rows instead of
        // only hydrating missing ids so persisted cache does not mask backend updates.
        await fetchByIds(items, true)
      }

      listIds.value = items
      total.value = search_total

      await ensureListRelations(query.groupId, items)

      return listMembers.value
    } catch (error) {
      mainStore.checkApiError(error)
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

  async function search(filters: SearchFilter[], order: SearchOrder) {
    return loadListQuery({
      groupId: -1,
      filters,
      order,
    })
  }

  // 📡 API Actions
  async function getGroupMembers(groupId: Id) {
    loading.value = true
    try {
      const data = await api.getGroupMembers(groupId)
      setEntities(data)
      return data
    } catch (error) {
      await mainStore.checkApiError(error)
    } finally {
      loading.value = false
    }
  }

  async function getMember(id: Id) {
    loading.value = true
    try {
      const data = await rpc<MemberDto>({
        operation: 'Get',
        return_type: 'Member',
        payload: id,
      })
      setEntities([data])
      return data
    } catch (error) {
      await mainStore.checkApiError(error)
    } finally {
      loading.value = false
    }
  }

  async function createMember(payload: CreateMemberDto) {
    try {
      const data = await api.createMember(payload)
      const refreshed = await getMember(data.id)
      mainStore.addNotification({ content: 'Group member successfully created', color: 'success' })
      return refreshed ?? data
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  async function updateMember(payload: { id: Id; data: UpdateMemberDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Member',
        id: payload.id,
        payload: payload.data,
      })
      const data = await getMember(payload.id)
      mainStore.addNotification({ content: 'Group member successfully updated', color: 'success' })
      return data
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  async function removeMember(id: Id) {
    try {
      await api.deleteMember(id)
      deleteEntity(id)
      mainStore.addNotification({ content: 'Group member successfully deleted', color: 'success' })
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  // 🔍 UI Helpers

  function setPage(p: number) {
    page.value = p
  }

  function setLimit(l: number) {
    limit.value = l
  }

  return {
    // state
    ids,
    listIds,
    entities,
    page,
    limit,
    total,
    loading,

    // getters
    members,
    listMembers,
    getMemberById,
    hasMember,
    getMembersForGroup,

    // actions
    setEntities,
    deleteEntity,
    reset,
    resetListQuery,

    fetchByIds,
    search,
    loadListQuery,
    reloadListQuery,
    getByGroupId,
    getGroupMembers,
    getMember,
    createMember,
    updateMember,
    removeMember,

    setPage,
    setLimit,
  }
}, { persist: true })
