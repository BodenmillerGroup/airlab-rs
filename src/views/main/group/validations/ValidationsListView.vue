<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Validations</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn text @click="exportFile()" color="primary">Export CSV</v-btn>
        <v-btn
          text
          :to="`/main/groups/${activeGroupId}/validations/create`"
          color="primary"
        >
          Create Validation
        </v-btn>
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
          <FilterField key-name="speciesName" class="mb-1" />
          <FilterField key-name="validationApplication" class="mb-1" />
          <FilterField key-name="validationStatus" class="mb-1" />
          <FilterField key-name="validationProtocol" class="mb-1" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-text-field
      v-model="globalSearch"
      label="Search all validation fields"
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
        :loading="validationStore.loading"
        :items-length="validationStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        show-expand
        v-model:expanded="expanded"
        v-model:sort-by="sortBy"
        item-value="id"
      >
        <template #item.species="{ item }">
          <router-link
            v-if="item.speciesId"
            class="link"
            :to="{
              name: 'main-group-species-edit',
              params: {
                groupId: activeGroupId,
                id: item.speciesId,
              },
            }"
          >
            {{ item.speciesName }}
          </router-link>
        </template>

        <template #item.clone="{ item }">
          <router-link
            class="link"
            :to="{
              name: 'main-group-clones-edit',
              params: {
                groupId: activeGroupId,
                id: item.cloneId,
              },
            }"
          >
            {{ item.cloneName }}
          </router-link>
        </template>

        <template #item.protein="{ item }">
          <router-link
            v-if="item.proteinId"
            class="link"
            :to="{
              name: 'main-group-proteins-edit',
              params: {
                groupId: activeGroupId,
                id: item.proteinId,
              },
            }"
          >
            {{ item.proteinName }}
          </router-link>
        </template>

        <template #item.lot="{ item }">
          <router-link
            v-if="item.lotId"
            class="link"
            :to="{
              name: 'main-group-lots-edit',
              params: {
                groupId: activeGroupId,
                id: item.lotId,
              },
            }"
          >
            {{ item.lotNumber }}
          </router-link>
        </template>

        <template #item.conjugate="{ item }">
          <router-link
            v-if="item.conjugateId"
            class="link"
            :to="{
              name: 'main-group-conjugates-edit',
              params: {
                groupId: activeGroupId,
                id: item.conjugateId,
              },
            }"
          >
            {{ item.tubeNumber }}
          </router-link>
        </template>

        <template #item.user="{ item }">
          <router-link
            v-if="item.userId"
            class="link"
            :to="{
              name: 'main-admin-users-edit',
              params: { id: item.userId },
            }"
          >
            {{ item.userName }}
          </router-link>
        </template>

        <template #item.application="{ item }">
          {{ applicationToString(item.application) }}
        </template>

        <template #item.status="{ item }">
          <v-tooltip v-if="item.status === 0" location="bottom">
            <template #activator="{ props }">
              <v-icon v-bind="props" color="green">mdi-check-circle-outline</v-icon>
            </template>
            <span>Yes</span>
          </v-tooltip>

          <v-tooltip v-if="item.status === 1" location="bottom">
            <template #activator="{ props }">
              <v-icon v-bind="props" color="orange">mdi-circle-outline</v-icon>
            </template>
            <span>So-So</span>
          </v-tooltip>

          <v-tooltip v-if="item.status === 2" location="bottom">
            <template #activator="{ props }">
              <v-icon v-bind="props" color="red">mdi-cancel</v-icon>
            </template>
            <span>No</span>
          </v-tooltip>

          <v-tooltip v-if="item.status === 3" location="bottom">
            <template #activator="{ props }">
              <v-icon v-bind="props">mdi-help-circle-outline</v-icon>
            </template>
            <span>Undefined</span>
          </v-tooltip>
        </template>

        <template #item.action="{ item }">
          <v-menu location="bottom left">
            <template #activator="{ props }">
              <v-btn icon v-bind="props">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
            <v-list density="compact">
              <v-list-item
                :to="{
                  name: 'main-group-validations-edit',
                  params: {
                    groupId: activeGroupId,
                    id: item.id,
                  },
                }"
              >
                <template #prepend>
                  <v-icon color="grey">mdi-pencil-outline</v-icon>
                </template>
                <v-list-item-title>Edit</v-list-item-title>
              </v-list-item>

              <v-list-item
                v-if="isGroupAdmin"
                @click="updateValidationArchiveState(item.id, !item.isArchived)"
              >
                <template #prepend>
                  <v-icon color="red accent-1">
                    {{
                      item.isArchived
                        ? 'mdi-archive-arrow-up-outline'
                        : 'mdi-archive-arrow-down-outline'
                    }}
                  </v-icon>
                </template>
                <v-list-item-title>
                  {{ item.isArchived ? 'Unarchive' : 'Archive' }}
                </v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>

          <v-tooltip location="bottom">
            <template #activator="{ props }">
              <v-btn v-bind="props" icon @click.stop="showDetails(item)">
                <v-icon>mdi-information-outline</v-icon>
              </v-btn>
            </template>
            <span>Show details</span>
          </v-tooltip>
        </template>

        <template #expanded-row="{ columns, item }">
          <td :colspan="columns.length">
            <v-card flat tile class="my-2">
              <v-card-title>{{ item.id }}</v-card-title>
              <v-card-text>
                <div><strong>Positive control:</strong> {{ item.positiveControl }}</div>
                <div><strong>Negative control:</strong> {{ item.negativeControl }}</div>
              </v-card-text>
            </v-card>
          </td>
        </template>
      </v-data-table-server>
    </v-card>

    <v-navigation-drawer v-model="drawer" location="right" fixed temporary width="600">
      <ValidationDetailsView v-if="drawer" :validation-id="detailsItem.id" />
    </v-navigation-drawer>
  </v-col>
