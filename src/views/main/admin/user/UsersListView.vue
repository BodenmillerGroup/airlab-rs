<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Users</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn v-if="isAdmin" text to="/main/admin/users/create" color="primary">Create User</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-card>
      <v-card-title>
        <v-spacer />
        <v-text-field v-model="search" append-icon="mdi-magnify" label="Search" single-line hide-details clearable />
      </v-card-title>

      <v-data-table
        :headers="headers"
        :items="users"
        :loading="!users.length"
        :search="search"
        :items-per-page="15"
        :footer-props="{
          itemsPerPageOptions: [],
          showFirstLastPage: true,
          showCurrentPage: true,
        }"
        multi-sort
      >
        <template v-slot:item.isActive="{ item }">
          <v-icon v-if="item.isActive">mdi-check</v-icon>
        </template>

        <template v-slot:item.isAdmin="{ item }">
          <v-icon v-if="item.isAdmin">mdi-check</v-icon>
        </template>

        <template v-slot:item.action="{ item }">
          <v-menu bottom left>
            <template v-slot:activator="{ props }">
              <v-btn icon v-bind="props">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
            <v-list dense>
              <v-list-item :to="{ name: 'main-admin-users-edit', params: { id: item.id } }">
                  <v-icon color="grey">mdi-pencil-outline</v-icon>
                  <v-list-item-title>Edit</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </template>
      </v-data-table>
    </v-card>
  </v-col>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue'
import { useMainStore } from '@/stores/main'
import { useUserStore } from '@/stores/user'

const mainStore = useMainStore()
const userStore = useUserStore()

const isAdmin = computed(() => mainStore.isAdmin)
const users = computed(() => userStore.users)

const search = ref('')

const rawHeaders = [
  { title: 'Id', key: 'id', align: 'end', sortable: true, width: 80 },
  { title: 'Email', key: 'email', align: 'start', sortable: true },
  { title: 'Name', key: 'name', align: 'start', sortable: true },
  { title: 'Active', key: 'isActive', align: 'start', sortable: true, width: 110 },
  { title: 'Admin', key: 'isAdmin', align: 'start', sortable: true, width: 110 },
  { title: 'Actions', key: 'action', align: 'start', sortable: false, width: 70 },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))

onMounted(async () => {
  await userStore.getUsers()
})
</script>

<style scoped>
.toolbar {
  margin-bottom: 10px;
}
</style>
