<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Group Members</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn v-if="isGroupAdmin" text :to="`/main/groups/${activeGroupId}/members/create`" color="primary">
          Create Member
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-text-field
      v-model="globalSearch"
      label="Search all member fields"
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
        :loading="memberStore.loading"
        :items-length="memberStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        v-model:sort-by="sortBy"
        item-value="id"
      >

        <template v-slot:item.user="{ item }">
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

        <template v-slot:item.role="{ item }">
          {{ roleToString(item.role) }}
        </template>

        <template v-slot:item.isActive="{ item }">
          <v-icon v-if="item.isActive">mdi-check</v-icon>
        </template>

        <template v-slot:item.allPanels="{ item }">
          <v-icon v-if="item.allPanels">mdi-check</v-icon>
        </template>

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
                  name: 'main-group-members-edit',
                  params: {
                    groupId: activeGroupId,
                    id: item.id,
                  },
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

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useGroupStore } from '@/stores/group';
import { useMemberStore } from '@/stores/member';
import { useFilterStore } from '@/stores/useFilterStore'
import { roleToString } from '@/utils/converters';
import { roleEnum } from '@/utils/enums';
import { storeToRefs } from 'pinia';
import { useServerTablePagination } from '@/composables/useServerTablePagination';

const groupStore = useGroupStore();
const memberStore = useMemberStore();
const filterStore = useFilterStore()
const { page, limit } = storeToRefs(memberStore);
const route = useRoute();

const search = ref('');
const roleFilter = ref<number[]>([]);
const showInactiveMembers = ref(false);
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'role', order: 'desc' },
]);

const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const roles = roleEnum;

const rawHeaders = [
  {
    title: 'Name',
    key: 'userName',
    sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? ''),
  },
  { title: 'Role', key: 'role', sortable: true },
  {
    title: 'Active',
    key: 'isActive',
    align: 'start',
    sortable: true,
    filterable: false,
    width: 100,
  },
  {
    title: 'All panels',
    key: 'allPanels',
    align: 'start',
    sortable: true,
    filterable: false,
    width: 130,
  },
  {
    title: 'Actions',
    key: 'action',
    sortable: false,
    filterable: false,
    width: 70,
  },
] as const


const headers = rawHeaders.map(h => ({
  sortable: h.key !== 'action',
  ...h,
}))

import { useMembers } from '@/composables/useMembers'
const globalSearch = computed({
  get: () => filterStore.filters.memberGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('memberGlobalSearch', value),
})
const { items, loading, reload } = useMembers({ sortBy, globalFilter: globalSearch })
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit);

function filter(value, search, item) {
  if (!search) return true;
  const normalizedSearch = search.toLowerCase().trim();
  return item.user?.name?.toLowerCase().includes(normalizedSearch);
}

function removeRoleFilter(item) {
  roleFilter.value = roleFilter.value.filter((role) => role !== item.value);
}

async function deleteMember(id: number) {
  if (
    confirm('Are you sure you want to delete the group member?') &&
    confirm('All children entities will be deleted!')
  ) {
    await memberStore.removeMember(id);
  }
}

</script>

<style scoped>
.toolbar {
  margin-bottom: 10px;
}
</style>
