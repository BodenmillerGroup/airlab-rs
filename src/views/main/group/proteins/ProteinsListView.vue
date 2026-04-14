<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Proteins</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
         <v-btn text :to="`/main/groups/${activeGroupId}/proteins/create`" color="primary">Create Protein</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-expansion-panels>
      <v-expansion-panel>
        <v-expansion-panel-title>
          <FilterSummary />
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <FilterField key-name="proteinName" class="mb-1" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-text-field
      v-model="globalSearch"
      label="Search all protein fields"
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
        :loading="proteinStore.loading"
        :items-length="proteinStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        show-expand
        v-model:sort-by="sortBy"
        item-value="id"
      >

        <template v-slot:item.action="{ item }">
          <v-menu location="bottom left">
            <template v-slot:activator="{ props }">
              <v-btn icon v-bind="props">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
        
            <v-list density="compact">
              <v-list-item
                :to="{
                  name: 'main-group-clones-create',
                  params: { groupId: activeGroupId, proteinId: item.id },
                }"
              >
                <v-icon color="primary">mdi-plus-circle-outline</v-icon>
                <v-list-item-title>Add Clone</v-list-item-title>
              </v-list-item>
        
              <v-divider />
        
              <v-list-item
                :to="{
                  name: 'main-group-proteins-edit',
                  params: { groupId: activeGroupId, id: item.id },
                }"
              >
                <v-icon color="grey">mdi-pencil-outline</v-icon>
                <v-list-item-title>Edit</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        
          <v-tooltip location="bottom">
            <template v-slot:activator="{ props }">
              <v-btn icon v-bind="props" @click.stop="showDetails(item)">
                <v-icon>mdi-information-outline</v-icon>
              </v-btn>
            </template>
            <span>Show details</span>
          </v-tooltip>
        </template>


        <template #expanded-row="{ item, columns }">
          <td :colspan="columns?.length || 5">
            <ProteinExpandedView :protein="item" />
          </td>
        </template>
      </v-data-table-server>
    </v-card>

    <v-navigation-drawer v-model="drawer" right fixed temporary width="400">
      <ProteinDetailsView v-if="drawer" :protein="detailsItem" />
    </v-navigation-drawer>
  </v-col>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import ProteinDetailsView from '@/views/main/group/proteins/ProteinDetailsView.vue'
import ProteinExpandedView from '@/views/main/group/proteins/ProteinExpandedView.vue'
import FilterField from '@/components/FilterField.vue'
import FilterSummary from '@/components/FilterSummary.vue'
import { useProteinStore } from '@/stores/protein'
import { useGroupStore } from '@/stores/group'
import { useFilterStore } from '@/stores/useFilterStore'
import type { ProteinDto } from '@/modules/protein/types'
import { storeToRefs } from 'pinia'
import { useServerTablePagination } from '@/composables/useServerTablePagination'

const proteinStore = useProteinStore()
const groupStore = useGroupStore()
const filterStore = useFilterStore()
const { searchstr } = storeToRefs(proteinStore)
const { page, limit } = storeToRefs(proteinStore)
const route = useRoute()

const activeGroupId = computed(() => groupStore.activeGroupId);

const drawer = ref(false)
const detailsItem = ref<ProteinDto | null>(null)
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'id', order: 'desc' },
])

import { useProteins } from '@/composables/useProteins'
const globalSearch = computed({
  get: () => filterStore.filters.proteinGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('proteinGlobalSearch', value),
})
const { items, loading, reload } = useProteins({ sortBy, globalFilter: globalSearch })
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit)

const rawHeaders = [
  { title: 'Id', key: 'id', sortable: true },
  { title: 'Name', key: 'name', sortable: true },
  { title: 'Description', key: 'description', sortable: true },
  { title: 'Actions', key: 'action', sortable: false, width: "130", },
  { title: '', key: 'data-table-expand' },
]


const headers = rawHeaders.map(h => ({
  sortable: h.key !== 'action' && h.key !== 'data-table-expand',
  ...h,
}))

const onSearchChange = (val: string) => {
  proteinStore.updateSearch(val)
}

function showDetails(item: ProteinDto) {
  detailsItem.value = item
  drawer.value = true
}

</script>

<style scoped>
.toolbar {
  margin-bottom: 10px;
}
</style>
