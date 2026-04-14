import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';

import type {
  ProviderDto,
  CreateProviderDto,
  UpdateProviderDto,
} from '@/modules/provider/types';

type Id = number;

type ProviderListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  globalFilter?: string;
};

export const useProviderStore = defineStore('provider', () => {
  // 🧬 State
  const ids = ref<Id[]>([]);
  const listIds = ref<Id[]>([]);
  const entities = ref<Record<Id, ProviderDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const loading = ref(false);

  const mainStore = useMainStore();
  const activeListQuery = ref<ProviderListQuery | null>(null);

  // 🧠 Getters
  const providers = computed(() => ids.value.map((id) => entities.value[id]));
  const listProviders = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((provider): provider is ProviderDto => provider !== undefined)
  );
  const getProvider = (id: Id) => entities.value[id];
  const hasProvider = (id: Id) => id in entities.value;

  const nameMap = computed(() =>
    ids.value.reduce((acc, id) => {
      const p = entities.value[id];
      if (p?.name) acc[id] = p.name;
      return acc;
    }, {} as Record<Id, string>)
  );

  // 🔧 Mutations
  function setEntities(payload: ProviderDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(payload: ProviderDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id);
    entities.value[payload.id] = payload;
    total.value = ids.value.length;
  }

  function addEntity(payload: ProviderDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id);
    entities.value[payload.id] = payload;
    total.value = ids.value.length;
  }

  function updateEntity(payload: ProviderDto) {
    entities.value[payload.id] = payload;
  }

  function deleteEntity(id: Id) {
    ids.value = ids.value.filter(x => x !== id);
    delete entities.value[id];
    total.value = ids.value.length;
  }

  function reset() {
    ids.value = [];
    listIds.value = [];
    entities.value = {};
    page.value = 1;
    limit.value = 50;
    total.value = 0;
    loading.value = false;
    activeListQuery.value = null;
  }

  function resetListQuery() {
    listIds.value = [];
    activeListQuery.value = null;
    loading.value = false;
  }

  // 🚀 RPC Actions
  async function createProvider(payload: CreateProviderDto) {
    try {
      const data = await rpc<{ id?: Id } | ProviderDto>({
        operation: 'Insert',
        return_type: 'Provider',
        payload: payload,
      });
      const id = (data as ProviderDto)?.id;
      const entity = typeof id === 'number' ? await getProviderById(id) : undefined;
      mainStore.addNotification({ content: 'Provider successfully created', color: 'success' });
      return entity;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function getProviderById(id: Id) {
    loading.value = true;
    try {
      const data = await rpc<ProviderDto>({
        operation: 'Get',
        return_type: 'Provider',
        payload: id,
      });
      setEntity(data);
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    } finally {
      loading.value = false;
    }
  }

  async function fetchByIds(groupId: number, providerIds: number[]) {
    const uniqueIds = [...new Set(providerIds)].filter(Boolean) as number[]
    const missingIds = uniqueIds.filter(id => !(id in entities.value))


    if (missingIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: ProviderDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Provider',
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

  async function fetchByIdsV2(providerIds: number[]) {
    const uniqueIds = [...new Set(providerIds)].filter(Boolean) as number[]
    const missingIds = uniqueIds.filter(id => !(id in entities.value))


    if (missingIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: ProviderDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Provider',
        filters: [
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

  async function loadListQuery(query: ProviderListQuery) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Provider',
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

      return listProviders.value;
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

  async function updateProvider(payload: { id: Id; data: UpdateProviderDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Provider',
        id: payload.id,
        payload: payload.data,
      });
      const data = await getProviderById(payload.id);
      mainStore.addNotification({ content: 'Provider successfully updated', color: 'success' });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function deleteProvider(id: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Provider',
        id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Provider successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function getGroupProviders(groupId: Id) {
    loading.value = true;
    try {
      const res = await rpc<{ items: ProviderDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Provider',
        filters: [{ field: 'group_id', op: 'eq', value: groupId }],
        limit: limit.value,
        page: page.value,
      });
      setEntities(res.items);
      total.value = res.total;
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
    } finally {
      loading.value = false;
    }
  }

  async function getProviderLots(providerId: Id) {
    try {
      const res = await rpc<any[] | { items?: any[] }>({
        operation: 'Get',
        return_type: 'Lot',
        filters: [{ field: 'provider_id', op: 'eq', value: providerId }],
      });
      return Array.isArray(res) ? res : (res.items ?? []);
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  // UI State Helpers
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

    // Getters
    providers,
    listProviders,
    getProvider,
    hasProvider,
    nameMap,

    // Mutations & RPC
    reset,
    resetListQuery,
    setEntities,
    setEntity,
    addEntity,
    updateEntity,
    fetchByIds,
    fetchByIdsV2,
    search,
    loadListQuery,
    reloadListQuery,
    createProvider,
    getProviderById,
    updateProvider,
    deleteProvider,
    getGroupProviders,
    getProviderLots,

    // UI State
    //setSearch,
    setPage,
    setLimit,
  };
});
