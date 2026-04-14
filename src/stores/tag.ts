import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';

import type {
  TagDto,
  CreateTagDto,
  UpdateTagDto,
} from '@/modules/tag/types';

type Id = number;

type TagListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  globalFilter?: string;
};

export const useTagStore = defineStore('tag', () => {
  // 🧬 State
  const ids = ref<Id[]>([]);
  const listIds = ref<Id[]>([]);
  const entities = ref<Record<Id, TagDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const loading = ref(false);

  const mainStore = useMainStore();
  const activeListQuery = ref<TagListQuery | null>(null);

  // 🧠 Getters
  const tags = computed(() => ids.value.map(id => entities.value[id]));
  const listTags = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((tag): tag is TagDto => tag !== undefined)
  );
  const getTag = (id: Id) => entities.value[id];
  const hasTag = (id: Id) => id in entities.value;

  const nameMap = computed(() =>
    ids.value.reduce((acc, id) => {
      const t = entities.value[id];
      if (t?.name) acc[id] = t.name;
      return acc;
    }, {} as Record<Id, string>)
  );

  // 🛠️ Mutations
  function setEntities(payload: TagDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(payload: TagDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id);
    entities.value[payload.id] = payload;
    total.value = ids.value.length;
  }

  function addEntity(payload: TagDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id);
    entities.value[payload.id] = payload;
    total.value = ids.value.length;
  }

  function updateEntity(payload: TagDto) {
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
  async function createTag(payload: CreateTagDto) {
    try {
      const data = await rpc<{ id?: Id } | TagDto>({
        operation: 'Insert',
        return_type: 'Tag',
        payload: payload,
      });
      const id = (data as TagDto)?.id;
      const entity = typeof id === 'number' ? await getTagById(id) : undefined;
      mainStore.addNotification({ content: 'Tag successfully created', color: 'success' });
      return entity;
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
      const res = await rpc<{ items: TagDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Tag',
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

  async function fetchByIdsV2(itemIds: number[]) {
    const uniqueIds = [...new Set(itemIds)].filter(Boolean) as number[]
    const missingIds = uniqueIds.filter(id => !(id in entities.value))

    if (missingIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: TagDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Tag',
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

  async function loadListQuery(query: TagListQuery) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Tag',
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

      return listTags.value;
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

  async function getTagById(id: Id) {
    loading.value = true;
    try {
      const data = await rpc<TagDto>({
        operation: 'Get',
        return_type: 'Tag',
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

  async function updateTag(payload: { id: Id; data: UpdateTagDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Tag',
        id: payload.id,
        payload: payload.data,
      });
      const data = await getTagById(payload.id);
      mainStore.addNotification({ content: 'Tag successfully updated', color: 'success' });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function deleteTag(id: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Tag',
        id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Tag successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function getGroupTags(groupId: Id) {
    loading.value = true;
    try {
      const res = await rpc<{ items: TagDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Tag',
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

  // Optional helpers for server-driven UIs
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
    tags,
    listTags,
    getTag,
    hasTag,
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
    createTag,
    getTagById,
    updateTag,
    deleteTag,
    getGroupTags,

    // UI helpers
    //setSearch,
    setPage,
    setLimit,
  };
});
