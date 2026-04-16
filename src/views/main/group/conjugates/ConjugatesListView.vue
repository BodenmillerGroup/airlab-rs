<template>
  <v-col>
    <v-toolbar density="compact" class="toolbar">
      <v-toolbar-title>Conjugates</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn
          text
          :to="`/main/groups/${activeGroupId}/conjugates/create`"
          color="primary"
        >
          Create Conjugate
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>
    <v-expansion-panels>
      <v-expansion-panel>
        <v-expansion-panel-title>
          <FilterSummary />
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <FilterField key-name="tubeNumber" class="mb-1" />
          <FilterField key-name="proteinName" class="mb-1" />
          <FilterField key-name="cloneName" class="mb-1" />
          <FilterField key-name="mass" class="mb-1" />
          <FilterField key-name="lotCollectionName" class="mb-1" />
          <FilterField key-name="conjugateStatus" class="mb-1" />
          <FilterField key-name="validationApplication" class="mb-1" />
          <FilterField key-name="validationStatus" class="mb-1" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-text-field
      v-model="globalSearch"
      label="Search all conjugate fields"
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
        :loading="conjugateStore.loading"
        :items-length="conjugateStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        show-expand
        v-model:expanded="expanded"
        v-model:sort-by="sortBy"
        item-value="id"
      >

        <template #item.protein="{ item }">
          <router-link
            class="link"
            :to="{
              name: 'main-group-proteins-edit',
              params: { groupId: item.groupId, id: item.proteinId }
            }"
          >
            {{ item.proteinName }}
          </router-link>
        </template>

        <template #item.clone ="{ item }">
          <router-link
            class="link"
            :to="{
              name: 'main-group-clones-edit',
              params: { groupId: item.groupId, id: item.cloneId }
            }"
          >
            {{ item.cloneName }}
          </router-link>
        </template>

        <template #item.lot ="{ item }">
          <router-link
            class="link"
            :to="{
              name: 'main-group-lots-edit',
              params: { groupId: item.groupId, id: item.lotId }
            }"
          >
            {{ item.lotName }}
          </router-link>
        </template>

        <template #item.collection ="{ item }">
          <router-link
            v-if="item.lotCollectionId"
            class="link"
            :to="{
              name: 'main-group-collections-edit',
              params: { groupId: item.groupId, id: item.lotCollectionId }
            }"
          >
            {{ item.lotCollectionName }}
          </router-link>
          <span v-else>—</span>
        </template>

        <template #item.tag ="{ item }">
          <router-link
            class="link"
            :to="{
              name: 'main-group-tags-edit',
              params: { groupId: item.groupId, id: item.tagId }
            }"
          >
            {{ item.tagName }}
          </router-link>
        </template>

        <template #item.storage="{ item }">
          {{ item.storageId ? (storageStore.getStorage(item.storageId)?.name ?? '—') : '—' }}
        </template>

        <template #item.user="{ item }">
          <router-link
            v-if="item.userId"
            class="link"
            :to="{
              name: 'main-admin-users-edit',
              params: { id: item.userId }
            }"
          >
            {{ item.userName }}
          </router-link>
        </template>

        <template #item.status="{ item }">
          <v-chip
            :color="getConjugateStatusColorWrapper(item.status)"
            class="mr-1"
            size="small"
            label
          >
            {{ conjugateStatusToString(item.status) }}
          </v-chip>
        </template>

        <template #item.validations="{ item }">
          <v-chip
            v-for="validation in item.validations"
            :key="validation.id"
            :color="getStatusColor(validation)"
            class="mr-1"
            size="x-small"
            label
            @click.stop="showValidation(validation.id)"
          >
            {{ applicationToString(validation.application) }}
          </v-chip>
        </template>

        <template #item.action="{ item }">
          <div class="action-buttons">
            <v-menu location="bottom">
              <template #activator="{ props }">
                <v-btn icon v-bind="props">
                  <v-icon>mdi-dots-vertical</v-icon>
                </v-btn>
              </template>
              <v-list density="compact">
                <v-list-item
                  v-if="item.status !== 0"
                  @click="updateConjugateStatus(item.id, 0)"
                >
                  <v-list-item-title>
                    <v-icon start color="green">mdi-flask-empty</v-icon>
                    Mark as Stock
                  </v-list-item-title>
                </v-list-item>

                <v-list-item
                  v-if="item.status !== 1"
                  @click="updateConjugateStatus(item.id, 1)"
                >
                  <v-list-item-title>
                    <v-icon start color="orange">mdi-speedometer-slow</v-icon>
                    Mark as Low
                  </v-list-item-title>
                </v-list-item>

                <v-list-item
                  v-if="item.status !== 2"
                  @click="updateConjugateStatus(item.id, 2)"
                >
                  <v-list-item-title>
                    <v-icon start color="red accent-1">mdi-flask-empty-remove-outline</v-icon>
                    Mark as Finished
                  </v-list-item-title>
                </v-list-item>

                <v-divider />

                <v-list-item
                  :to="{
                    name: 'main-group-validations-create',
                    params: { groupId: activeGroupId },
                    query: {
                      cloneId: item.cloneId,
                      lotId: item.lotId,
                      conjugateId: item.id
                    }
                  }"
                >
                  <v-list-item-title>
                    <v-icon start color="primary">mdi-plus-circle-outline</v-icon>
                    Add Validation
                  </v-list-item-title>
                </v-list-item>

                <v-divider />

                <v-list-item
                  :to="{
                    name: 'main-group-conjugates-edit',
                    params: { groupId: activeGroupId, id: item.id }
                  }"
                >
                  <v-list-item-title>
                    <v-icon start color="grey">mdi-pencil-outline</v-icon>
                    Edit
                  </v-list-item-title>
                </v-list-item>

                <v-list-item
                  v-if="isGroupAdmin"
                  @click="updateConjugateArchiveState(item.id, !item.isArchived)"
                >
                  <v-list-item-title>
                    <v-icon start color="red accent-1">
                      {{ item.isArchived ? 'mdi-archive-arrow-up-outline' : 'mdi-archive-arrow-down-outline' }}
                    </v-icon>
                    {{ item.isArchived ? 'Unarchive' : 'Archive' }}
                  </v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>

            <v-tooltip location="bottom">
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
          <td :colspan="columns.length">
            <ConjugateExpandedView :conjugate="item" />
          </td>
        </template>
      </v-data-table-server>
    </v-card>

    <v-navigation-drawer v-model="drawer" location="right" temporary width="600">
      <ConjugateDetailsView v-if="drawer" :conjugate="detailsItem" />
    </v-navigation-drawer>

    <v-navigation-drawer v-model="validationDrawer" location="right" temporary width="600">
      <ValidationDetailsView v-if="validationDrawer" :validation-id="selectedValidationId" />
    </v-navigation-drawer>
  </v-col>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import { useRoute } from 'vue-router';
