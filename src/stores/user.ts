// src/stores/user.ts
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { api } from '@/modules/user/api'
import { useMainStore } from '@/stores/main'

import type {
  UserDto,
  CreateUserDto,
  UpdateUserDto,
} from '@/modules/user/types'

type Id = number

type UserListQuery = {
  groupId: number
  filters: SearchFilter[]
  order: SearchOrder
}

export const useUserStore = defineStore('user', () => {
  // 🧪 State
  const ids = ref<Id[]>([])
  const listIds = ref<Id[]>([])
  const entities = ref<Record<Id, UserDto>>({})
  const page = ref(1)
  const limit = ref(50)
  const total = ref(0)
  const loading = ref(false)
  const mainStore = useMainStore();
  const activeListQuery = ref<UserListQuery | null>(null)

  // 🧠 Getters
  const users = computed(() => ids.value.map(id => entities.value[id]))
  const listUsers = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((user): user is UserDto => user !== undefined)
  )
  const getUserById = (id: Id) => entities.value[id]
  const hasUser = (id: Id) => id in entities.value

  const nameMap = computed<Record<Id, string>>(() =>
    ids.value.reduce((acc, id) => {
      const u = entities.value[id]
      if (u) acc[id] = (u.name ?? u.email ?? String(id)) as string
      return acc
    }, {} as Record<Id, string>)
  )

  // 🔧 Mutations
  function setEntities(payload: UserDto[]) {
    for (const item of payload) {
      if (!item || typeof item.id !== 'number' || Number.isNaN(item.id)) {
        continue
      }
      entities.value[item.id] = item;
    }
    const newIds = payload
      .filter((item): item is UserDto => Boolean(item) && typeof item.id === 'number' && !Number.isNaN(item.id))
      .map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(payload: UserDto) {
    if (!payload || typeof payload.id !== 'number' || Number.isNaN(payload.id)) {
      return
    }
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id)
    entities.value[payload.id] = payload
    total.value = ids.value.length
  }

  function addEntity(payload: UserDto) {
    if (!payload || typeof payload.id !== 'number' || Number.isNaN(payload.id)) {
      return
    }
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id)
    entities.value[payload.id] = payload
    total.value = ids.value.length
  }

  function updateEntity(payload: UserDto) {
    if (!payload || typeof payload.id !== 'number' || Number.isNaN(payload.id)) {
      return
    }
    entities.value[payload.id] = payload
  }

  function deleteEntity(id: Id) {
    ids.value = ids.value.filter(x => x !== id)
    delete entities.value[id]
    total.value = ids.value.length
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

  // 📡 API Actions
  async function getUsers() {
    const mainStore = useMainStore()
    loading.value = true
    try {
      const data = await api.getUsers()
      setEntities(data)
      return data
    } catch (error) {
      await mainStore.checkApiError(error)
    } finally {
      loading.value = false
    }
  }

  async function fetchByIds(itemIds: readonly number[]) {
    if (!Array.isArray(itemIds)) {
      console.error("fetchByIds expected number[], got:", itemIds);
      return [];
    }
    const uniqueIds = [...new Set(itemIds)].filter((x): x is number => Number.isInteger(x));

    const missingIds = uniqueIds.filter(id => !(id in entities.value));
    if (missingIds.length === 0) return [];

    try {
      const res = await rpc<{ items: UserDto[] }>({
        operation: 'Get',
        return_type: 'User',
        filters: [{ field: 'id', op: 'in', value: missingIds }],
      });

      setEntities(res.items);
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function loadListQuery(query: UserListQuery) {
    activeListQuery.value = query
    loading.value = true
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'User',
        filters: query.filters,
        limit: limit.value,
        page: page.value,
        order: query.order,
      })

      const missingIDs = items.filter(id => !(id in entities.value))
      if (missingIDs.length > 0) {
        await fetchByIds(missingIDs)
      }

      listIds.value = items
      total.value = search_total

      return listUsers.value
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

  async function getUser(id: Id) {
    const mainStore = useMainStore()
    loading.value = true
    try {
      const data = await api.getUser(id)
      setEntity(data)
      return data
    } catch (error) {
      await mainStore.checkApiError(error)
    } finally {
      loading.value = false
    }
  }

  async function createUser(payload: CreateUserDto) {
    const mainStore = useMainStore()
    try {
      const data = await api.createUser(payload)
      const refreshed = await getUser(data.id)
      mainStore.addNotification({ content: 'User successfully created', color: 'success' })
      return refreshed ?? data
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  async function updateUser(payload: { id: Id; data: UpdateUserDto }) {
    const mainStore = useMainStore()
    try {
      const rpcPayload = {
        email: payload.data.email,
        name: payload.data.name,
        is_active: payload.data.isActive,
        is_admin: payload.data.isAdmin,
        ...(payload.data.password ? { password: payload.data.password } : {}),
      }

      await rpc<unknown>({
        operation: 'Update',
        return_type: 'User',
        id: payload.id,
        payload: rpcPayload,
      })
      const data = await getUser(payload.id)
      mainStore.addNotification({ content: 'User successfully updated', color: 'success' })
      return data
    } catch (error) {
      await mainStore.checkApiError(error)
    }
  }

  async function checkUserExists(email: string): Promise<boolean> {
    const mainStore = useMainStore()
    try {
      const { exists } = await api.checkUserExists(email)
      return Boolean(exists)
    } catch (error) {
      await mainStore.checkApiError(error)
      return false
    }
  }

  async function signUp(payload: CreateUserDto) {
    const mainStore = useMainStore()
    try {
      await api.signUp(payload)
      mainStore.routeLogOut()
      mainStore.addNotification({ content: 'Account confirmation email was sent', color: 'success' })
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
    users,
    listUsers,
    getUserById,
    hasUser,
    nameMap,

    // actions
    setEntities,
    setEntity,
    addEntity,
    updateEntity,
    deleteEntity,
    reset,
    resetListQuery,

    getUsers,
    fetchByIds,
    search,
    loadListQuery,
    reloadListQuery,
    getUser,
    createUser,
    updateUser,
    checkUserExists,
    signUp,

    //setSearch,
    setPage,
    setLimit,
  }
}, { persist: true })
