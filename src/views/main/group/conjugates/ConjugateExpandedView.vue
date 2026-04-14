<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useGroupStore } from '@/stores/group';
import { applicationToString, getStatusColor } from '@/utils/converters';
import type { ConjugateDto } from '@/modules/conjugate/types';

const tab = ref<'panels' | 'clones'>('panels')

const props = defineProps<{ conjugate: ConjugateDto }>();

const groupStore = useGroupStore();

import { useClones } from '@/composables/useClones'
const { items: clones, loading } = useClones( { conjugateId: props.conjugate.id })
import { usePanels } from '@/composables/usePanels'
const { items: panels, loading: panelsLoading } = usePanels( { conjugateId: props.conjugate.id })

const activeGroupId = computed(() => groupStore.activeGroupId);

const panelsHeaders = [
  { title: 'Id', key: 'id', align: 'end', sortable: true, width: 80 },
  { title: 'Name', key: 'name' },
  { title: 'Description', key: 'description' },
  { title: 'Fluorophore', key: 'isFluorophore', sortable: false },
  { title: 'Locked', key: 'isLocked', sortable: false },

  {
    title: 'Application',
    key: 'application',
    sortable: true,
    sort: (a: any, b: any) => {
      if (a === null) return 1
      if (b === null) return -1
      return applicationToString(a).localeCompare(applicationToString(b))
    },
  },

  {
    title: 'Created by',
    key: 'userName',
    sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? ''),
  },
] as const



const clonesHeaders = [
  { title: 'Id', key: 'id', align: 'end', sortable: true, width: 80 },
  { title: 'Name', key: 'name' },

  {
    title: 'Host',
    key: 'speciesName',
    sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? ''),
  },

  { title: 'Isotype', key: 'isotype' },
  { title: 'Phospho', key: 'isPhospho', sortable: false },
  { title: 'Polyclonal', key: 'isPolyclonal', sortable: false },
  { title: 'Validations', key: 'validations', sortable: false },
] as const

function getValidationStatusColor(status: string) {
  switch (status) {
    case 'Yes':
      return 'green lighten-1';
    case 'So-So':
      return 'orange lighten-1';
    case 'No':
      return 'red lighten-1';
    default:
      return 'grey';
  }
}

</script>

<template>
  <v-tabs v-model="tab" class="my-1">
    <v-tab value="panels">Panels</v-tab>
    <v-tab value="clones">Clones</v-tab>
  </v-tabs>

  <v-window v-model="tab">
    <v-window-item value="panels">
      <v-data-table
        :headers="panelsHeaders"
        :items="panels"
        :loading="panelsLoading"
        :items-per-page="-1"
        hide-default-footer
        dense
      >
        <template #item.id="{ item }">
          <router-link
            class="link"
            :to="{ name: 'main-group-panels-edit', params: { groupId: activeGroupId, id: item.id } }"
          >
            {{ item.id }}
          </router-link>
        </template>

        <template #item.application="{ item }">
          {{ applicationToString(item.application) }}
        </template>

        <template #item.isFluorophore="{ item }">
          <v-icon v-if="item.isFluorophore">mdi-check</v-icon>
        </template>

        <template #item.isLocked="{ item }">
          <v-icon v-if="item.isLocked">mdi-lock-outline</v-icon>
        </template>

        <template #item.userName="{ item }">
          <router-link
            v-if="item.userId"
            class="link"
            :to="{ name: 'main-admin-users-edit', params: { id: item.userId } }"
          >
            {{ item.userName }}
          </router-link>
        </template>
      </v-data-table>
    </v-window-item>

    <v-window-item value="clones">
      <v-data-table
        :headers="clonesHeaders"
        :items="clones"
        :loading="loading"
        :items-per-page="-1"
        hide-default-footer
        dense
      >
        <template #item.id="{ item }">
          <router-link
            class="link"
            :to="{ name: 'main-group-clones-edit', params: { groupId: activeGroupId, id: item.id } }"
          >
            {{ item.id }}
          </router-link>
        </template>

        <template #item.speciesName="{ item }">
          <router-link
            v-if="item.speciesId"
            class="link"
            :to="{ name: 'main-group-species-edit', params: { groupId: activeGroupId, id: item.speciesId } }"
          >
            {{ item.speciesName }}
          </router-link>
        </template>

        <template #item.isPhospho="{ item }">
          <v-icon v-if="item.isPhospho">mdi-check</v-icon>
        </template>

        <template #item.isPolyclonal="{ item }">
          <v-icon v-if="item.isPolyclonal">mdi-check</v-icon>
        </template>

        <template #item.validations="{ item }">
          <v-chip
            v-for="validation in item.validations"
            :key="validation.id"
            :color="getValidationStatusColor(validation.status)"
            class="mr-1"
            size="x-small"
          >
            {{ validation.application }}
          </v-chip>
        </template>
      </v-data-table>
    </v-window-item>
  </v-window>
</template>