import { useGroupStore } from '@/stores/group';
import { useConjugateStore } from '@/stores/conjugate';
import { useCloneStore } from '@/stores/clone';
import { useProteinStore } from '@/stores/protein';
import { useLotStore } from '@/stores/lot';
import { useTagStore } from '@/stores/tag';
import { useStorageStore } from '@/stores/storage';
import { useFilterStore } from '@/stores/useFilterStore';
import { applicationToString, getConjugateStatusColor, getStatusColor } from '@/utils/converters';
import { applicationEnum } from '@/utils/enums';
import type { ConjugateDto } from '@/modules/conjugate/types';
import type { ConjugateStatus } from '@/modules/conjugate/ConjugateStatus';
import FilterField from '@/components/FilterField.vue'
import ConjugateExpandedView from "@/views/main/group/conjugates/ConjugateExpandedView.vue";
import ConjugateDetailsView from "@/views/main/group/conjugates/ConjugateDetailsView.vue";
import ValidationDetailsView from "@/views/main/group/validations/ValidationDetailsView.vue";
import FilterSummary from '@/components/FilterSummary.vue'
import { useServerTablePagination } from '@/composables/useServerTablePagination'
import { conjugateStatusToString } from '@/utils/converters'

const expanded = ref<any[]>([])
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'id', order: 'desc' },
]);

const groupStore = useGroupStore();
const conjugateStore = useConjugateStore();
const { page, limit } = storeToRefs(conjugateStore);
const cloneStore = useCloneStore();
const proteinStore = useProteinStore();
const lotStore = useLotStore();
const tagStore = useTagStore();
const storageStore = useStorageStore();
const filterStore = useFilterStore();
const route = useRoute();
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit)

