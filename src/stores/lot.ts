import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Filter } from '@/modules/json/api';
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';
import { useCloneStore } from '@/stores/clone';
import { useProviderStore } from '@/stores/provider';
import { useCollectionStore } from '@/stores/collection';
import { useStorageStore } from '@/stores/storage';
import { useValidationStore } from '@/stores/validation';
import { LotStatus } from '@/modules/lot/LotStatus';
import { lotStatusToString } from '@/utils/converters';

import type {
  LotDto,
  CreateLotDto,
  UpdateLotDto,
  LotQuery,
  UpdateLotStatusDto,
  ReorderLotDto,
} from '@/modules/lot/types';
import type { UpdateStateDto } from '@/modules/core/types';

type LotListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  globalFilter?: string;
};

export const useLotStore = defineStore('lot', () => {
  // 🔧 State
  const ids = ref<number[]>([]);
  const listIds = ref<number[]>([]);
  const entities = ref<Record<number, LotDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const loading = ref(false);
  const mainStore = useMainStore();
  const cloneStore = useCloneStore();
  const providerStore = useProviderStore();
  const collectionStore = useCollectionStore();
  const storageStore = useStorageStore();
  const validationStore = useValidationStore();
  const activeListQuery = ref<LotListQuery | null>(null);

  // 🧠 Getters
  const lots = computed(() => ids.value.map(id => entities.value[id]));
  const listLots = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((lot): lot is LotDto => lot !== undefined)
  );
  const getLotById = (id: number) => entities.value[id];

  const getCsv = (items: readonly LotDto[]) => {
    const separator = ';';
    const header = ['Id', 'Name', 'Clone', 'Provider', 'Number', 'Reference', 'Price', 'Status'];
    const lines = items.map((item) => {
      const line = [
        item.id,
        item.name,
        (item as any).clone?.name,
        (item as any).provider?.name || ' ',
        item.number,
        item.reference,
        item.price,
        lotStatusToString(item.status),
      ];
      return line.join(separator);
    });
    return [header.join(separator), ...lines].join('\n');
  };

  // 🛠️ Mutations
  function setEntities(payload: LotDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(payload: LotDto) {
    if (!ids.value.includes(payload.id)) {
      ids.value.push(payload.id);
    }
    entities.value[payload.id] = payload;
  }

  function addEntity(payload: LotDto) {
    if (!ids.value.includes(payload.id)) {
      ids.value.push(payload.id);
    }
    entities.value[payload.id] = payload;
  }

  function updateEntity(payload: LotDto) {
    entities.value[payload.id] = payload;
  }

  function deleteEntity(id: number) {
    ids.value = ids.value.filter((i) => i !== id);
    const { [id]: _, ...rest } = entities.value;
    entities.value = rest;
  }

  function reset() {
    ids.value = [];
    listIds.value = [];
    entities.value = {};
    page.value = 1;
    total.value = 0;
    activeListQuery.value = null;
  }

  function resetListQuery() {
    listIds.value = [];
    activeListQuery.value = null;
    loading.value = false;
  }

  // 🚀 RPC Actions
  async function createLot(payload: CreateLotDto) {
    try {
      const lot = await rpc<{ id?: number } | LotDto>({
        operation: 'Insert',
        return_type: 'Lot',
        payload: payload,
      });
      const id = (lot as LotDto)?.id;
      if (typeof id === 'number') {
        await fetchByIdsV2([id]);
      }
      mainStore.addNotification({ content: 'Lot successfully created', color: 'success' });
    } catch (e) {
      mainStore.checkApiError(e);
    }
  }

  async function fetchByIds(groupId: number, itemIds: number[], forceRefresh = false) {
    const uniqueIds = [...new Set(itemIds)].filter(Boolean) as number[]
    const targetIds = forceRefresh
      ? uniqueIds
      : uniqueIds.filter(id => entities.value[id] === undefined)

    if (targetIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: LotDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Lot',
        filters: [
          { field: 'group_id', op: 'eq', value: groupId },
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

  async function fetchByIdsV2(itemIds: number[], forceRefresh = false) {
    const uniqueIds = [...new Set(itemIds)].filter(Boolean) as number[]
    const targetIds = forceRefresh
      ? uniqueIds
      : uniqueIds.filter(id => entities.value[id] === undefined)

    if (targetIds.length === 0) {
      return []
    }
    try {
      const res = await rpc<{ items: LotDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Lot',
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

  const getLot = (id: number) => entities.value[id];

  async function ensureListRelations(groupId: number, itemIds: readonly number[]) {
    if (groupId <= 0) {
      return;
    }

    const visibleLots = itemIds
      .map((id) => entities.value[id])
      .filter((lot): lot is LotDto => lot !== undefined);

    const cloneIds = visibleLots
      .map((lot) => lot.cloneId)
      .filter((id): id is number => typeof id === 'number');

    const providerIds = visibleLots
      .map((lot) => lot.providerId)
      .filter((id): id is number => typeof id === 'number');

    const storageIds = visibleLots
      .map((lot) => lot.storageId)
      .filter((id): id is number => typeof id === 'number');

    await cloneStore.fetchByIds(cloneIds);
    await validationStore.fetchByCloneIds(groupId, [...new Set(cloneIds)]);
    await providerStore.fetchByIds(groupId, providerIds);
    await storageStore.fetchByIds(storageIds);

    if (collectionStore.collections.length === 0) {
      await collectionStore.getCollections();
    }
  }

  async function loadListQuery(query: LotListQuery) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Lot',
        filters: query.filters,
        limit: limit.value,
        page: page.value,
        order: query.order,
        global_filter: query.globalFilter,
      });

      const missingIDs = items.filter((id) => !(id in entities.value));
      if (missingIDs.length > 0) {
        await fetchByIdsV2(missingIDs);
      }

      listIds.value = items;
      total.value = search_total;

      await ensureListRelations(query.groupId, items);

      return listLots.value;
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

  async function updateLot(id: number, data: UpdateLotDto) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Lot',
        id,
        payload: data,
      });
      await fetchByIdsV2([id], true);
      mainStore.addNotification({ content: 'Lot successfully updated', color: 'success' });
    } catch (e) {
      mainStore.checkApiError(e);
    }
  }

  async function updateLotArchiveState(id: number, data: UpdateStateDto) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Lot',
        id,
        payload: data,
      });
      await fetchByIdsV2([id], true);
      mainStore.addNotification({
        content: `Lot successfully ${data.state ? 'archived' : 'unarchived'}`,
        color: 'success',
      });
    } catch (e) {
      mainStore.checkApiError(e);
    }
  }

  async function updateLotStatus(id: number, data: UpdateLotStatusDto) {
    try {
      const { lotNumber, ...statusData } = data;
      const payload = lotNumber === undefined
        ? statusData
        : { ...statusData, number: lotNumber };

      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Lot',
        id,
        payload,
      });
      await fetchByIdsV2([id], true);
      mainStore.addNotification({
        content: `Lot status changed to ${lotStatusToString(data.status)}`,
        color: 'success',
      });
    } catch (e) {
      mainStore.checkApiError(e);
    }
  }

  async function reorderLot(id: number, data: ReorderLotDto) {
    try {
      const lot = await rpc<LotDto>({
        operation: 'Reorder',
        return_type: 'Lot',
        id,
        payload: data,
      });
      addEntity(lot);
      mainStore.addNotification({ content: 'Lot successfully reordered', color: 'success' });
    } catch (e) {
      mainStore.checkApiError(e);
    }
  }

  async function deleteLot(id: number) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Lot',
        id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Lot successfully deleted', color: 'success' });
    } catch (e) {
      mainStore.checkApiError(e);
    }
  }

  async function getGroupLots(groupId: number, query?: LotQuery) {
    try {
      const filters = ref<Filter[]>(
        ((query?.status != null)
          ? [
            { field: 'group_id', op: 'eq', value: groupId },
            { field: 'status', op: 'eq', value: query.status }
          ]
          : [
            { field: 'group_id', op: 'eq', value: groupId }
          ]) as Filter[]
      );
      const climit = query?.limit ? query.limit : limit.value;

      const res = await rpc<{ items: LotDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Lot',
        filters: filters.value,
        limit: climit,
        page: page.value,
      });
      setEntities(res.items);
      total.value = res.total;
      return res.items;
    } catch (e) {
      mainStore.checkApiError(e);
    }
  }

  async function getRecentOrders(groupId: number) {
    return getGroupLots(groupId, { limit: 10, status: LotStatus.Requested });
  }

  async function getFinishedLots(groupId: number) {
    return getGroupLots(groupId, { limit: 10, status: LotStatus.Finished });
  }

  async function getLowLots(groupId: number) {
    return getGroupLots(groupId, { limit: 10, status: LotStatus.Low });
  }

  async function getLotConjugates(lotId: number) {
    try {
      return await rpc<any[]>({
        operation: 'Get',
        return_type: 'Conjugate',
        filters: [{ field: 'lot_id', op: 'eq', value: lotId }],
      });
    } catch (e) {
      mainStore.checkApiError(e);
      return [];
    }
  }

  async function getLotValidations(lotId: number) {
    try {
      return await rpc<any[]>({
        operation: 'Get',
        return_type: 'Validation',
        filters: [{ field: 'lot_id', op: 'eq', value: lotId }],
      });
    } catch (e) {
      mainStore.checkApiError(e);
      return [];
    }
  }

  // 🧼 Return store API
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
    lots,
    listLots,
    getLotById,
    getLot,
    getCsv,

    // mutations & actions
    fetchByIds,
    fetchByIdsV2,
    search,
    loadListQuery,
    reloadListQuery,
    reset,
    resetListQuery,
    setEntities,
    setEntity,
    addEntity,
    updateEntity,
    createLot,
    updateLot,
    updateLotArchiveState,
    updateLotStatus,
    reorderLot,
    deleteLot,
    getGroupLots,
    getRecentOrders,
    getFinishedLots,
    getLowLots,
    getLotConjugates,
    getLotValidations,
  };
});
