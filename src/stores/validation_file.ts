import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main'

import type { ValidationFileDto } from '@/modules/validation_file/types'

type Id = number

type ValidationFileListQuery = {
  groupId: number
  filters: SearchFilter[]
  order: SearchOrder
}

export const useValidationFileStore = defineStore('validation_file', () => {
  // 🧪 State
  const ids = ref<Id[]>([])
  const listIds = ref<Id[]>([])
  const entities = ref<Record<Id, ValidationFileDto>>({})
  const page = ref(1)
  const limit = ref(50)
  const total = ref(0)
  const loading = ref(false)

  const mainStore = useMainStore();
  const activeListQuery = ref<ValidationFileListQuery | null>(null)

  // 🧠 Getters
  const files = computed(() => ids.value.map(id => entities.value[id]))
  const listFiles = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((file): file is ValidationFileDto => file !== undefined)
  )
  const getFileById = (id: Id) => entities.value[id]
  const hasFile = (id: Id) => id in entities.value

  const byValidationId = computed(() =>
    files.value.reduce((acc, file) => {
      (acc[file.validationId] ??= []).push(file)
      return acc
    }, {} as Record<Id, ValidationFileDto[]>)
  )

  // 🔧 Mutations
  function setEntities(payload: ValidationFileDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(payload: ValidationFileDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id)
    entities.value[payload.id] = payload
    total.value = ids.value.length
  }

  function addEntity(payload: ValidationFileDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id)
    entities.value[payload.id] = payload
    total.value = ids.value.length
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

  // 📡 RPC Actions
  async function getValidationFiles(validationId: Id) {
    const main = useMainStore()
    loading.value = true
    try {
      const { items, total: totalCount } = await rpc<{ items: ValidationFileDto[]; total: number }>({
        operation: 'Get',
        return_type: 'ValidationFile',
        filters: [
          { field: 'validation_id', op: 'eq', value: validationId },
        ],
        page: page.value,
        limit: limit.value,
      })

      setEntities(items)
      total.value = totalCount
      return items
    } catch (error) {
      await main.checkApiError(error)
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
      const res = await rpc<{ items: ValidationFileDto[] }>({
        operation: 'Get',
        return_type: 'ValidationFile',
        filters: [{ field: 'id', op: 'in', value: missingIds }],
      });

      setEntities(res.items);
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function loadListQuery(query: ValidationFileListQuery) {
    activeListQuery.value = query
    loading.value = true
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'ValidationFile',
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

      return listFiles.value
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

  async function deleteValidationFile(id: Id) {
    const main = useMainStore()
    try {
      await rpc({
        operation: 'Delete',
        return_type: 'ValidationFile',
        id,
      })

      deleteEntity(id)
      main.addNotification({ content: 'Validation file deleted', color: 'success' })
    } catch (error) {
      await main.checkApiError(error)
    }
  }

  // 🔍 Helpers
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
    files,
    listFiles,
    getFileById,
    hasFile,
    byValidationId,

    // mutations
    setEntities,
    setEntity,
    addEntity,
    deleteEntity,
    reset,
    resetListQuery,

    // actions
    fetchByIds,
    search,
    loadListQuery,
    reloadListQuery,
    getValidationFiles,
    deleteValidationFile,

    // UI helpers
    //setSearch,
    setPage,
    setLimit,
  }
}, { persist: true })
