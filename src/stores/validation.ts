import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Filter } from '@/modules/json/api';
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';
import { useCloneStore } from '@/stores/clone';
import { useProteinStore } from '@/stores/protein';
import { useConjugateStore } from '@/stores/conjugate';
import { useSpeciesStore } from '@/stores/species';
import { useUserStore } from '@/stores/user';
import { useMemberStore } from '@/stores/member';
import { useLotStore } from '@/stores/lot';
import { api as validationFileApi } from '@/modules/validation_file/api';

import type {
  ValidationDto,
  CreateValidationDto,
  UpdateValidationDto,
} from '@/modules/validation/types';
import type { UpdateStateDto } from '@/modules/core/types';

type Id = number;

type ValidationListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  globalFilter?: string;
};

export const useValidationStore = defineStore('validation', () => {
  // 📦 State
  const ids = ref<Id[]>([]);
  const listIds = ref<Id[]>([]);
  const entities = ref<Record<Id, ValidationDto>>({});
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const searchstr = ref('');
  const loading = ref(false);
  const fetchedCloneIds = ref<Record<number, true>>({});

  const mainStore = useMainStore();
  const cloneStore = useCloneStore();
  const proteinStore = useProteinStore();
  const conjugateStore = useConjugateStore();
  const speciesStore = useSpeciesStore();
  const userStore = useUserStore();
  const memberStore = useMemberStore();
  const lotStore = useLotStore();
  const activeListQuery = ref<ValidationListQuery | null>(null);

  // 🧠 Getters
  const validations = computed(() => ids.value.map((id) => entities.value[id]));
  const listValidations = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((validation): validation is ValidationDto => validation !== undefined)
  );
  const getValidation = (id: Id) => entities.value[id];
  const hasValidation = (id: Id) => id in entities.value;

  const cloneValidationMap = computed(() => {
    const map: Record<number, ValidationDto[]> = {};
    for (const id of ids.value) {
      const v = entities.value[id];
      if (!v || !('cloneId' in v)) continue;
      const cId = (v as any).cloneId as number;
      if (!cId) continue;
      (map[cId] ??= []).push(v);
    }
    return map;
  });

  const lotValidationMap = computed(() => {
    const map: Record<number, ValidationDto[]> = {};
    for (const id of ids.value) {
      const v = entities.value[id];
      if (!v || !('lotId' in v)) continue;
      const cId = (v as any).lotId as number;
      if (!cId) continue;
      (map[cId] ??= []).push(v);
    }
    return map;
  });

  // 🧱 Mutations
  function setEntities(payload: ValidationDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(payload: ValidationDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id);
    entities.value[payload.id] = payload;
  }

  function addEntity(payload: ValidationDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id);
    entities.value[payload.id] = payload;
    total.value = ids.value.length;
  }

  function updateEntity(payload: ValidationDto) {
    entities.value[payload.id] = payload;
  }

  function deleteEntity(id: Id) {
    ids.value = ids.value.filter((x) => x !== id);
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
    searchstr.value = '';
    loading.value = false;
    fetchedCloneIds.value = {};
    activeListQuery.value = null;
  }

  function resetListQuery() {
    listIds.value = [];
    activeListQuery.value = null;
    loading.value = false;
  }

  // 🚀 RPC Actions
  async function createValidation(payload: CreateValidationDto) {
    try {
      const data = await rpc<{ id?: Id } | ValidationDto>({
        operation: 'Insert',
        return_type: 'Validation',
        payload: payload,
      });
      const id = typeof (data as { id?: Id }).id === 'number' ? (data as { id: Id }).id : undefined;
      const entity = typeof id === 'number' ? await getValidationById(id) : undefined;
      mainStore.addNotification({ content: 'Validation successfully created', color: 'success' });
      return entity;
    } catch (error) {
      mainStore.checkApiError(error);
    }
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
      const res = await rpc<{ items: ValidationDto[] }>({
        operation: 'Get',
        return_type: 'Validation',
        filters: [{ field: 'id', op: 'in', value: missingIds }],
      });

      setEntities(res.items);
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function getValidationById(id: Id) {
    try {
      const res = await rpc<{ items: ValidationDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Validation',
        filters: [{ field: 'id', op: 'eq', value: id }],
        limit: 1,
        page: 1,
      });
      const data = res.items[0];
      if (!data) {
        return undefined;
      }
      setEntity(data);
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function fetchByCloneIds(groupId: number, cloneIds: number[]) {
    const uniqueCloneIds = [...new Set(cloneIds)].filter((id): id is number => Number.isInteger(id))
    const missingCloneIds = uniqueCloneIds.filter(id => !(id in fetchedCloneIds.value))

    if (missingCloneIds.length === 0) {
      return []
    }

    const relationLimit = Math.min(1000, Math.max(200, missingCloneIds.length * 4))
    try {
      const res = await rpc<{ items: ValidationDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Validation',
        filters: [
          { field: 'group_id', op: 'eq', value: groupId },
          { field: 'clone_id', op: 'in', value: missingCloneIds },
        ],
        limit: relationLimit,
        page: 1,
      });

      if (res.total > res.items.length) {
        console.warn(
          '[validationStore.fetchByCloneIds] relation hydration truncated',
          {
            groupId,
            cloneCount: missingCloneIds.length,
            fetched: res.items.length,
            total: res.total,
            relationLimit,
          },
        )
      }

      setEntities(res.items);
      total.value = res.total;
      if (res.total <= res.items.length) {
        for (const cloneId of missingCloneIds) {
          fetchedCloneIds.value[cloneId] = true;
        }
      }
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function ensureForCloneIds(groupId: number, cloneIds: number[]) {
    return fetchByCloneIds(groupId, cloneIds)
  }

  async function ensureListRelations(groupId: number, itemIds: readonly number[]) {
    if (groupId <= 0) {
      return;
    }

    const visibleValidations = itemIds
      .map((id) => entities.value[id])
      .filter((validation): validation is ValidationDto => validation !== undefined);

    const cloneIds = visibleValidations
      .map((validation) => validation.cloneId)
      .filter((id): id is number => typeof id === 'number');

    await cloneStore.fetchByIds(cloneIds);

    const conjugateIds = visibleValidations
      .map((validation) => validation.conjugateId)
      .filter((id): id is number => typeof id === 'number');

    await conjugateStore.fetchByIds(groupId, conjugateIds);

    const speciesIds = visibleValidations
      .map((validation) => validation.speciesId)
      .filter((id): id is number => typeof id === 'number');

    await speciesStore.fetchByIds(groupId, speciesIds);

    const lotIds = visibleValidations
      .map((validation) => validation.lotId)
      .filter((id): id is number => typeof id === 'number');

    await lotStore.fetchByIds(groupId, lotIds);

    const memberIds = visibleValidations
      .map((validation) => validation.createdBy)
      .filter((id): id is number => typeof id === 'number');

    await memberStore.fetchByIds(memberIds);

    const userIds = [...new Set(
      memberIds
        .map((id) => memberStore.getMemberById(id)?.userId)
        .filter((id): id is number => typeof id === 'number')
    )];

    await userStore.fetchByIds(userIds);

    const proteinIds = [...new Set(
      cloneIds
        .map((id) => cloneStore.getClone(id)?.proteinId)
        .filter((id): id is number => typeof id === 'number')
    )];

    await proteinStore.fetchByIds(groupId, proteinIds);
  }

  async function loadListQuery(query: ValidationListQuery) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Validation',
        filters: query.filters,
        limit: limit.value,
        page: page.value,
        order: query.order,
        global_filter: query.globalFilter,
      });

      const missingIDs = items.filter((id) => !(id in entities.value));
      if (missingIDs.length > 0) {
        await fetchByIds(missingIDs);
      }

      listIds.value = items;
      total.value = search_total;

      await ensureListRelations(query.groupId, items);

      return listValidations.value;
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

  async function updateValidation(payload: { id: Id; data: UpdateValidationDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Validation',
        id: payload.id,
        payload: payload.data,
      });
      const data = await getValidationById(payload.id);
      mainStore.addNotification({ content: 'Validation successfully updated', color: 'success' });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function updateValidationArchiveState(payload: { id: Id; data: UpdateStateDto }) {
    try {
      const data = await rpc<ValidationDto>({
        operation: 'Update',
        return_type: 'Validation',
        id: payload.id,
        payload: payload.data,
      });
      updateEntity(data);
      mainStore.addNotification({
        content: `Validation successfully ${payload.data.state ? 'archived' : 'unarchived'}`,
        color: 'success',
      });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function deleteValidation(id: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Validation',
        id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Validation successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function getGroupValidations(groupId: Id) {
    loading.value = true;
    try {
      const filters = ref<Filter[]>(
        searchstr.value ?

          [{ field: 'group_id', op: 'eq', value: groupId },
          { field: 'name', op: 'contains', value: searchstr.value }]
          :
          [{ field: 'group_id', op: 'eq', value: groupId }]

      );

      const res = await rpc<{ items: ValidationDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Validation',
        filters: filters.value,
        limit: limit.value,
        page: page.value,
      });
      setEntities(res.items);
      total.value = res.total;
      return res.items;
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    } finally {
      loading.value = false;
    }
  }

  // 📤 File actions (if RPC supports file uploads)
  async function uploadValidationFile(payload: { validationId: Id; formData: FormData }) {
    try {
      const res = await validationFileApi.uploadValidationFile(
        payload.validationId,
        payload.formData,
      );
      mainStore.addNotification({ content: 'Validation file successfully uploaded', color: 'success' });
      return res;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function deleteValidationFile(fileId: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'ValidationFile',
        id: fileId,
      });
      mainStore.addNotification({ content: 'Validation file successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  // 🔍 UI helpers
  function setSearch(value: string) {
    searchstr.value = value;
    page.value = 1;
  }

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
    searchstr,
    loading,

    // Getters
    validations,
    listValidations,
    getValidation,
    hasValidation,
    cloneValidationMap,
    lotValidationMap,

    // Mutations
    reset,
    resetListQuery,
    setEntities,
    setEntity,
    addEntity,
    updateEntity,

    // RPC Actions
    fetchByIds,
    search,
    loadListQuery,
    reloadListQuery,
    createValidation,
    getValidationById,
    updateValidation,
    updateValidationArchiveState,
    deleteValidation,
    getGroupValidations,
    uploadValidationFile,
    deleteValidationFile,
    fetchByCloneIds,
    ensureForCloneIds,

    // UI
    setSearch,
    setPage,
    setLimit,
  };
});
