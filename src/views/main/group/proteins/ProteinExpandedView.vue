<template>
  <v-card flat class="my-2">
    <v-card-text class="text-subtitle-2">Clones</v-card-text>

    <v-data-table
      :headers="headers"
      :items="items"
      :loading="loading"
      :items-per-page="-1"
      hide-default-footer
      disable-filtering
      density="compact"
    >
      <template #item.id="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-clones-edit',
            params: { groupId: activeGroupId, id: item.id },
          }"
        >
          {{ item.id }}
        </router-link>
      </template>

      <template #item.species="{ item }">
        <router-link
          v-if="item.speciesId" 
          class="link"
          :to="{
            name: 'main-group-species-edit',
            params: { groupId: activeGroupId, id: item.speciesId },
          }"
        >
          {{ item.speciesName }}
        </router-link>
        <span v-else class="text-disabled">Unknown</span>
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
          dark
        >
          {{ validation.application }}
        </v-chip>
      </template>
    </v-data-table>
  </v-card>
</template>

<script lang="ts" setup>
import { ref, onMounted, computed } from 'vue';
import { useProteinStore } from '@/stores/protein';
import { useGroupStore } from '@/stores/group';
import { applicationToString } from '@/utils/converters';
import type { ProteinDto } from '@/modules/protein/types';
import type { CloneDto } from '@/modules/clone/types';

const props = defineProps<{
  protein: ProteinDto;
}>();

const groupStore = useGroupStore();
const proteinStore = useProteinStore();

const activeGroupId = computed(() => groupStore.activeGroupId);

import { useClones } from '@/composables/useClones'
const { items, loading, reload } = useClones( { proteinId: props.protein.id })

const rawHeaders = [
  { title: 'Id', key: 'id', align: 'end', sortable: true, width: 80 },
  { title: 'Name', key: 'name' },

  {
    title: 'Host',
    key: 'species',
    sort: (a: string | undefined, b: string | undefined) => (a ?? '').localeCompare(b ?? ''),
  },

  { title: 'Isotype', key: 'isotype' },
  { title: 'Phospho', key: 'isPhospho', sortable: false },
  { title: 'Polyclonal', key: 'isPolyclonal', sortable: false },
  { title: 'Validations', key: 'validations', sortable: false },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))

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

<style scoped>
.link {
  color: inherit;
  text-decoration: none;
}
</style>
