<template>
  <v-col>
    <v-toolbar dense class="toolbar">
      <v-toolbar-title>Groups</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn v-if="isAdmin" text @click="exportAllData" color="primary">
          <v-icon small left>mdi-cloud-download</v-icon>
          Export All Data
        </v-btn>
        <v-btn v-if="isAdmin" text @click="importAllData" color="primary">
          <v-icon small left>mdi-cloud-upload</v-icon>
          Import All Data
        </v-btn>
        <v-divider vertical />
        <v-btn v-if="isAdmin" text @click="importGroupData" color="primary">
          <v-icon small left>mdi-cloud-upload</v-icon>
          Import Group
        </v-btn>
        <v-btn v-if="isAdmin" text to="/main/admin/groups/create" color="primary">Create Group</v-btn>
      </v-toolbar-items>
      <input ref="groupFileInput" class="visually-hidden" type="file" @change="groupFiles" />
      <input ref="allFileInput" class="visually-hidden" type="file" @change="allFiles" />
    </v-toolbar>

    <v-card>
      <v-card-title>
        <v-spacer />
        <v-text-field v-model="search" append-icon="mdi-magnify" label="Search" single-line hide-details clearable />
      </v-card-title>

      <v-data-table
        :headers="headers"
        :items="groups"
        :loading="!groups.length"
        :search="search"
        :items-per-page="15"
        :footer-props="{
          itemsPerPageOptions: [],
          showFirstLastPage: true,
          showCurrentPage: true,
        }"
        multi-sort
      >
        <template v-slot:item.isOpen="{ item }">
          <v-icon v-if="item.isOpen">mdi-check</v-icon>
        </template>

        <template v-slot:item.action="{ item }">
          <v-menu bottom left>
            <template v-slot:activator="{ props }">
              <v-btn icon v-bind="props">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
            <v-list dense>
              <v-list-item :to="{ name: 'main-admin-groups-edit', params: { id: item.id } }">
                <v-icon color="primary">mdi-pencil-outline</v-icon>
                <v-list-item-title>Edit</v-list-item-title>
              </v-list-item>

              <v-list-item @click="exportGroupData(item.id)">
                <v-icon color="grey">mdi-database-export</v-icon>
                <v-list-item-title>Export</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </template>
      </v-data-table>
    </v-card>
  </v-col>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue';
import { useGroupStore } from '@/stores/group'
import { useMainStore } from '@/stores/main'

const groupStore = useGroupStore()
const mainStore = useMainStore()

const groupFileInput = ref<HTMLInputElement | null>(null)
const allFileInput = ref<HTMLInputElement | null>(null)
const search = ref('')

const isAdmin = computed(() => mainStore.isAdmin)
const groups = computed(() => groupStore.groups)

const rawHeaders = [
  { title: 'Id', key: 'id', align: 'end', sortable: true, width: 80 },
  { title: 'Name', key: 'name', align: 'start', sortable: true },
  { title: 'Institution', key: 'institution', align: 'start', sortable: true },
  { title: 'Public', key: 'isOpen', align: 'start', sortable: true, width: 100 },
  { title: 'Actions', key: 'action', align: 'start', sortable: false, width: 70 },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))

const importGroupData = () => groupFileInput.value?.click()
const importAllData = () => allFileInput.value?.click()

const groupFiles = async (e: Event) => {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  const formData = new FormData()
  formData.append('file', file, file.name)
  input.value = ''
  await groupStore.importGroupData(formData)
}

const allFiles = async (e: Event) => {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  if (confirm('Import AirLab data?') && confirm('All existing data will be overwritten! Are you sure?')) {
    const formData = new FormData()
    formData.append('file', file, file.name)
    input.value = ''
    await groupStore.importAllData(formData)
  }
}

const exportGroupData = async (id: number) => {
  if (confirm('Download all group data as .zip file?')) {
    await groupStore.exportGroupData(id, 'json')
  }
}

const exportAllData = async () => {
  if (confirm('Download all data as .zip file?')) {
    await groupStore.exportAllData('json')
  }
}

onMounted(async () => {
  await groupStore.getGroups()
})
</script>

<style scoped>
.toolbar {
  margin-bottom: 10px;
}
.visually-hidden {
  position: absolute !important;
  height: 1px;
  width: 1px;
  overflow: hidden;
  clip: rect(1px, 1px, 1px, 1px);
}
</style>
