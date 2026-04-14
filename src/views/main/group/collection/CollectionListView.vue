<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Collections</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn
          text
          :to="`/main/groups/${activeGroupId}/collections/create`"
          color="primary"
        >
          Add Collection
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-text-field
      v-model="globalSearch"
      label="Search all collection fields"
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
        :loading="collectionStore.loading"
        :items-length="collectionStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        v-model:sort-by="sortBy"
        item-value="id"
      >
        <template #item.description="{ item }">
          {{ item.description || '-' }}
        </template>

        <template #item.created_at="{ item }">
          {{ formatDate(item.created_at) }}
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
                  name: 'main-group-collections-edit',
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
import { useCollectionStore } from '@/stores/collection';
import { useFilterStore } from '@/stores/useFilterStore';
import { useServerTablePagination } from '@/composables/useServerTablePagination';

const groupStore = useGroupStore();
const collectionStore = useCollectionStore();
const filterStore = useFilterStore();

const { activeGroupId } = storeToRefs(groupStore);
const { page, limit } = storeToRefs(collectionStore);
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'id', order: 'desc' },
]);
const globalSearch = computed({
  get: () => filterStore.filters.collectionGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('collectionGlobalSearch', value),
});

import { useCollections } from '@/composables/useCollections';
const { items } = useCollections({ sortBy, globalFilter: globalSearch });
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit);

const rawHeaders = [
  { title: 'Name', key: 'name', sortable: true },
  { title: 'Description', key: 'description', sortable: true },
  { title: 'Created At', key: 'created_at', sortable: true },
  { title: 'Created By', key: 'createdByName', sortable: false },
  { title: 'Actions', key: 'action', sortable: false, width: '100' },
] as const;

const headers = rawHeaders.map((header) => ({
  sortable: header.key !== 'action',
  ...header,
}));

function formatDate(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;
  return date.toLocaleString();
}

</script>

<style scoped>
.toolbar {
  margin-bottom: 10px;
}
</style>
