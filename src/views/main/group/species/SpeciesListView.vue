<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Species</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn
          text
          :to="`/main/groups/${activeGroupId}/species/create`"
          color="primary"
        >
          Create Species
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-expansion-panels>
      <v-expansion-panel>
        <v-expansion-panel-title>
          <FilterSummary />
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <FilterField key-name="speciesName" class="mb-1" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-text-field
      v-model="globalSearch"
      label="Search all species fields"
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
        :loading="speciesStore.loading"
        :items-length="speciesStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        show-expand
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
                  name: 'main-group-species-edit',
                  params: {
                    groupId: activeGroupId,
                    id: item.id
                  }
                }"
              >
                  <v-icon color="grey">mdi-pencil-outline</v-icon>
                  <v-list-item-title>Edit</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>

          <v-tooltip bottom>
            <template v-slot:activator="{ props }">
              <v-btn icon v-bind="props" @click.stop="showDetails(item)">
                <v-icon>mdi-information-outline</v-icon>
              </v-btn>
            </template>
            <span>Show details</span>
          </v-tooltip>
        </template>

        <template v-slot:expanded-row="{ columns, item }">
          <td :colspan="columns.length">
            <SpeciesExpandedView :species="item" />
          </td>
        </template>
      </v-data-table-server>
    </v-card>

    <v-navigation-drawer v-model="drawer" right fixed temporary width="400">
      <SpeciesDetailsView v-if="drawer" :species="detailsItem" />
    </v-navigation-drawer>
  </v-col>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { storeToRefs } from 'pinia';

import { useGroupStore } from '@/stores/group';
import { useSpeciesStore } from '@/stores/species';
import { useFilterStore } from '@/stores/useFilterStore'

import SpeciesExpandedView from '@/views/main/group/species/SpeciesExpandedView.vue';
import SpeciesDetailsView from '@/views/main/group/species/SpeciesDetailsView.vue';
import FilterField from '@/components/FilterField.vue'
import FilterSummary from '@/components/FilterSummary.vue'
import type { SpeciesDto } from '@/modules/species/types';
import { useServerTablePagination } from '@/composables/useServerTablePagination';

// Stores
const groupStore = useGroupStore();
const speciesStore = useSpeciesStore();
const filterStore = useFilterStore()
const { activeGroupId, isGroupAdmin } = storeToRefs(groupStore);
const { page, limit } = storeToRefs(speciesStore);

import { useSpecies } from '@/composables/useSpecies'
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'name', order: 'asc' },
])
const globalSearch = computed({
  get: () => filterStore.filters.speciesGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('speciesGlobalSearch', value),
})
const { items, loading, reload } = useSpecies({ sortBy, globalFilter: globalSearch })
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit);

// State
const drawer = ref(false);
const detailsItem = ref<SpeciesDto | null>(null);

const rawHeaders = [
  { title: 'Name', sortable: true, key: 'name', align: 'start' },
  { title: 'Acronym', sortable: true, key: 'acronym', align: 'start' },
  { title: 'Actions', key: 'action', sortable: false, filterable: false, width: '130' },
  { title: '', key: 'data-table-expand' }
] as const


const headers = rawHeaders.map(h => ({
  sortable: h.key !== 'action' && h.key !== 'data-table-expand',
  ...h,
}))

// Logic
function showDetails(item: SpeciesDto) {
  detailsItem.value = item;
  drawer.value = true;
}

const route = useRoute();

function updateSearch(val: string) {
  speciesStore.updateSearch(val);
}

</script>

<style scoped>
.toolbar {
  margin-bottom: 10px;
}
</style>
