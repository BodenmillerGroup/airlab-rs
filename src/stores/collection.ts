import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';

import type {
  CollectionDto,
  CreateCollectionDto,
  UpdateCollectionDto,
} from '@/modules/collection/types';

type Id = number;

type CollectionListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  globalFilter?: string;
};

export const useCollectionStore = defineStore('collection', () => {
  const ids = ref<Id[]>([]);
  const listIds = ref<Id[]>([]);
  const entities = ref<Record<Id, CollectionDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const loading = ref(false);

  const mainStore = useMainStore();
  const activeListQuery = ref<CollectionListQuery | null>(null);

  const collections = computed(() => ids.value.map((id) => entities.value[id]).filter(Boolean));
  const listCollections = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((collection): collection is CollectionDto => collection !== undefined)
  );
  const getCollection = (id: Id) => entities.value[id];

  function setEntities(payload: CollectionDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map((item) => item.id);
    ids.value = Array.from(new Set([...ids.value, ...newIds]));
  }

  function setEntity(payload: CollectionDto) {
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

  async function createCollection(payload: CreateCollectionDto) {
    try {
      const data = await rpc<{ id?: Id } | CollectionDto>({
        operation: 'Insert',
        return_type: 'Collection',
        payload,
      });
      const id = (data as CollectionDto)?.id;
      const entity = typeof id === 'number' ? await getCollectionById(id) : undefined;
      mainStore.addNotification({ content: 'Collection successfully created', color: 'success' });
      return entity;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function getCollectionById(id: Id) {
    try {
      const data = await rpc<CollectionDto>({
        operation: 'Get',
        return_type: 'Collection',
        payload: id,
      });
      setEntity(data);
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function updateCollection(payload: { id: Id; data: UpdateCollectionDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Collection',
        id: payload.id,
        payload: payload.data,
      });
      const data = await getCollectionById(payload.id);
      mainStore.addNotification({ content: 'Collection successfully updated', color: 'success' });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function deleteCollection(id: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Collection',
        id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Collection successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function fetchByIds(collectionIds: readonly number[]) {
    const uniqueIds = [...new Set(collectionIds)].filter((id): id is number => Number.isInteger(id));
    const missingIds = uniqueIds.filter((id) => !(id in entities.value));

    if (missingIds.length === 0) {
      return [];
    }

    try {
      const res = await rpc<{ items: CollectionDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Collection',
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

  async function loadListQuery(query: CollectionListQuery) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Collection',
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
      return listCollections.value;
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

  async function getCollections() {
    return loadListQuery({
      groupId: -1,
      filters: [],
      order: { table: 'Collection', field: 'id', direction: 'desc' },
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
    collections,
    listCollections,
    getCollection,
    reset,
    resetListQuery,
    setEntities,
    createCollection,
    getCollectionById,
    updateCollection,
    deleteCollection,
    fetchByIds,
    loadListQuery,
    reloadListQuery,
    getCollections,
    updatePage,
  };
});
