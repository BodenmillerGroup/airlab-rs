import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';
import { useLotStore } from '@/stores/lot';
import { useCloneStore } from '@/stores/clone';
import { useProteinStore } from '@/stores/protein';
import { useUserStore } from '@/stores/user';
import { useMemberStore } from '@/stores/member';
import { useTagStore } from '@/stores/tag';
import { useValidationStore } from '@/stores/validation';
import { useStorageStore } from '@/stores/storage';

import type {
  ConjugateDto,
  CreateConjugateDto,
  UpdateConjugateDto,
  UpdateConjugateStatusDto,
} from '@/modules/conjugate/types';
import type { UpdateStateDto } from '@/modules/core/types';
import { ConjugateStatus } from '@/modules/conjugate/ConjugateStatus';

type Id = number;

type ConjugateListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  globalFilter?: string;
};

export const useConjugateStore = defineStore('conjugate', () => {
  // 🔧 State
  const ids = ref<Id[]>([]);
  const listIds = ref<Id[]>([]);
  const entities = ref<Record<Id, ConjugateDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const loading = ref(false);

  const mainStore = useMainStore();
  const lotStore = useLotStore();
  const cloneStore = useCloneStore();
  const proteinStore = useProteinStore();
  const userStore = useUserStore();
  const memberStore = useMemberStore();
  const tagStore = useTagStore();
  const validationStore = useValidationStore();
  const storageStore = useStorageStore();
  const activeListQuery = ref<ConjugateListQuery | null>(null);

  // 🧠 Getters
  const conjugates = computed(() => ids.value.map((id) => entities.value[id]));
  const listConjugates = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((conjugate): conjugate is ConjugateDto => conjugate !== undefined)
  );
  const getConjugate = (id: Id) => entities.value[id];
  const hasConjugate = (id: Id) => id in entities.value;

  const getConjugatesForTag = (tagId: Id): ConjugateDto[] =>
    ids.value
      .map((id) => entities.value[id])
      .filter((c) => c && c.tagId === tagId) as ConjugateDto[];

  const tubeNumberMap = computed(() =>
    ids.value.reduce((acc, id) => {
      const c = entities.value[id];
      if (c) acc[id] = c.tubeNumber as unknown as string;
      return acc;
    }, {} as Record<Id, string>)
  );

  // 🔨 Mutations
  function setEntities(payload: ConjugateDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(payload: ConjugateDto) {
    if (!ids.value.includes(payload.id)) {
      ids.value.push(payload.id);
    }
    entities.value[payload.id] = payload;
  }

  function addEntity(payload: ConjugateDto) {
    if (!ids.value.includes(payload.id)) {
      ids.value.push(payload.id);
    }
    entities.value[payload.id] = payload;
    total.value = ids.value.length;
  }

  function updateEntity(payload: ConjugateDto) {
    entities.value[payload.id] = payload;
  }

  function deleteEntity(id: Id) {
    ids.value = ids.value.filter((x) => x !== id);
    delete entities.value[id];
    total.value = ids.value.length;
  }

  function reset() {
    ids.value = [];
    entities.value = {};
    page.value = 1;
    limit.value = 50;
    total.value = 0;
    resetListQuery();
  }

  function resetListQuery() {
    listIds.value = [];
    activeListQuery.value = null;
    loading.value = false;
  }

  // 🚀 RPC Actions
  async function createConjugate(payload: CreateConjugateDto) {
    try {
      const data = await rpc<{ id?: Id } | ConjugateDto>({
        operation: 'Insert',
        return_type: 'Conjugate',
        payload: payload,
      });
      const id = (data as ConjugateDto)?.id;
      const entity = typeof id === 'number' ? await getConjugateById(id) : undefined;
      mainStore.addNotification({ content: 'Conjugate successfully created', color: 'success' });
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
      const res = await rpc<{ items: ConjugateDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Conjugate',
        filters: [
          { field: 'group_id', op: 'eq', value: groupId },
          { field: 'id', op: 'in', value: missingIds },
        ],
        // Fetching explicit ids should not be paginated by the current table state.
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

  async function fetchByIdsV2(itemIds: number[], forceRefresh = false) {
    const uniqueIds = [...new Set(itemIds)].filter(Boolean) as number[]
    const targetIds = forceRefresh
      ? uniqueIds
      : uniqueIds.filter(id => !(id in entities.value))

    if (targetIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: ConjugateDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Conjugate',
        filters: [
          { field: 'id', op: 'in', value: targetIds },
        ],
        // Fetching explicit ids should not be paginated by the current table state.
        limit: targetIds.length,
        page: 1,
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
      return;
    }

    const visibleConjugates = itemIds
      .map((id) => entities.value[id])
      .filter((conjugate): conjugate is ConjugateDto => conjugate !== undefined);

    const memberIds = visibleConjugates
      .map((conjugate) => conjugate.labeledBy)
      .filter((id): id is number => typeof id === 'number');

    await memberStore.fetchByIds(memberIds);

    const userIds = [...new Set(
      memberIds
        .map((id) => memberStore.getMemberById(id)?.userId)
        .filter((id): id is number => typeof id === 'number')
    )];

    await userStore.fetchByIds(userIds);

    const tagIds = visibleConjugates
      .map((conjugate) => conjugate.tagId)
      .filter((id): id is number => typeof id === 'number');

    await tagStore.fetchByIds(groupId, tagIds);

    const storageIds = visibleConjugates
      .map((conjugate) => conjugate.storageId)
      .filter((id): id is number => typeof id === 'number');

    await storageStore.fetchByIds(storageIds);

    const lotIds = visibleConjugates
      .map((conjugate) => conjugate.lotId)
      .filter((id): id is number => typeof id === 'number');

    await lotStore.fetchByIds(groupId, lotIds, true);

    const cloneIds = [...new Set(
      lotIds
        .map((id) => lotStore.getLotById(id)?.cloneId)
        .filter((id): id is number => typeof id === 'number')
    )];

    await cloneStore.fetchByIds(cloneIds);

    const proteinIds = [...new Set(
      cloneIds
        .map((id) => cloneStore.getClone(id)?.proteinId)
        .filter((id): id is number => typeof id === 'number')
    )];

    await proteinStore.fetchByIds(groupId, proteinIds);
    await validationStore.fetchByCloneIds(groupId, cloneIds);
  }

  async function loadListQuery(query: ConjugateListQuery, forceRefresh = false) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Conjugate',
        filters: query.filters,
        limit: limit.value,
        page: page.value,
        order: query.order,
        global_filter: query.globalFilter,
      });

      const targetIds = forceRefresh
        ? items
        : items.filter((id) => !(id in entities.value));
      if (targetIds.length > 0) {
        await fetchByIdsV2(targetIds, forceRefresh);
      }

      listIds.value = items;
      total.value = search_total;

      await ensureListRelations(query.groupId, items);

      return listConjugates.value;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    } finally {
      loading.value = false;
    }
  }

  async function reloadListQuery(forceRefresh = false) {
    if (!activeListQuery.value) {
      return [];
    }

    return loadListQuery(activeListQuery.value, forceRefresh);
  }

  async function search(filters: SearchFilter[], order: SearchOrder) {
    return loadListQuery({
      groupId: -1,
      filters,
      order,
    });
  }

  async function getConjugateById(id: Id) {
    try {
      const res = await rpc<{ items: ConjugateDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Conjugate',
        filters: [{ field: 'id', op: 'eq', value: id }],
        limit: 1,
        page: 1,
      });
      const data = res.items[0];
      if (!data) {
        return;
      }
      setEntity(data);
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function updateConjugate(payload: { id: Id; data: UpdateConjugateDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Conjugate',
        id: payload.id,
        payload: payload.data,
      });
      const fetched = await fetchByIdsV2([payload.id], true);
      const data = fetched[0];
      if (activeListQuery.value) {
        await reloadListQuery(true);
      }
      mainStore.addNotification({ content: 'Conjugate successfully updated', color: 'success' });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function updateConjugateArchiveState(payload: { id: Id; data: UpdateStateDto }) {
    try {
      const data = await rpc<ConjugateDto>({
        operation: 'Update',
        return_type: 'Conjugate',
        id: payload.id,
        payload: payload.data,
      });
      updateEntity(data);
      mainStore.addNotification({
        content: `Conjugate successfully ${payload.data.state ? 'archived' : 'unarchived'}`,
        color: 'success',
      });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function updateConjugateStatus(payload: { id: Id; data: UpdateConjugateStatusDto }) {
    try {
      const data = await rpc<ConjugateDto>({
        operation: 'Update',
        return_type: 'Conjugate',
        id: payload.id,
        payload: payload.data,
      });
      updateEntity(data);
      const statusName = ConjugateStatus[payload.data.status];
      mainStore.addNotification({
        content: `Conjugate successfully changed its status to ${statusName}`,
        color: 'success',
      });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function deleteConjugate(id: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Conjugate',
        id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Conjugate successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function getGroupConjugates(groupId: Id) {
    loading.value = true;
    try {
      const res = await rpc<{ items: ConjugateDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Conjugate',
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

  async function getConjugatePanels(id: Id) {
    try {
      return await rpc<any[]>({
        operation: 'Get',
        return_type: 'Panel',
        filters: [{ field: 'conjugate_id', op: 'eq', value: id }],
      });
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function getConjugateClones(id: Id) {
    try {
      return await rpc<any[]>({
        operation: 'Get',
        return_type: 'Clone',
        filters: [{ field: 'conjugate_id', op: 'eq', value: id }],
      });
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function getConjugateValidations(id: Id) {
    try {
      return await rpc<any[]>({
        operation: 'Get',
        return_type: 'Validation',
        filters: [{ field: 'conjugate_id', op: 'eq', value: id }],
      });
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  function setPage(value: number) {
    page.value = value;
  }

  function setLimit(value: number) {
    limit.value = value;
  }

  return {
    // 📦 State
    ids,
    listIds,
    entities,
    page,
    limit,
    total,
    search,
    loading,

    // 🧠 Getters
    conjugates,
    listConjugates,
    getConjugate,
    hasConjugate,
    getConjugatesForTag,
    tubeNumberMap,

    // 🔨 Mutations + Actions
    reset,
    resetListQuery,
    setEntity,
    addEntity,
    //setSearch,
    setPage,
    setLimit,

    // RPC Actions
    createConjugate,
    getConjugateById,
    updateConjugate,
    updateConjugateArchiveState,
    updateConjugateStatus,
    fetchByIds,
    fetchByIdsV2,
    loadListQuery,
    reloadListQuery,
    deleteConjugate,
    getGroupConjugates,
    getConjugatePanels,
    getConjugateClones,
    getConjugateValidations,
  };
});