</template>

<script lang="ts" setup>
import { ref, computed, type Ref } from 'vue';
import { storeToRefs } from 'pinia';
import { useRoute } from 'vue-router';
import ValidationDetailsView from '@/views/main/group/validations/ValidationDetailsView.vue';
import FilterField from '@/components/FilterField.vue'
import FilterSummary from '@/components/FilterSummary.vue'
import { useFilterStore } from '@/stores/useFilterStore'
import { exportCsv } from '@/utils/exporters';
import { applicationEnum, statusEnum, antigenRetrievalTypes } from '@/utils/enums';
import { applicationToString } from '@/utils/converters';
import { useGroupStore } from '@/stores/group';
import { useValidationStore } from '@/stores/validation';
import { useSpeciesStore } from '@/stores/species';
import { useServerTablePagination } from '@/composables/useServerTablePagination';

const expanded = ref<any[]>([])

const route = useRoute();

const groupStore = useGroupStore();
const validationStore = useValidationStore();
const { page, limit } = storeToRefs(validationStore);
const filterStore = useFilterStore()
const speciesStore = useSpeciesStore();
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit);

const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const species = computed(() => speciesStore.species);

const rawHeaders = [
  { title: 'Protein', key: 'proteinName', sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? '') },
  { title: 'Clone', key: 'cloneName', sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? '') },
  { title: 'Lot', key: 'lotNumber', sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? '') },
  { title: 'Species', key: 'speciesName', sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? '') },
  { title: 'Conjugate', key: 'tubeNumber', sort: (a: string | number, b: string | number) => Number(a ?? 0) - Number(b ?? 0) },
  { title: 'Application', key: 'application', sortable: true, sort: (a: number, b: number) => applicationToString(a).localeCompare(applicationToString(b)) },
  { title: 'Protocol', key: 'antigenRetrievalType', sortable: true, sort: (a, b) => a?.localeCompare(b) ?? 0, width: '120' },
  { title: 'Created by', key: 'userName', sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? '') },
  { title: 'Status', key: 'status', sortable: true },
  { title: 'Actions', key: 'action', sortable: false, filterable: false, width: '130' },
  { title: '', key: 'data-table-expand' },
];


const headers = rawHeaders.map(h => ({
  sortable: h.key !== 'action' && h.key !== 'data-table-expand',
  ...h,
}))

const search = ref('');
const drawer = ref(false);
const detailsItem = ref<any | null>(null);
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'application', order: 'desc' },
]);

const speciesFilter = ref<number[]>([]);
const applicationFilter = ref<number[]>([]);
const statusFilter = ref<number[]>([]);
const retrievalFilter = ref<string[]>([]);

const applications = applicationEnum;
const statuses = statusEnum;
const antigenRetrievalTypesList = antigenRetrievalTypes;

function showDetails(item: any) {
  detailsItem.value = item;
  drawer.value = true;
}

function exportFile() {
  const csv_headers = [
    'Id',
    'Clone',
    'Protein',
    'Lot',
    'Species',
    'Conjugate',
    'Application',
    'Status',
    'Created By',
    'Created At',
  ];
  const rows = items.value.map(item => [
    item.id,
    item.cloneName,
    item.proteinName,
    item.lotNumber ?? '',
    item.speciesName ?? '',
    item.tubeNumber ?? '',
    applicationToString(item.application),
    item.status,
    item.userName ?? '',
    item.createdAt,
  ]);
  const csv = [csv_headers, ...rows]
    .map(row => row.map(value => `"${String(value ?? '').replace(/"/g, '""')}"`).join(','))
    .join('\n');
  exportCsv(csv, 'validations.csv');
}

function filter(value: any, search: string | null, item: any) {
  if (!search) return true;
  const term = search.toLowerCase().trim();
  return (
    item.speciesName?.toLowerCase().includes(term) ||
    item.cloneName?.toLowerCase().includes(term) ||
    item.lotNumber?.toLowerCase().includes(term) ||
    String(item.tubeNumber ?? '').toLowerCase().includes(term) ||
    item.userName?.toLowerCase().includes(term) ||
    item.proteinName?.toLowerCase().includes(term)
  );
}

async function updateValidationArchiveState(id: number, state: boolean) {
  if (confirm(`Are you sure you want to ${state ? 'archive' : 'unarchive'} the validation?`)) {
    await validationStore.updateValidationArchiveState({ id, data: { state } });
  }
}

function removeFromFilter<T>(filter: Ref<T[]>, item: T) {
  const index = filter.value.indexOf(item);
  if (index !== -1) {
    filter.value.splice(index, 1);
    filter.value = [...filter.value];
  }
}

import { useValidations } from '@/composables/useValidations'
const globalSearch = computed({
  get: () => filterStore.filters.validationGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('validationGlobalSearch', value),
})
const { items, loading, reload } = useValidations({ sortBy, globalFilter: globalSearch })

</script>
