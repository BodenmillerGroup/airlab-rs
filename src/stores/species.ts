import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Filter } from '@/modules/json/api';
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';
import { useGroupStore } from '@/stores/group';

import type {
  CreateSpeciesDto,
  UpdateSpeciesDto,
  SpeciesDto,
} from '@/modules/species/types';

type Id = number;

type SpeciesListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  globalFilter?: string;
};

export const useSpeciesStore = defineStore('species', () => {
  // 📦 State
  const ids = ref<Id[]>([]);
  const listIds = ref<Id[]>([]);
  const entities = ref<Record<Id, SpeciesDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const searchstr = ref('');
  const loading = ref(false);
  const revision = ref(0);

  const main = useMainStore();
  const activeListQuery = ref<SpeciesListQuery | null>(null);

  // 🧠 Getters
  const species = computed(() => ids.value.map((id) => entities.value[id]));
  const listSpecies = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((item): item is SpeciesDto => item !== undefined)
  );
  const getSpecies = (id: Id) => entities.value[id];
  const hasSpecies = (id: Id) => id in entities.value;

  const nameMap = computed(() =>
    ids.value.reduce((acc, id) => {
      const s = entities.value[id];
      if (s) acc[id] = s.name;
      return acc;
    }, {} as Record<Id, string>)
  );

  // 🔧 Mutations

  function setEntities(payload: SpeciesDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
    revision.value++;
  }

  function deleteEntity(id: Id) {
    ids.value = ids.value.filter((x) => x !== id);
    delete entities.value[id];
    total.value = ids.value.length;
    revision.value++;
  }

  function reset() {
    ids.value = [];
    listIds.value = [];
    entities.value = {};
    page.value = 1;
    limit.value = 50;
    total.value = 0;
    searchstr.value = '';
    loading.value = false;
    revision.value++;
    activeListQuery.value = null;
  }

  function resetListQuery() {
    listIds.value = [];
    activeListQuery.value = null;
    loading.value = false;
  }

  // 🚀 RPC Actions
  async function createSpecies(payload: CreateSpeciesDto) {
    try {
      const data = await rpc<{ id?: Id } | SpeciesDto>({
        operation: 'Insert',
        return_type: 'Species',
        payload: payload,
      });
      const id = (data as SpeciesDto)?.id;
      if (typeof id === 'number') {
        await getSpeciesById(id);
      }
      main.addNotification({ content: 'Species successfully created', color: 'success' });
    } catch (error) {
      main.checkApiError(error);
    }
  }

  async function getSpeciesById(id: Id) {
    try {
      const data = await rpc<SpeciesDto>({
        operation: 'Get',
        return_type: 'Species',
        payload: id,
      });
      setEntities([data]);
      return data;
    } catch (error) {
      main.checkApiError(error);
    }
  }

  async function updateSpecies(payload: { id: Id; data: UpdateSpeciesDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Species',
        id: payload.id,
        payload: payload.data,
      });
      await getSpeciesById(payload.id);
      main.addNotification({ content: 'Species successfully updated', color: 'success' });
    } catch (error) {
      main.checkApiError(error);
    }
  }

  async function deleteSpecies(id: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Species',
        id,
      });
      deleteEntity(id);
      main.addNotification({ content: 'Species successfully deleted', color: 'success' });
    } catch (error) {
      main.checkApiError(error);
    }
  }

  async function fetchByIds(groupId: number, itemIds: number[]) {
    const uniqueIds = [...new Set(itemIds)].filter(Boolean) as number[]
    const missingIds = uniqueIds.filter(id => !(id in entities.value))


    if (missingIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: SpeciesDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Species',
        filters: [
          { field: 'group_id', op: 'eq', value: groupId },
          { field: 'id', op: 'in', value: missingIds },
        ],
        limit: limit.value,
        page: page.value,
      });
      setEntities(res.items);
      total.value = res.total;
      return res.items;
    } catch (error) {
      main.checkApiError(error);
      return [];
    }
  }

  async function ensureByIds(groupId: number, itemIds: number[]) {
    return fetchByIds(groupId, itemIds)
  }

  async function fetchByIdsV2(itemIds: number[]) {
    const uniqueIds = [...new Set(itemIds)].filter(Boolean) as number[]
    const missingIds = uniqueIds.filter(id => !(id in entities.value))


    if (missingIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: SpeciesDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Species',
        filters: [
          { field: 'id', op: 'in', value: missingIds },
        ],
        limit: limit.value,
        page: page.value,
      });
      setEntities(res.items);
      total.value = res.total;
      return res.items;
    } catch (error) {
      main.checkApiError(error);
      return [];
    }
  }

  async function loadListQuery(query: SpeciesListQuery) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Species',
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

      return listSpecies.value;
    } catch (error) {
      main.checkApiError(error);
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

  async function getGroupSpecies(groupId: Id) {
    loading.value = true;
    try {
      const filters = ref<Filter[]>(
        searchstr.value ?

          [{ field: 'group_id', op: 'eq', value: groupId },
          { field: 'name', op: 'contains', value: searchstr.value }]
          :
          [{ field: 'group_id', op: 'eq', value: groupId }]

      );

      const res = await rpc<{ items: SpeciesDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Species',
        filters: filters.value,
        limit: limit.value,
        page: page.value,
      });

      setEntities(res.items);
      total.value = res.total;
    } catch (error) {
      main.checkApiError(error);
    } finally {
      loading.value = false;
    }
  }

  async function updatePage(value: number) {
    page.value = value;
    const groupId = useGroupStore().activeGroupId;
    if (typeof groupId === 'number') {
      await getGroupSpecies(groupId);
    }
  }

  async function updateSearch(value: string) {
    searchstr.value = value;
    page.value = 1;
    const groupId = useGroupStore().activeGroupId;
    if (typeof groupId === 'number') {
      await getGroupSpecies(groupId);
    }
  }

  async function getSpeciesClones(speciesId: Id) {
    try {
      return await rpc<any[]>({
        operation: 'Get',
        return_type: 'Clone',
        filters: [{ field: 'species_id', op: 'eq', value: speciesId }],
      });
    } catch (error) {
      main.checkApiError(error);
      return [];
    }
  }

  // Optional UI helpers
  function setPage(value: number) {
    page.value = value;
  }

  function setLimit(value: number) {
    limit.value = value;
  }

  return {
    // State
    ids,
    listIds,
    entities,
    page,
    limit,
    total,
    loading,
    revision,

    // Getters
    species,
    listSpecies,
    getSpecies,
    hasSpecies,
    nameMap,

    // Mutations + Actions
    reset,
    resetListQuery,
    //setEntity,
    //addEntity,
    createSpecies,
    getSpeciesById,
    updateSpecies,
    deleteSpecies,
    ensureByIds,
    fetchByIds,
    fetchByIdsV2,
    search,
    loadListQuery,
    reloadListQuery,
    getGroupSpecies,
    getSpeciesClones,

    // UI controls
    setEntities,
    //setSearch,
    setPage,
    setLimit,
    updatePage,
    updateSearch,
  };
});
