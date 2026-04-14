import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';

import type { CreateStorageDto, StorageDto, UpdateStorageDto } from '@/modules/storage/types';

type Id = number;

type StorageListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  globalFilter?: string;
};

export const useStorageStore = defineStore('storage', () => {
  const ids = ref<Id[]>([]);
  const listIds = ref<Id[]>([]);
  const entities = ref<Record<Id, StorageDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const loading = ref(false);

  const mainStore = useMainStore();
  const activeListQuery = ref<StorageListQuery | null>(null);

  const storages = computed(() => ids.value.map((id) => entities.value[id]).filter(Boolean));
  const listStorages = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((storage): storage is StorageDto => storage !== undefined)
  );
  const getStorage = (id: Id) => entities.value[id];

  function setEntities(payload: StorageDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }

    const newIds = payload.map((item) => item.id);
    ids.value = Array.from(new Set([...ids.value, ...newIds]));
  }

  function setEntity(payload: StorageDto) {
    entities.value[payload.id] = payload;
    if (!ids.value.includes(payload.id)) {
      ids.value.push(payload.id);
    }
    total.value = Math.max(total.value, ids.value.length);
  }

  function deleteEntity(id: Id) {
    ids.value = ids.value.filter((itemId) => itemId !== id);
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

  async function createStorage(payload: CreateStorageDto) {
    try {
      const data = await rpc<{ id?: Id } | StorageDto>({
        operation: 'Insert',
        return_type: 'Storage',
        payload,
      });
      const id = (data as StorageDto)?.id;
      const entity = typeof id === 'number' ? await getStorageById(id) : undefined;
      mainStore.addNotification({ content: 'Storage successfully created', color: 'success' });
      return entity;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function getStorageById(id: Id) {
    try {
      const data = await rpc<StorageDto>({
        operation: 'Get',
        return_type: 'Storage',
        payload: id,
      });
      setEntity(data);
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function updateStorage(payload: { id: Id; data: UpdateStorageDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Storage',
        id: payload.id,
        payload: payload.data,
      });
      const data = await getStorageById(payload.id);
      mainStore.addNotification({ content: 'Storage successfully updated', color: 'success' });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function deleteStorage(id: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Storage',
        id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Storage successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function fetchByIds(storageIds: readonly number[]) {
    const uniqueIds = [...new Set(storageIds)].filter((id): id is number => Number.isInteger(id));
    const missingIds = uniqueIds.filter((id) => !(id in entities.value));

    if (missingIds.length === 0) {
      return [];
    }

    try {
      const res = await rpc<{ items: StorageDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Storage',
        filters: [{ field: 'id', op: 'in', value: missingIds }],
        limit: missingIds.length,
        page: 1,
      });
      setEntities(res.items);
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function loadListQuery(query: StorageListQuery) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Storage',
        filters: query.filters,
        limit: limit.value,
        page: page.value,
        order: query.order,
        global_filter: query.globalFilter,
      });

      const missingIds = items.filter((id) => !(id in entities.value));
      if (missingIds.length > 0) {
        await fetchByIds(missingIds);
      }

      listIds.value = items;
      total.value = search_total;
      return listStorages.value;
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

  async function getStorages() {
    return loadListQuery({
      groupId: -1,
      filters: [],
      order: { table: 'Storage', field: 'id', direction: 'desc' },
    });
  }

  async function updatePage(value: number) {
    page.value = value;
    await reloadListQuery();
  }

  return {
    ids,
    listIds,
    entities,
    page,
    limit,
    total,
    loading,
    storages,
    listStorages,
    getStorage,
    reset,
    resetListQuery,
    setEntities,
    createStorage,
    getStorageById,
    updateStorage,
    deleteStorage,
    fetchByIds,
    loadListQuery,
    reloadListQuery,
    getStorages,
    updatePage,
  };
});
