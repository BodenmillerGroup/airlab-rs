<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Lots</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn text @click="exportFile" color="primary">Export CSV</v-btn>
        <v-btn text :to="`/main/groups/${activeGroupId}/lots/create`" color="primary">Create Lot</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-expansion-panels>
      <v-expansion-panel>
        <v-expansion-panel-title>
          <FilterSummary />
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <FilterField key-name="proteinName" class="mb-1" />
          <FilterField key-name="cloneName" class="mb-1" />
          <FilterField key-name="statusName" class="mb-1" />
          <FilterField key-name="providerName" class="mb-1" />
          <FilterField key-name="collectionId" class="mb-1" />
          <FilterField key-name="validationApplication" class="mb-1" />
          <FilterField key-name="validationStatus" class="mb-1" />
          <FilterField key-name="lotName" class="mb-1" />
          <FilterField key-name="lotNumber" class="mb-1" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-text-field
      v-model="globalSearch"
      label="Search all lot fields"
      variant="solo"
      density="comfortable"
      clearable
      prepend-inner-icon="mdi-magnify"
      class="mt-3 mb-3"
    />

    <v-card>

      <v-data-table-server
        :headers="headers"
        :items="items"
        :loading="lotStore.loading"
        :items-length="lotStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        show-expand
        v-model:expanded="expanded"
        v-model:sort-by="sortBy"
        item-value="id"
      >
        <template #item.provider="{ item }">
          <span v-if="item.providerId">
            <router-link
              v-if="item.providerId"
              class="link"
              :to="{
                name: 'main-group-providers-edit',
                params: { groupId: activeGroupId, id: item.providerId },
              }"
            >
             {{ providerStore.getProvider(item.providerId)?.name || 'Unknown' }}
          </router-link>

          </span>
          <span v-else>
            No provider
         </span>
        </template>

        <template #item.clone="{ item }">
          <router-link
            class="link"
            :to="{
              name: 'main-group-clones-edit',
              params: { groupId: activeGroupId, id: item.cloneId || 0 },
            }"
          >
             {{ cloneStore.getClone(item.cloneId)?.name || 'Unknown' }}
          </router-link>
        </template>

        <template #item.collection="{ item }">
          <span v-if="item.collectionId">
            <router-link
              class="link"
              :to="{
                name: 'main-group-collections-edit',
                params: { groupId: activeGroupId, id: item.collectionId },
              }"
            >
              {{ collectionStore.getCollection(item.collectionId)?.name || 'Unknown' }}
            </router-link>
          </span>
          <span v-else>—</span>
        </template>

        <template #item.storage="{ item }">
          <span v-if="item.storageId">
            <router-link
              class="link"
              :to="{
                name: 'main-group-storage-edit',
                params: { groupId: activeGroupId, id: item.storageId },
              }"
            >
              {{ storageStore.getStorage(item.storageId)?.name || 'Unknown' }}
            </router-link>
          </span>
          <span v-else>—</span>
        </template>

        <template #item.status="{ item }">
          <v-chip :color="getLotStatusColor(item.status as LotStatus)" class="mr-1" small dark label>
            {{ lotStatusToString(item.status) }}
          </v-chip>
        </template>

        <template #item.validations="{ item }">
          <template v-if="lotValidationMap[item.id]?.length">
            <v-chip
              v-for="validation in lotValidationMap[item.id]"
              :key="validation.id"
              :color="getStatusColor(validation)"
              class="mr-1"
              size="x-small"
              @click.stop="showValidation(validation.id)"
            >
              {{ applicationToString(validation.application) }}
            </v-chip>
          </template>
          <span v-else>—</span>
        </template>

        <template #item.action="{ item }">
          <div class="action-buttons">
            <v-menu bottom left>
              <template #activator="{ props }">
                <v-btn icon v-bind="props">
                  <v-icon>mdi-dots-vertical</v-icon>
                </v-btn>
              </template>

              <v-list dense>
                <template v-for="status in statuses" :key="status.value">
                    <v-list-item
                      v-if="item.status !== status.value && (status.value <= 4 ? isGroupAdmin : true)"
                      @click="status.value === 4 ? updateLotStatusAndNumber(item.id, status.value as LotStatus) : updateLotStatus(item.id, status.value as LotStatus)"
                    >
                      <v-icon color="primary">mdi-flask</v-icon>
                      <v-list-item-title>Mark as {{ status.text }}</v-list-item-title>
                    </v-list-item>
                </template>

                <v-divider />

                <v-list-item
                  :to="{
                    name: 'main-group-conjugates-create',
                    params: { groupId: activeGroupId },
                    query: { lotId: item.id, cloneId: item.cloneId },
                  }"
                >
                  <v-icon color="primary">mdi-plus-circle-outline</v-icon>
                  <v-list-item-title>Add Conjugate</v-list-item-title>
                </v-list-item>

                <v-list-item
                  :to="{
                    name: 'main-group-validations-create',
                    params: { groupId: activeGroupId },
                    query: { cloneId: item.cloneId, lotId: item.id },
                  }"
                >
                  <v-icon color="primary">mdi-plus-circle-outline</v-icon>
                  <v-list-item-title>Add Validation</v-list-item-title>
                </v-list-item>

                <v-divider />

                <v-list-item @click="reorderLot(item.id)">
                  <v-icon color="primary">mdi-shopping-outline</v-icon>
                  <v-list-item-title>Reorder</v-list-item-title>
                </v-list-item>

                <v-divider />

                <v-list-item
                  :to="{
                    name: 'main-group-lots-edit',
                    params: { groupId: activeGroupId, id: item.id },
                  }"
                >
                  <v-icon color="grey">mdi-pencil-outline</v-icon>
                  <v-list-item-title>Edit</v-list-item-title>
                </v-list-item>

                <v-list-item
                  v-if="isGroupAdmin"
                  @click="updateLotArchiveState(item.id, !item.isArchived)"
                >
                    <v-icon color="red accent-1">
                      {{ item.isArchived ? "mdi-archive-arrow-up-outline" : "mdi-archive-arrow-down-outline" }}
                    </v-icon>
                    <v-list-item-title>
                      {{ item.isArchived ? "Unarchive" : "Archive" }}
                    </v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>

            <v-tooltip bottom>
              <template #activator="{ props }">
                <v-btn icon v-bind="props" @click.stop="showDetails(item)">
                  <v-icon>mdi-information-outline</v-icon>
                </v-btn>
              </template>
              <span>Show details</span>
            </v-tooltip>
          </div>
        </template>

        <template #expanded-row="{ columns, item }">
          <td :colspan="columns?.length || 9">
            <LotExpandedView :lot="item" />
          </td>
        </template>
      </v-data-table-server>
    </v-card>

    <v-navigation-drawer v-model="drawer" right fixed temporary width="600">
      <LotDetailsView v-if="drawer" :lot="detailsItem" />
    </v-navigation-drawer>

    <v-navigation-drawer v-model="validationDrawer" right fixed temporary width="600">
      <ValidationDetailsView v-if="validationDrawer" :validation-id="selectedValidationId" />
    </v-navigation-drawer>
  </v-col>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { storeToRefs } from "pinia";
