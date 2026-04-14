import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useMainStore } from '@/stores/main'
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useConjugateStore } from '@/stores/conjugate'
import { useLotStore } from '@/stores/lot'
import { useCloneStore } from '@/stores/clone'
import { useProteinStore } from '@/stores/protein'
import { useTagStore } from '@/stores/tag'
import { useValidationStore } from '@/stores/validation'

import type { PanelElementDto } from '@/modules/panel_element/types'

type Id = number

type PanelElementListQuery = {
  groupId: number
  panelId: Id
}

export const usePanelElementStore = defineStore('panel_element', () => {
  // 🧪 State
  const ids = ref<Id[]>([])
  const listIds = ref<Id[]>([])
  const entities = ref<Record<Id, PanelElementDto>>({})
  const page = ref(1)
  const limit = ref(50)
  const total = ref(0)
  const loading = ref(false)

  const mainStore = useMainStore();
  const conjugateStore = useConjugateStore()
  const lotStore = useLotStore()
  const cloneStore = useCloneStore()
  const proteinStore = useProteinStore()
  const tagStore = useTagStore()
  const validationStore = useValidationStore()
  const activeListQuery = ref<PanelElementListQuery | null>(null)

  // 🧠 Getters
  const panelElements = computed(() => ids.value.map(id => entities.value[id]))
  const listPanelElements = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((element): element is PanelElementDto => element !== undefined)
  )
  const getPanelElementById = (id: Id) => entities.value[id]
  const hasPanelElement = (id: Id) => id in entities.value

  const byPanelId = computed(() =>
    panelElements.value.reduce((acc, el) => {
      (acc[el.panelId] ??= []).push(el)
      return acc
    }, {} as Record<number, PanelElementDto[]>)
  )

  // 🔧 Mutations
  function setEntities(payload: PanelElementDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(payload: PanelElementDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id)
    entities.value[payload.id] = payload
    total.value = ids.value.length
  }

  function addEntity(payload: PanelElementDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id)
    entities.value[payload.id] = payload
    total.value = ids.value.length
  }

  function updateEntity(payload: PanelElementDto) {
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

  // 📡 RPC Actions
  async function getPanelElements(panelId: Id) {
    loading.value = true
    try {
      const { items, total: totalCount } = await rpc<{ items: PanelElementDto[]; total: number }>({
        operation: 'Get',
        return_type: 'PanelElement',
        filters: [
          { field: 'panel_id', op: 'eq', value: panelId },
        ],
        limit: limit.value,
        page: page.value,
      })

      setEntities(items)
      total.value = totalCount
      return items
    } catch (error) {
      await mainStore.checkApiError(error)
    } finally {
      loading.value = false
    }
  }

  async function getByConjugateId(conjugateId: Id) {
    loading.value = true
    try {
      const { items } = await rpc<{ items: PanelElementDto[]; total: number }>({
        operation: 'Get',
        return_type: 'PanelElement',
        filters: [
          { field: 'conjugate_id', op: 'eq', value: conjugateId },
        ],
        limit: limit.value,
        page: 1,
      })

      setEntities(items)
      return items
    } catch (error) {
      await mainStore.checkApiError(error)
      return []
    } finally {
      loading.value = false
    }
  }

  async function ensureListRelations(groupId: number, itemIds: readonly number[]) {
    if (groupId <= 0) {
      return
    }

    const visibleElements = itemIds
      .map((id) => entities.value[id])
      .filter((element): element is PanelElementDto => element !== undefined)

    const conjugateIds = visibleElements
      .map((element) => element.conjugateId)
      .filter((id): id is number => typeof id === 'number')

    const conjugates = await conjugateStore.fetchByIds(groupId, conjugateIds)

    const lotIds = conjugates
      .map((conjugate) => conjugate.lotId)
      .filter((id): id is number => typeof id === 'number')

    const lots = await lotStore.fetchByIds(groupId, lotIds)

    const tagIds = conjugates
      .map((conjugate) => conjugate.tagId)
      .filter((id): id is number => typeof id === 'number')

    await tagStore.fetchByIds(groupId, tagIds)

    const cloneIds = [...new Set(
      lots
        .map((lot) => lot.cloneId)
        .filter((id): id is number => typeof id === 'number')
    )]

    await cloneStore.fetchByIds(cloneIds)

    const proteinIds = cloneIds
      .map((id) => cloneStore.getClone(id)?.proteinId)
      .filter((id): id is number => typeof id === 'number')

    await proteinStore.fetchByIds(groupId, proteinIds)

    const previousValidationPage = validationStore.page
    validationStore.setPage(1)
    await validationStore.fetchByCloneIds(groupId, cloneIds)
    validationStore.setPage(previousValidationPage)
  }

  async function loadListQuery(query: PanelElementListQuery) {
    activeListQuery.value = query
    loading.value = true
    try {
      const allElements: PanelElementDto[] = []
      const previousPage = page.value
      let nextPage = 1
      const currentLimit = Number(limit.value) || 50

      while (true) {
        page.value = nextPage
        const pageItems = await getPanelElements(query.panelId) ?? []
        allElements.push(...pageItems)

        const expectedTotal = Number(total.value) || allElements.length
        if (allElements.length >= expectedTotal) break
        if (pageItems.length < currentLimit) break
        nextPage += 1
      }

      page.value = previousPage
      listIds.value = allElements.map((item) => item.id)
      await ensureListRelations(query.groupId, listIds.value)

      return listPanelElements.value
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

  async function fetchByIds(itemIds: readonly number[]) {
    if (!Array.isArray(itemIds)) {
      console.error("fetchByIds expected number[], got:", itemIds);
      return [];
    }
    const uniqueIds = [...new Set(itemIds)].filter((x): x is number => Number.isInteger(x));

    const missingIds = uniqueIds.filter(id => !(id in entities.value));
    if (missingIds.length === 0) return [];

    try {
      const res = await rpc<{ items: PanelElementDto[] }>({
        operation: 'Get',
        return_type: 'PanelElement',
        filters: [{ field: 'id', op: 'in', value: missingIds }],
      });

      setEntities(res.items);
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function search(filters: SearchFilter[], order: SearchOrder) {
    loading.value = true;
    try {

      const { items, search_total } = await rpcSearch({
        return_type: 'PanelElement',
        filters: filters,
        limit: limit.value,
        page: page.value,
        order: order,
      });

      const missingIDs = items.filter(id => !(id in entities.value));
      if (missingIDs.length > 0) {
        await fetchByIds(missingIDs);
      }

      total.value = search_total;

      return items
        .map(id => entities.value[id])
        .filter((c): c is PanelElementDto => c !== undefined);

    } catch (error) {
      mainStore.checkApiError(error);
    } finally {
      loading.value = false;
    }
  }

  async function createPanelElement(payload: Omit<PanelElementDto, 'id'>) {
    const main = useMainStore()
    try {
      const created = await rpc<PanelElementDto>({
        operation: 'Insert',
        return_type: 'PanelElement',
        payload: payload,
      })

      addEntity(created)
      main.addNotification({ content: 'Panel element created', color: 'success' })
      return created
    } catch (error) {
      await main.checkApiError(error)
    }
  }

  async function updatePanelElement(id: Id, data: Partial<PanelElementDto>) {
    const main = useMainStore()
    try {
      const updated = await rpc<PanelElementDto>({
        operation: 'Update',
        return_type: 'PanelElement',
        id,
        payload: data,
      })

      updateEntity(updated)
      main.addNotification({ content: 'Panel element updated', color: 'success' })
      return updated
    } catch (error) {
      await main.checkApiError(error)
    }
  }

  async function deletePanelElement(id: Id) {
    const main = useMainStore()
    try {
      await rpc({
        operation: 'Delete',
        return_type: 'PanelElement',
        id,
      })

      deleteEntity(id)
      main.addNotification({ content: 'Panel element deleted', color: 'success' })
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
    panelElements,
    listPanelElements,
    getPanelElementById,
    hasPanelElement,
    byPanelId,

    // mutations
    setEntities,
    setEntity,
    addEntity,
    updateEntity,
    deleteEntity,
    reset,
    resetListQuery,

    // actions
    fetchByIds,
    search,
    loadListQuery,
    reloadListQuery,
    getPanelElements,
    getByConjugateId,
    createPanelElement,
    updatePanelElement,
    deletePanelElement,

    // ui helpers
    //setSearch,
    setPage,
    setLimit,
  }
}, { persist: true })
