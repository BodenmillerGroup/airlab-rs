import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';
import { useProteinStore } from '@/stores/protein';
import { useSpeciesStore } from '@/stores/species';
import { useValidationStore } from '@/stores/validation';

import type { CloneDto, CreateCloneDto, UpdateCloneDto } from '@/modules/clone/types';
import type { UpdateStateDto } from '@/modules/core/types';

type CloneListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  globalFilter?: string;
};

export const useCloneStore = defineStore('clone', () => {
  // 🔧 State
  const ids = ref<number[]>([]);
  const listIds = ref<number[]>([]);
  const entities = ref<Record<number, CloneDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const loading = ref(false);
  const revision = ref(0);

  const mainStore = useMainStore();
  const proteinStore = useProteinStore();
  const speciesStore = useSpeciesStore();
  const validationStore = useValidationStore();
  const activeListQuery = ref<CloneListQuery | null>(null);

  // 🧠 Getters
  const clones = computed(() => ids.value.map((id) => entities.value[id]));
  const listClones = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((clone): clone is CloneDto => clone !== undefined)
  );
  const getClone = (id: number) => entities.value[id];

  const getGroupClones = (groupId: number) => {
    return ids.value.map(id => entities.value[id]).filter(clone => clone.groupId === groupId);
  };

  type CloneCsvItem = Pick<
    CloneDto,
    'id' | 'name' | 'isotype' | 'epitope' | 'isPhospho' | 'isPolyclonal'
  > & {
    proteinName?: string;
    speciesName?: string;
    protein?: { name?: string };
    species?: { name?: string };
  };

  const getCsv = (items: ReadonlyArray<CloneCsvItem>) => {
    const separator = ';';
    const header = [
      'Id',
      'Clone',
      'Protein',
      'Host',
      'Isotype',
      'Epitope',
      'Phospho',
      'Polyclonal',
    ];
    const lines = items.map((item) => {
      const line = [
        item.id,
        item.name,
        item.proteinName ?? (item as any).protein?.name ?? '',
        item.speciesName ?? (item as any).species?.name ?? '',
        item.isotype,
        item.epitope,
        item.isPhospho,
        item.isPolyclonal,
      ];
      return line.join(separator);
    });
    return [header.join(separator), ...lines].join('\n');
  };

  function setEntities(payload: CloneDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
    revision.value++;
  }

  function deleteEntity(id: number) {
    ids.value = ids.value.filter((existingId) => existingId !== id);
    const { [id]: _, ...rest } = entities.value;
    entities.value = rest;
    revision.value++;
  }

  function reset() {
    ids.value = [];
    entities.value = {};
    page.value = 1;
    total.value = 0;
    resetListQuery();
    revision.value++;
  }

  function resetListQuery() {
    listIds.value = [];
    activeListQuery.value = null;
  }

  // 🔁 RPC-based Actions
  async function createClone(payload: CreateCloneDto) {
    try {
      const clone = await rpc<{ id?: number } | CloneDto>({
        operation: 'Insert',
        return_type: 'Clone',
        payload: payload,
      });
      const id = (clone as CloneDto)?.id;
      if (typeof id === 'number') {
        await getCloneById(id);
      }
      mainStore.addNotification({ content: 'Clone successfully created', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function updateClone(payload: { id: number; data: UpdateCloneDto }) {
    const current = entities.value[payload.id];

    try {
      if (current) {
        setEntities([
          {
            ...current,
            ...payload.data,
          },
        ]);
      }

      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Clone',
        id: payload.id,
        payload: payload.data,
      });

      await getCloneById(payload.id);
      mainStore.addNotification({ content: 'Clone successfully updated', color: 'success' });
    } catch (error) {
      if (current) {
        setEntities([current]);
      }
      mainStore.checkApiError(error);
    }
  }

  async function deleteClone(id: number) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Clone',
        payload: id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Clone successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function updateCloneArchiveState(payload: { id: number; data: UpdateStateDto }) {
    try {
      const clone = await rpc<CloneDto>({
        operation: 'Update',
        return_type: 'Clone',
        payload: payload,
      });
      setEntities([clone]);
      mainStore.addNotification({
        content: `Clone successfully ${payload.data.state ? 'archived' : 'unarchived'}`,
        color: 'success',
      });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  const getByIds = (ids: number[]): CloneDto[] =>
    ids
      .map(id => clones.value.find(clone => clone.id === id))
      .filter((c): c is CloneDto => c !== undefined);

  async function getCloneById(id: number) {
    try {
      const clone = await rpc<CloneDto>({
        operation: 'Get',
        return_type: 'Clone',
        payload: id,
      });
      setEntities([clone]);
      return clone;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function fetchByIds(itemIds: readonly number[], forceRefresh = false) {
    if (!Array.isArray(itemIds)) {
      console.error("fetchByIds expected number[], got:", itemIds);
      return [];
    }
    const uniqueIds = [...new Set(itemIds)].filter((x): x is number => Number.isInteger(x));

    const targetIds = forceRefresh
      ? uniqueIds
      : uniqueIds.filter(id => !(id in entities.value));
    if (targetIds.length === 0) return [];

    try {
      const res = await rpc<{ items: CloneDto[] }>({
        operation: 'Get',
        return_type: 'Clone',
        filters: [{ field: 'id', op: 'in', value: targetIds }],
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

    const visibleClones = itemIds
      .map((id) => entities.value[id])
      .filter((clone): clone is CloneDto => clone !== undefined);

    const proteinIds = visibleClones
      .map((clone) => clone.proteinId)
      .filter((id): id is number => typeof id === 'number');

    const speciesIds = visibleClones
      .map((clone) => clone.speciesId)
      .filter((id): id is number => typeof id === 'number');

    await proteinStore.ensureByIds(groupId, proteinIds);
    await speciesStore.ensureByIds(groupId, speciesIds);
    await validationStore.ensureForCloneIds(groupId, [...new Set(itemIds)]);
  }

  async function loadListQuery(query: CloneListQuery) {
    loading.value = true;
    try {
      activeListQuery.value = query;
      const { items, search_total } = await rpcSearch({
        return_type: 'Clone',
        filters: query.filters,
        limit: limit.value,
        page: page.value,
        order: query.order,
        global_filter: query.globalFilter,
      });

      const missingIDs = items.filter(id => !(id in entities.value));
      if (missingIDs.length > 0) {
        await fetchByIds(missingIDs);
      }

      listIds.value = items;
      total.value = search_total;
      await ensureListRelations(query.groupId, items);

      return listClones.value;
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

  async function fetchGroupClonesQ(groupId: number) {
    return rpc<CloneDto[]>({
      operation: 'Get',
      return_type: 'Clone',
      filters: [{ field: 'group_id', op: '=', value: groupId }],
      limit: 10,
      page: 1,
    });
  }

  async function fetchGroupArchivedClones(groupId: number) {
    loading.value = true;
    try {
      const res = await rpc<CloneDto[]>({
        operation: 'Get',
        return_type: 'Clone',
        filters: [
          { field: 'group_id', op: 'eq', value: groupId },
          { field: 'state', op: 'eq', value: 'archived' },
        ],
      });
      setEntities(res);
    } catch (error) {
      mainStore.checkApiError(error);
    } finally {
      loading.value = false;
    }
  }

  async function getCloneValidations(cloneId: number) {
    try {
      return await rpc<any[]>({
        operation: 'Get',
        return_type: 'Validation',
        filters: [{ field: 'clone_id', op: 'eq', value: cloneId }],
      });
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  // 🧼 Exposed properties and methods
  return {
    // state
    ids,
    listIds,
    entities,
    page,
    limit,
    total,
    loading,
    revision,

    // getters
    clones,
    listClones,
    getClone,
    getCsv,
    getGroupClones,
    fetchByIds,

    // actions
    reset,
    resetListQuery,
    createClone,
    updateClone,
    deleteClone,
    getCloneById,
    getByIds,
    updateCloneArchiveState,
    search,
    loadListQuery,
    reloadListQuery,
    fetchGroupClonesQ,
    fetchGroupArchivedClones,
    getCloneValidations,
  };
});
