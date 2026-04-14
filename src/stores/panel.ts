import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { rpc, rpcSearch } from '@/modules/json/api';
import type { SearchFilter, SearchOrder } from '@/modules/json/api';
import { useMainStore } from '@/stores/main';
import { useUserStore } from '@/stores/user';
import { useMemberStore } from '@/stores/member';

import type {
  CreatePanelDto,
  DuplicatePanelDto,
  UpdatePanelDto,
  PanelDto,
  PanelElementDataDto,
} from '@/modules/panel/types';
import type { UpdateStateDto } from '@/modules/core/types';

type Id = number;

type PanelListQuery = {
  groupId: number;
  filters: SearchFilter[];
  order: SearchOrder;
  showAll?: boolean;
  globalFilter?: string;
};

export const usePanelStore = defineStore('panel', () => {
  // 📦 State
  const ids = ref<Id[]>([]);
  const listIds = ref<Id[]>([]);
  const entities = ref<Record<Id, PanelDto>>({});
  const activePanelTagId = ref<Id | null>(null);
  const page = ref(1);
  const limit = ref(50);
  const total = ref(0);
  const loading = ref(false);

  const mainStore = useMainStore();
  const userStore = useUserStore();
  const memberStore = useMemberStore();
  const activeListQuery = ref<PanelListQuery | null>(null);

  // 🧠 Getters
  const panels = computed(() => ids.value.map((id) => entities.value[id]));
  const listPanels = computed(() =>
    listIds.value
      .map((id) => entities.value[id])
      .filter((panel): panel is PanelDto => panel !== undefined)
  );
  const getPanel = (id: Id) => entities.value[id];
  const hasPanel = (id: Id) => id in entities.value;
  const getActivePanelTagId = computed(() => activePanelTagId.value);

  // 🛠️ Mutations
  function setEntities(payload: PanelDto[]) {
    for (const item of payload) {
      entities.value[item.id] = item;
    }
    const newIds = payload.map(item => item.id);
    const mergedIds = new Set([...ids.value, ...newIds]);
    ids.value = Array.from(mergedIds);
  }

  function setEntity(payload: PanelDto) {
    if (!ids.value.includes(payload.id)) ids.value.push(payload.id);
    entities.value[payload.id] = payload;
    total.value = ids.value.length;
  }

  function addEntity(payload: PanelDto) {
    if (!ids.value.includes(payload.id)) ids.value.unshift(payload.id);
    entities.value[payload.id] = payload;
    total.value = ids.value.length;
  }

  function updateEntity(payload: PanelDto) {
    entities.value[payload.id] = payload;
  }

  function deleteEntity(id: Id) {
    ids.value = ids.value.filter((x) => x !== id);
    delete entities.value[id];
    total.value = ids.value.length;
  }

  function setActivePanelTag(id: Id | null) {
    activePanelTagId.value = id;
  }

  function reset() {
    ids.value = [];
    listIds.value = [];
    entities.value = {};
    activePanelTagId.value = null;
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
  async function createPanel(payload: CreatePanelDto) {
    try {
      const data = await rpc<{ id?: Id } | PanelDto>({
        operation: 'Insert',
        return_type: 'Panel',
        payload: payload,
      });
      const id = (data as PanelDto)?.id;
      const entity = typeof id === 'number' ? await getPanelById(id) : undefined;
      mainStore.addNotification({ content: 'Panel successfully created', color: 'success' });
      return entity;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function duplicatePanel(payload: { id: Id; data: DuplicatePanelDto }) {
    try {
      const data = await rpc<PanelDto>({
        operation: 'Duplicate',
        return_type: 'Panel',
        id: payload.id,
        payload: payload.data,
      });
      addEntity(data);
      mainStore.addNotification({ content: 'Panel successfully duplicated', color: 'success' });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function getPanelById(id: Id) {
    loading.value = true;
    try {
      const data = await rpc<{ items: PanelDto[]; total?: number }>({
        operation: 'Get',
        return_type: 'Panel',
        filters: [{ field: 'id', op: 'eq', value: id }],
        limit: 1,
        page: 1,
      });
      const panel = data.items?.[0];
      if (panel) {
        setEntity(panel);
      }
      return panel;
    } catch (error) {
      mainStore.checkApiError(error);
    } finally {
      loading.value = false;
    }
  }

  async function updatePanel(payload: { id: Id; data: UpdatePanelDto }) {
    try {
      await rpc<unknown>({
        operation: 'Update',
        return_type: 'Panel',
        id: payload.id,
        payload: payload.data,
      });
      const data = await getPanelById(payload.id);
      mainStore.addNotification({ content: 'Panel successfully updated', color: 'success' });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function updatePanelArchiveState(payload: { id: Id; data: UpdateStateDto }) {
    try {
      const data = await rpc<PanelDto>({
        operation: 'Update',
        return_type: 'Panel',
        id: payload.id,
        payload: payload.data,
      });
      updateEntity(data);
      mainStore.addNotification({
        content: `Panel successfully ${payload.data.state ? 'archived' : 'unarchived'}`,
        color: 'success',
      });
      return data;
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function deletePanel(id: Id) {
    try {
      await rpc<void>({
        operation: 'Delete',
        return_type: 'Panel',
        id,
      });
      deleteEntity(id);
      mainStore.addNotification({ content: 'Panel successfully deleted', color: 'success' });
    } catch (error) {
      mainStore.checkApiError(error);
    }
  }

  async function getGroupPanels(groupId: Id) {
    loading.value = true;
    try {
      const res = await rpc<{ items: PanelDto[]; total: number }>({
        operation: 'Get',
        return_type: 'Panel',
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

  async function getPanelElements(panelId: Id): Promise<PanelElementDataDto[]> {
    try {
      const res = await rpc<PanelElementDataDto[] | { items?: PanelElementDataDto[] }>({
        operation: 'Get',
        return_type: 'PanelElement',
        filters: [{ field: 'panel_id', op: 'eq', value: panelId }],
      });
      return Array.isArray(res) ? res : res.items ?? [];
    } catch (error) {
      mainStore.checkApiError(error);
      return [];
    }
  }

  async function fetchByIds(itemIdss: readonly number[]) {
    if (!Array.isArray(itemIdss)) {
      console.error("fetchByIds expected number[], got:", itemIdss);
      return [];
    }
    const uniqueIds = [...new Set(itemIdss)].filter((x): x is number => Number.isInteger(x));

    const missingIds = uniqueIds.filter(id => !(id in entities.value));
    if (missingIds.length === 0) return [];

    try {
      const res = await rpc<{ items: PanelDto[] }>({
        operation: 'Get',
        return_type: 'Panel',
        filters: [{ field: 'id', op: 'in', value: missingIds }],
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

    const visiblePanels = itemIds
      .map((id) => entities.value[id])
      .filter((panel): panel is PanelDto => panel !== undefined);

    const memberIds = visiblePanels
      .map((panel) => panel.createdBy)
      .filter((id): id is number => typeof id === 'number');

    await memberStore.fetchByIds(memberIds);

    const userIds = [...new Set(
      memberIds
        .map((id) => memberStore.getMemberById(id)?.userId)
        .filter((id): id is number => typeof id === 'number')
    )];

    await userStore.fetchByIds(userIds);
  }

  async function loadListQuery(query: PanelListQuery) {
    activeListQuery.value = query;
    loading.value = true;
    try {
      const { items, search_total } = await rpcSearch({
        return_type: 'Panel',
        filters: query.filters,
        limit: limit.value,
        page: page.value,
        order: query.order,
        show_all: query.showAll,
        global_filter: query.globalFilter,
      });

      const missingIDs = items.filter(id => !(id in entities.value));
      if (missingIDs.length > 0) {
        await fetchByIds(missingIDs);
      }

      listIds.value = items;
      total.value = search_total;

      await ensureListRelations(query.groupId, items);

      return listPanels.value;
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

  // 🔍 UI Helpers
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
    activePanelTagId,
    page,
    limit,
    total,
    loading,

    // 🧠 Getters
    panels,
    listPanels,
    getPanel,
    hasPanel,
    getActivePanelTagId,

    // 🔨 Mutations + Actions
    reset,
    resetListQuery,
    setEntities,
    setEntity,
    addEntity,
    updateEntity,
    setActivePanelTag,

    // RPC
    fetchByIds,
    search,
    loadListQuery,
    reloadListQuery,
    createPanel,
    duplicatePanel,
    getPanelById,
    updatePanel,
    updatePanelArchiveState,
    deletePanel,
    getGroupPanels,
    getPanelElements,

    // UI helpers
    //setSearch,
    setPage,
    setLimit,
  };
});
