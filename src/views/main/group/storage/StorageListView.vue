<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Storage</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn
          text
          :to="`/main/groups/${activeGroupId}/storage/create`"
          color="primary"
        >
          Add Storage
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-text-field
      v-model="globalSearch"
      label="Search all storage fields"
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
        :loading="storageStore.loading"
        :items-length="storageStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        v-model:sort-by="sortBy"
        item-value="id"
      >
        <template #item.temperature_c="{ item }">
          {{ item.temperature_c }} C
        </template>

        <template #item.active="{ item }">
          <v-chip :color="item.active ? 'success' : 'grey'" size="small" label>
            {{ item.active ? 'Active' : 'Inactive' }}
          </v-chip>
        </template>

        <template #item.action="{ item }">
          <v-menu bottom left>
            <template #activator="{ props }">
              <v-btn icon v-bind="props">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
            <v-list dense>
              <v-list-item
                :to="{
                  name: 'main-group-storage-edit',
                  params: { groupId: activeGroupId, id: item.id },
                }"
              >
                <v-icon color="grey">mdi-pencil-outline</v-icon>
                <v-list-item-title>Edit</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </template>
      </v-data-table-server>
    </v-card>
  </v-col>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { storeToRefs } from 'pinia';

import { useGroupStore } from '@/stores/group';
import { useStorageStore } from '@/stores/storage';
import { useFilterStore } from '@/stores/useFilterStore';
import { useServerTablePagination } from '@/composables/useServerTablePagination';

const groupStore = useGroupStore();
const storageStore = useStorageStore();
const filterStore = useFilterStore();

const { activeGroupId } = storeToRefs(groupStore);
const { page, limit } = storeToRefs(storageStore);
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'id', order: 'desc' },
]);
const globalSearch = computed({
  get: () => filterStore.filters.storageGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('storageGlobalSearch', value),
});

import { useStorages } from '@/composables/useStorages';
const { items } = useStorages({ sortBy, globalFilter: globalSearch });
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit);

const rawHeaders = [
  { title: 'Name', key: 'name', sortable: true },
  { title: 'Type', key: 'type', sortable: true },
  { title: 'Location', key: 'location', sortable: true },
  { title: 'Temp', key: 'temperature_c', sortable: true },
  { title: 'Status', key: 'active', sortable: true },
  { title: 'Actions', key: 'action', sortable: false, width: '100' },
] as const;

const headers = rawHeaders.map((header) => ({
  sortable: header.key !== 'action',
  ...header,
}));

</script>

<style scoped>
.toolbar {
  margin-bottom: 10px;
}
</style>
