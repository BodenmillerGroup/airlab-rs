<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Providers</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn text :to="`/main/groups/${activeGroupId}/providers/create`" color="primary">
          Create Provider
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-expansion-panels>
      <v-expansion-panel>
        <v-expansion-panel-title>
          <FilterSummary />
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <FilterField key-name="providerName" class="mb-1" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-text-field
      v-model="globalSearch"
      label="Search all provider fields"
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
        :loading="providerStore.loading"
        :items-length="providerStore.total"
        v-model:items-per-page="providerItemsPerPage"
        v-model:page="providerPage"
        :footer-props="tableFooterProps"
        show-expand
        v-model:expanded="expanded"
        v-model:sort-by="sortBy"
        item-value="id"
      >
        <template v-slot:item.action="{ item }">
          <v-menu bottom left>
            <template v-slot:activator="{ props }">
              <v-btn icon v-bind="props">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>

            <v-list dense>
              <v-list-item
                :to="{
                  name: 'main-group-providers-edit',
                  params: { groupId: activeGroupId, id: item.id },
                }"
              >
                  <v-icon color="grey">mdi-pencil-outline</v-icon>
                  <v-list-item-title>Edit</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>

          <v-tooltip bottom>
            <template v-slot:activator="{ props }">
              <v-btn v-bind="props" icon @click.stop="showDetails(item)">
                <v-icon>mdi-information-outline</v-icon>
              </v-btn>
            </template>
            <span>Show details</span>
          </v-tooltip>
        </template>

        <template v-slot:expanded-row="{ columns, item }">
          <td :colspan="columns.length">
            <ProviderExpandedView :provider="item" />
          </td>
        </template>
      </v-data-table-server>
    </v-card>

    <v-navigation-drawer v-model="drawer" right fixed temporary width="500">
      <ProviderDetailsView v-if="drawer" :provider="detailsItem" />
    </v-navigation-drawer>
  </v-col>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useGroupStore } from '@/stores/group';
import { useProviderStore } from '@/stores/provider';
import { useFilterStore } from '@/stores/useFilterStore'
import { storeToRefs } from 'pinia'
import ProviderDetailsView from '@/views/main/group/providers/ProviderDetailsView.vue';
import ProviderExpandedView from '@/views/main/group/providers/ProviderExpandedView.vue';
import FilterField from '@/components/FilterField.vue'
import FilterSummary from '@/components/FilterSummary.vue'
import type { ProviderDto } from '@/modules/provider/types';
import { useServerTablePagination } from '@/composables/useServerTablePagination'

// Stores
const groupStore = useGroupStore();
const providerStore = useProviderStore();
const filterStore = useFilterStore()
const { page, limit } = storeToRefs(providerStore)

// State
const drawer = ref(false);
const search = ref('');
const detailsItem = ref<ProviderDto | null>(null);
const expanded = ref<any[]>([]);
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'id', order: 'desc' },
]);

// Derived
const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const {
  tablePage: providerPage,
  tableItemsPerPage: providerItemsPerPage,
  tableFooterProps,
} = useServerTablePagination(page, limit)

const globalSearch = computed({
  get: () => filterStore.filters.providerGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('providerGlobalSearch', value),
})

import { useProviders } from '@/composables/useProviders'
const { items, loading, reload } = useProviders({ sortBy, globalFilter: globalSearch })

// Headers
const rawHeaders = [
  { title: 'Id', key: 'id', sortable: true },
  { title: 'Name', key: 'name', sortable: true },
  { title: 'Description', key: 'description', sortable: true },
  { title: 'URL', key: 'url', sortable: true, filterable: false },
  {
    title: 'Actions',
    key: 'action',
    sortable: false,
    filterable: false,
    width: '130',
  },
  { title: '', key: 'data-table-expand' },
];


const headers = rawHeaders.map(h => ({
  sortable: h.key !== 'action' && h.key !== 'data-table-expand',
  ...h,
}))

// Methods
function showDetails(item: ProviderDto) {
  detailsItem.value = item;
  drawer.value = true;
}

async function deleteProvider(id: number) {
  if (self.confirm('Are you sure you want to delete the provider?')) {
    if (self.confirm('All children entities will be deleted!')) {
      await providerStore.deleteProvider(id);
    }
  }
}

</script>