import { lotStatusToString, applicationToString  } from '@/utils/converters'
import { useGroupStore } from '@/stores/group';
import { useLotStore } from '@/stores/lot';
import { useProviderStore } from '@/stores/provider';
import { useCloneStore } from '@/stores/clone';
import { useCollectionStore } from '@/stores/collection';
import { useStorageStore } from '@/stores/storage';
import { useValidationStore } from '@/stores/validation';
import { useFilterStore } from '@/stores/useFilterStore'
import { getLotStatusColor, getStatusColor } from '@/utils/converters';
import { exportCsv } from '@/utils/exporters';
import { LotStatus } from '@/modules/lot/LotStatus';
import { buildLotStatusUpdatePayload } from '@/modules/lot/statusUpdate';
import { applicationEnum } from '@/utils/enums';
import type { LotDto } from '@/modules/lot/types';

import LotExpandedView from '@/views/main/group/lots/LotExpandedView.vue';
import LotDetailsView from '@/views/main/group/lots/LotDetailsView.vue';
import ValidationDetailsView from '@/views/main/group/validations/ValidationDetailsView.vue';
import type { ValidationDto } from "@/modules/validation/types";
import FilterField from '@/components/FilterField.vue'
import FilterSummary from '@/components/FilterSummary.vue'
import { useServerTablePagination } from '@/composables/useServerTablePagination'

// 📦 Stores
const groupStore = useGroupStore();
const lotStore = useLotStore();
const { page, limit } = storeToRefs(lotStore);
const providerStore = useProviderStore();
const cloneStore = useCloneStore();
const collectionStore = useCollectionStore();
const storageStore = useStorageStore();
const validationStore = useValidationStore();
const filterStore = useFilterStore()
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit)

// 📊 State
const drawer = ref(false);
const detailsItem = ref<LotDto | null>(null);
const validationDrawer = ref(false);
const selectedValidationId = ref<number | null>(null);
const search = ref('');

const statusFilter = ref<number[]>([]);
const providerFilter = ref<number[]>([]);
const validationApplicationFilter = ref<number[]>([]);
const validationStatusFilter = ref<number[]>([]);

// 🔧 Static values
const applications = applicationEnum;
const validationStatuses = [
  { value: 0, text: 'Yes' },
  { value: 1, text: 'So-So' },
  { value: 2, text: 'No' },
  { value: 3, text: 'Undefined' },
  { value: -1, text: 'No validations' },
];

const statuses = [
  { value: 0, text: 'Requested' },
  { value: 1, text: 'Approved' },
  { value: 2, text: 'Rejected' },
  { value: 3, text: 'Ordered' },
  { value: 4, text: 'Stock' },
  { value: 5, text: 'Low' },
  { value: 6, text: 'Finished' },
];

const expanded = ref<string[]>([])
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'id', order: 'desc' },
])

