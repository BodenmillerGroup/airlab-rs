import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Filter } from '@/modules/json/api';
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';
import { useGroupStore } from '@/stores/group';

import type {
  CreateProteinDto,
  UpdateProteinDto,
  ProteinDto,
} from '@/modules/protein/types';

export const useProteinStore = defineStore('protein', () => {
  // 🧬 State
  const ids = ref<number[]>([]);
  const listIds = ref<number[]>([]);
  const entities = ref<Record<number, ProteinDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const searchstr = ref('');
  const loading = ref(false);
  const revision = ref(0);

  const mainStore = useMainStore();
  const activeListQuery = ref<{
    groupId: number;
    filters: SearchFilter[];
    order: SearchOrder;
  } | null>(null);

  // 🧠 Getters
  const proteins = computed(() => ids.value.map(id => entities.value[id]));
  const listProteins = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((protein): protein is ProteinDto => protein !== undefined)
  );
  const getProteinById = (id: number) => entities.value[id];

  const nameMap = computed(() => {
    const map: Record<number, string> = {};
    for (const [id, protein] of Object.entries(entities.value)) {
      map[+id] = protein.name;
    }
    return map;
  });

  const getGroupProteins = (groupId: number) => {
    return ids.value
      .map(id => entities.value[id])
      .filter(protein => protein.groupId === groupId);
  };

  // 🛠️ Mutations
  function setEntities(payload: ProteinDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
    revision.value++;
  }


  function deleteEntity(id: number) {
    ids.value = ids.value.filter(existingId => existingId !== id);
    const { [id]: _, ...rest } = entities.value;
    entities.value = rest;
    revision.value++;
  }

  function reset() {
    ids.value = [];
    listIds.value = [];
    entities.value = {};
    page.value = 1;
    total.value = 0;
    searchstr.value = '';
    revision.value++;
    activeListQuery.value = null;
  }

  function resetListQuery() {
    listIds.value = [];
    activeListQuery.value = null;
    loading.value = false;
  }

  // 🚀 RPC Actions
  async function createProtein(payload: CreateProteinDto) {
    try {
      const protein = await rpc<{ id?: number } | ProteinDto>({
        operation: 'Insert',
        return_type: 'Protein',
        payload: payload,
      });
      const id = (protein as ProteinDto)?.id;
      if (typeof id === 'number') {
        await fetchByIdsV2([id]);
      }
      mainStore.addNotification({ content: 'Protein successfully created', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }


  const getProtein = (id: number) => entities.value[id];

  async function updateProtein(payload: { id: number; data: UpdateProteinDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Protein',
        id: payload.id,
        payload: payload.data,
      });
      await fetchByIdsV2([payload.id], true);
      mainStore.addNotification({ content: 'Protein successfully updated', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function deleteProtein(id: number) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Protein',
        id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Protein successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function fetchByIds(groupId: number, itemIds: number[]) {
    const uniqueIds = [...new Set(itemIds)].filter(Boolean) as number[]
    const missingIds = uniqueIds.filter(id => !(id in entities.value))


    if (missingIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: ProteinDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Protein',
        filters: [
          { field: 'group_id', op: 'eq', value: groupId },
          { field: 'id', op: 'in', value: missingIds },
        ],
        limit: missingIds.length,
        page: 1,
      });
      setEntities(res.items);
      total.value = res.total;
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function ensureByIds(groupId: number, itemIds: number[]) {
    return fetchByIds(groupId, itemIds)
  }

  async function fetchByIdsV2(itemIds: number[], forceRefresh = false) {
    const uniqueIds = [...new Set(itemIds)].filter(Boolean) as number[]
    const targetIds = forceRefresh
      ? uniqueIds
      : uniqueIds.filter(id => !(id in entities.value))


    if (targetIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: ProteinDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Protein',
        filters: [
          { field: 'id', op: 'in', value: targetIds },
        ],
        limit: targetIds.length,
        page: 1,
      });
      setEntities(res.items);
      total.value = res.total;
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function fetchGroupProteins(groupId: number) {
    loading.value = true;
    try {
      const filters = ref<Filter[]>(
        searchstr.value ?

          [{ field: 'group_id', op: 'eq', value: groupId },
          { field: 'name', op: 'contains', value: searchstr.value }]
          :
          [{ field: 'group_id', op: 'eq', value: groupId }]

      );

      const res = await rpc<{ items: ProteinDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Protein',
        filters: filters.value,
        limit: limit.value,
        page: page.value,
      });

      setEntities(res.items);
      total.value = res.total;
    } catch (error) {
      mainStore.checkApiError(error);
    } finally {
      loading.value = false;
    }
  }

  async function updatePage(value: number) {
    page.value = value;
    const groupId = useGroupStore().activeGroupId;
    if (typeof groupId === 'number') {
      await fetchGroupProteins(groupId);
    }
  }

  async function updateSearch(value: string) {
    searchstr.value = value;
    page.value = 1;
    const groupId = useGroupStore().activeGroupId;
    if (typeof groupId === 'number') {
      await fetchGroupProteins(groupId);
    }
  }

  async function getProteinClones(proteinId: number) {
    try {
      return await rpc<any[]>({
        operation: 'Get',
        return_type: 'Clone',
        filters: [{ field: 'protein_id', op: 'eq', value: proteinId }],
      });
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function loadListQuery(query: {
    groupId: number;
    filters: SearchFilter[];
    order: SearchOrder;
    globalFilter?: string;
  }) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Protein',
        filters: query.filters,
        limit: limit.value,
        page: page.value,
        order: query.order,
        global_filter: query.globalFilter,
      });

      const missingIDs = items.filter(id => !(id in entities.value));
      if (missingIDs.length > 0) {
        await fetchByIdsV2(missingIDs);
      }

      listIds.value = items;
      total.value = search_total;

      return listProteins.value;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    } finally {
      loading.value = false;
    }
  }

  async function reloadListQuery() {
    if (!activeListQuery.value) {
      return [];
    }

    return loadListQuery(activeListQuery.value);
  }

  async function search(filters: SearchFilter[], order: SearchOrder) {
    return loadListQuery({
      groupId: -1,
      filters,
      order,
    });
  }

  return {
    // State
    ids,
    listIds,
    entities,
    page,
    limit,
    total,
    searchstr,
    loading,
    revision,

    // Getters
    proteins,
    listProteins,
    getProteinById,
    nameMap,
    getGroupProteins,

    // Actions
    reset,
    resetListQuery,
    //setEntity,
    //addEntity,
    createProtein,
    getProtein,
    updateProtein,
    deleteProtein,
    ensureByIds,
    fetchByIds,
    fetchByIdsV2,
    search,
    loadListQuery,
    reloadListQuery,
    fetchGroupProteins,
    updatePage,
    updateSearch,
    getProteinClones,
    setEntities,
  };
});
