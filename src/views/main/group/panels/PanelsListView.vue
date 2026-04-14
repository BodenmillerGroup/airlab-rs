<template>
  <v-col>
    <v-toolbar density="compact" class="toolbar">
      <v-toolbar-title>Panels</v-toolbar-title>
      <v-spacer />
      <v-btn variant="text" :to="`/main/groups/${activeGroupId}/panels/create`" color="primary">
        Create Panel
      </v-btn>
    </v-toolbar>

    <v-expansion-panels>
      <v-expansion-panel>
        <v-expansion-panel-title>
          <FilterSummary />
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <FilterField key-name="panelName" class="mb-1" />
          <FilterField key-name="panelApplication" class="mb-1" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-switch
      v-model="showAllPanels"
      label="Show all panels"
      color="primary"
      density="compact"
      hide-details
      class="mt-3 mb-2"
    />

    <v-text-field
      v-model="globalSearch"
      label="Search all panel fields"
      variant="solo"
      density="comfortable"
      clearable
      prepend-inner-icon="mdi-magnify"
      class="mb-3"
    />

    <v-card>
      <v-data-table-server
        :headers="headers"
        :items="items"
        :loading="panelStore.loading"
        :items-length="panelStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        show-expand
        v-model:expanded="expanded"
        v-model:sort-by="sortBy"
        item-value="id"
      >
        <template #item.name="{ item }">
          <router-link class="link" :to="{ name: 'main-group-panels-view', params: { id: item.id } }">
            {{ item.name }}
          </router-link>
        </template>

        <template #item.user="{ item }">
          <router-link
            v-if="item.userId"
            class="link"
            :to="{ name: 'main-admin-users-edit', params: { id: item.userId } }"
          >
            {{ item.userName }}
          </router-link>
        </template>

        <template #item.application="{ item }">
          {{ applicationToString(item.application) }}
        </template>

        <template #item.updatedAt="{ item }">
          {{ new Date(item.updatedAt).toUTCString() }}
        </template>

        <template #item.isFluorophore="{ item }">
          <v-icon v-if="item.isFluorophore">mdi-check</v-icon>
        </template>

        <template #item.isLocked="{ item }">
          <v-icon v-if="item.isLocked">mdi-lock-outline</v-icon>
        </template>


        <template #item.action="{ item }">
          <v-menu location="bottom">
            <template #activator="{ props }">
              <v-btn icon v-bind="props">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>

            <v-list density="compact">
              <v-list-item :to="{ name: 'main-group-panels-edit', params: { groupId: activeGroupId, id: item.id } }">
                <v-icon color="grey" start>mdi-pencil-outline</v-icon>
                <v-list-item-title>Edit</v-list-item-title>
              </v-list-item>

              <v-list-item :to="{ name: 'main-group-panels-view', params: { groupId: activeGroupId, id: item.id } }">
                <v-icon color="grey" start>mdi-eyedropper</v-icon>
                <v-list-item-title>Refine</v-list-item-title>
              </v-list-item>

              <v-list-item @click="duplicatePanel(item.id)">
                <v-icon color="grey" start>mdi-content-duplicate</v-icon>
                <v-list-item-title>Duplicate</v-list-item-title>
              </v-list-item>

              <v-list-item v-if="isGroupAdmin" @click="updatePanelArchiveState(item.id, !item.isArchived)">
                <v-icon color="red" start>
                  {{ item.isArchived ? 'mdi-archive-arrow-up-outline' : 'mdi-archive-arrow-down-outline' }}
                </v-icon>
                <v-list-item-title>
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
        </template>

        <template #expanded-row="{ item, columns }">
          <td :colspan="columns.length">
            <PanelExpandedView :panel="item" />
          </td>
        </template>
      </v-data-table-server>
    </v-card>

    <v-navigation-drawer v-model="drawer" location="right" temporary width="400">
      <PanelDetailsView v-if="drawer" :panel="detailsItem" />
    </v-navigation-drawer>
  </v-col>
</template>



<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import { useRoute } from 'vue-router';
import { useGroupStore } from '@/stores/group';
import { usePanelStore } from '@/stores/panel';
import PanelExpandedView from '@/views/main/group/panels/PanelExpandedView.vue';
import PanelDetailsView from '@/views/main/group/panels/PanelDetailsView.vue';
import { applicationEnum } from '@/utils/enums';
import { applicationToString } from '@/utils/converters';
import { useFilterStore } from '@/stores/useFilterStore'
import FilterField from '@/components/FilterField.vue'
import FilterSummary from '@/components/FilterSummary.vue'
import { useServerTablePagination } from '@/composables/useServerTablePagination'

const expanded = ref<any[]>([])
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'id', order: 'desc' },
]);

const route = useRoute();
const groupStore = useGroupStore();
const panelStore = usePanelStore();
const { page, limit } = storeToRefs(panelStore);
const filterStore = useFilterStore()
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit)

const drawer = ref(false);
const detailsItem = ref<any>(null);
const search = ref('');
const applicationFilter = ref<number[]>([]);
const showAllPanels = ref(false);

const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const myMember = computed(() => groupStore.myMember);

const applications = applicationEnum;

const rawHeaders = [
  { title: 'Id', key: 'id', sortable: true },
  { title: 'Name', key: 'name', sortable: true },
  { title: 'Description', key: 'description', sortable: true },
  { title: 'Fluorophore', key: 'isFluorophore', sortable: true, filterable: false },
  { title: 'Locked', key: 'isLocked', sortable: true, filterable: false },
  { title: 'Application', key: 'application', sortable: true, filterable: false },
  { title: 'Created by', key: 'user' },
  { title: 'Updated at', key: 'updatedAt', sortable: true },
  { title: 'Actions', key: 'action', sortable: false, filterable: false, width: 150 },
  { title: '', key: 'data-table-expand' },
];


const headers = rawHeaders.map(h => ({
  sortable: h.key !== 'action' && h.key !== 'data-table-expand',
  ...h,
}))

const filteredPanels = computed(() => {
  let list = items.value;
  if (applicationFilter.value.length > 0) {
    list = list.filter(p => applicationFilter.value.includes(p.application));
  }
  return list;
});

function filter(_value: any, search: string, item: any) {
  const s = search?.toLowerCase() || '';
  return (
    item.name?.toLowerCase().includes(s) ||
    item.description?.toLowerCase().includes(s) ||
    item.user?.name?.toLowerCase().includes(s)
  );
}

function showDetails(item: any) {
  detailsItem.value = item;
  drawer.value = true;
}

function removeApplicationFilter(item: any) {
  applicationFilter.value = applicationFilter.value.filter(id => id !== item.value);
}

async function duplicatePanel(id: number) {
  const name = prompt('New Panel Name', 'My Panel');
  if (name && activeGroupId.value) {
    await panelStore.duplicatePanel({ id, data: { name, groupId: activeGroupId.value } });
  }
}

async function updatePanelArchiveState(id: number, state: boolean) {
  const action = state ? 'archive' : 'unarchive';
  if (confirm(`Are you sure you want to ${action} the panel?`)) {
    await panelStore.updatePanelArchiveState({ id, data: { state } });
  }
}

import { usePanels } from '@/composables/usePanels'
const globalSearch = computed({
  get: () => filterStore.filters.panelGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('panelGlobalSearch', value),
})
const { items, loading, reload } = usePanels({ sortBy, showAllPanels, globalFilter: globalSearch })

</script>