const rawHeaders = [
  {
    title: "Id",
    key: "id",
    align: "end",
    filterable: false,
    width: "80",
  },
  {
    title: "Name",
    key: "name",
  },
  {
    title: "Number",
    key: "number",
  },
  {
    title: "Reference",
    key: "reference",
  },
  {
    title: "Provider",
    filterable: false,
    key: "provider",
  },
  {
    title: "Clone",
    key: "clone",
    filterable: false,
  },
  {
    title: "Collection",
    key: "collection",
    filterable: false,
  },
  {
    title: "Storage",
    key: "storage",
    filterable: false,
  },
  {
    title: "Status",
    key: "status",
    filterable: false,
  },
  {
    title: "Validations",
    key: "validations",
    filterable: false,
  },
  {
    title: "Actions",
    key: "action",
    sortable: false,
    filterable: false,
    width: "130",
  },
  {
    title: "",
    key: "data-table-expand",
  },
] as const

const headers = rawHeaders.map((header) => ({
  sortable: header.key !== 'action' && header.key !== 'data-table-expand',
  ...header,
}))



// 🔎 Computed
const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);


const providers = computed(() => {
  return [...providerStore.providers].sort((a, b) => a.name.localeCompare(b.name));
});

import { useLots } from '@/composables/useLots'
const globalSearch = computed({
  get: () => filterStore.filters.lotGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('lotGlobalSearch', value),
})
const { items, loading, reload } = useLots({ sortBy, globalFilter: globalSearch })

const lotValidationMap = computed<Record<number, ValidationDto[]>>(
  () => validationStore.lotValidationMap
)


const filter = (value: any, searchTerm: string, item: any) => {
  if (!searchTerm) return true;
  const term = searchTerm.toLowerCase();
  return (
    item.name?.toLowerCase().includes(term) ||
    item.number?.toLowerCase().includes(term) ||
    item.reference?.toLowerCase().includes(term) ||
    item.provider?.name?.toLowerCase().includes(term) ||
    item.clone?.name?.toLowerCase().includes(term) ||
    item.purpose?.toLowerCase().includes(term)
  );
};

// 🔁 UI Actions
function showDetails(item: LotDto) {
  detailsItem.value = item;
  drawer.value = true;
}

function showValidation(id: number) {
  selectedValidationId.value = id;
  validationDrawer.value = true;
}

// ❌ Filter removals
function removeStatusFilter(item: { value: number }) {
  statusFilter.value = statusFilter.value.filter(i => i !== item.value);
}

function removeProviderFilter(item: { id: number }) {
  providerFilter.value = providerFilter.value.filter(i => i !== item.id);
}

function removeValidationApplicationFilter(item: { value: number }) {
  validationApplicationFilter.value = validationApplicationFilter.value.filter(i => i !== item.value);
}

function removeValidationStatusFilter(item: { value: number }) {
  validationStatusFilter.value = validationStatusFilter.value.filter(i => i !== item.value);
}

// 📤 Export
function exportFile() {
  const csv = lotStore.getCsv(items.value);
  exportCsv(csv, 'lots.csv');
}

// 🧪 Status and Reorder logic
async function updateLotStatus(id: number, status: LotStatus) {
  if (confirm(`Change status to ${lotStatusToString(status)}?`)) {
    const actorId = await resolveCurrentMemberId();
    const actorPayload = actorId ? buildLotStatusUpdatePayload(status, actorId) : {};
    await lotStore.updateLotStatus(id, { status, ...actorPayload });
    await reload();
  }
}

async function updateLotStatusAndNumber(id: number, status: LotStatus) {
  const number = prompt('Enter lot number:');
  if (number) {
    const actorId = await resolveCurrentMemberId();
    const actorPayload = actorId ? buildLotStatusUpdatePayload(status, actorId) : {};
    await lotStore.updateLotStatus(id, { status, lotNumber: number, ...actorPayload });
    await reload();
  }
}

async function reorderLot(id: number) {
  const purpose = prompt('Enter reorder purpose:');
  if (purpose) {
    const actorId = await resolveCurrentMemberId();
    const actorPayload = actorId ? buildLotStatusUpdatePayload(LotStatus.Requested, actorId) : {};
    await lotStore.reorderLot(id, { purpose, ...actorPayload });
    await reload();
  }
}

async function updateLotArchiveState(id: number, state: boolean) {
  if (confirm(`${state ? 'Archive' : 'Unarchive'} this lot?`)) {
    await lotStore.updateLotArchiveState(id, { state });
    await reload();
  }
}

async function deleteLot(id: number) {
  if (confirm('Delete this lot?') && confirm('All children conjugates will be deleted!')) {
    await lotStore.deleteLot(id);
    await reload();
  }
}

async function resolveCurrentMemberId(): Promise<number | null> {
  if (groupStore.myMember?.id) return groupStore.myMember.id;
  const groupId = activeGroupId.value;
  if (!groupId) return null;
  const member = await groupStore.getMyMember(groupId);
  return member?.id ?? groupStore.myMember?.id ?? null;
}

const expandedPanels = ref([])

</script>

<style scoped>
.action-buttons {
  display: inline-flex;
  flex-wrap: nowrap;
  align-items: center;
}
</style>