const drawer = ref(false);
const validationDrawer = ref(false);
const selectedValidationId = ref<number | null>(null);
const detailsItem = ref<any | null>(null);

const search = ref('');
const tagFilter = ref<number[]>([]);
const statusFilter = ref<number[]>([]);
const validationApplicationFilter = ref<number[]>([]);
const validationStatusFilter = ref<number[]>([]);

const statuses = [
  { id: 0, name: 'Stock' },
  { id: 1, name: 'Low' },
  { id: 2, name: 'Finished' },
];

const validationStatuses = [
  { key: 0, title: "Yes" },
  { key: 1, title: "So-So" },
  { key: 2, title: "No" },
  { key: 3, title: "Undefined" },
  { key: -1, title: "No validations" },
];

const applications = applicationEnum;

const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const conjugates = computed(() => conjugateStore.conjugates);
const globalSearch = computed({
  get: () => filterStore.filters.conjugateGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('conjugateGlobalSearch', value),
});

function removeTagFilter(tag) {
  tagFilter.value = tagFilter.value.filter(id => id !== tag.id);
}
function removeStatusFilter(status) {
  statusFilter.value = statusFilter.value.filter(id => id !== status.id);
}
function removeValidationApplicationFilter(app) {
  validationApplicationFilter.value = validationApplicationFilter.value.filter(id => id !== app.value);
}
function removeValidationStatusFilter(status) {
  validationStatusFilter.value = validationStatusFilter.value.filter(id => id !== status.value);
}

  const rawHeaders = [
    {
      title: "Tube Number",
      key: "tubeNumber",
      align: "end",
      sortable: true,
    },
    {
      title: "Protein",
      key: "protein",
      sort: (a, b) => {
        if (a == null) return 1
        if (b == null) return -1
        return a.name.localeCompare(b.name)
      },
    },
    {
      title: "Clone",
      key: "clone",
      sort: (a, b) => {
        if (a === null) {
          return 1;
        }
        if (b === null) {
          return -1;
        }
        return a.name.localeCompare(b.name);
      },
    },
    {
      title: "Lot",
      key: "lot",
      sort: (a, b) => {
        if (a === null) {
          return 1;
        }
        if (b === null) {
          return -1;
        }
        return a.number.localeCompare(b.number);
      },
    },
    {
      title: "Collection",
      key: "collection",
    },
    {
      title: "Tag",
      key: "tag",
    },
    {
      title: "Storage",
      key: "storage",
      filterable: false,
    },
    {
      title: "Mass",
      key: "tagMw",
      align: "end",
      filterable: false,
      width: 100,
    },
    {
      title: "Labeled by",
      key: "user",
      sort: (a, b) => {
        if (a === null) {
          return 1;
        }
        if (b === null) {
          return -1;
        }
        return a.name.localeCompare(b.name);
      },
    },
    {
      title: "Stock Concentration",
      key: "concentration",
      align: "end",
      sortable: true,
      filterable: false,
    },
    {
      title: "Status",
      key: "status",
      sortable: true,
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
      width: "150",
    },
    {
      title: "",
      key: "data-table-expand",
    },
  ] as const


const headers = rawHeaders.map(h => ({
  sortable: h.key !== 'action' && h.key !== 'data-table-expand',
  ...h,
}))

import { useConjugates } from '@/composables/useConjugates'
const { items, loading, reload } = useConjugates({ sortBy, globalFilter: globalSearch })

function getConjugateStatusColorWrapper(status: number) {
  return getConjugateStatusColor({ status } as ConjugateDto);
}

function showValidation(id: number) {
  selectedValidationId.value = id;
  validationDrawer.value = true;
}

function showDetails(item: any) {
  detailsItem.value = item;
  drawer.value = true;
}

async function updateConjugateStatus(id: number, status: ConjugateStatus) {
  await conjugateStore.updateConjugateStatus({ id, data: { status } });
  await reload();
}

async function updateConjugateArchiveState(id: number, state: boolean) {
  await conjugateStore.updateConjugateArchiveState({ id, data: { state } });
  await reload();
}

onMounted(async () => {
  await storageStore.getStorages();
});

</script>

<style scoped>
.action-buttons {
  display: inline-flex;
  flex-wrap: nowrap;
  align-items: center;
  gap: 4px;
  min-width: 72px;
}
</style>
