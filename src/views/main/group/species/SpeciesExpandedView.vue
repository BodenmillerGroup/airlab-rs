<template>
  <v-tabs v-model="tab" class="my-1">
    <v-tab>Hosts</v-tab>
    <v-tab>Reactivity</v-tab>
  </v-tabs>

  <v-window v-model="tab">
    <v-window-item :value="0">
      <v-data-table
        :headers="headers"
        :items="hostItems"
        :loading="loading"
        :items-per-page="-1"
        hide-default-footer
        disable-filtering
        dense
      >
        <template v-slot:item.id="{ item }">
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
        <template v-slot:item.protein="{ item }">
          <router-link
            v-if="item.proteinId"
            class="link"
            :to="{
              name: 'main-group-proteins-edit',
              params: { groupId: activeGroupId, id: item.proteinId },
            }"
          >
            {{ item.proteinName }}
          </router-link>
        </template>
        <template v-slot:item.isPhospho="{ item }">
          <v-icon v-if="item.isPhospho">mdi-check</v-icon>
        </template>
        <template v-slot:item.isPolyclonal="{ item }">
          <v-icon v-if="item.isPolyclonal">mdi-check</v-icon>
        </template>
        <template v-slot:item.validations="{ item }">
          <v-chip
            v-for="validation in item.validations"
            :key="validation.id"
            :color="getValidationStatusColor(validation.status)"
            class="mr-1"
            x-small
            dark
          >
            {{ validation.application }}
          </v-chip>
        </template>
      </v-data-table>
    </v-window-item>

    <v-window-item :value="1">
      <v-data-table
        :headers="headers"
        :items="reactivityItems"
        :loading="loading"
        :items-per-page="-1"
        hide-default-footer
        disable-filtering
        dense
      >
        <template v-slot:item.id="{ item }">
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
        <template v-slot:item.protein="{ item }">
          <router-link
            v-if="item.proteinId"
            class="link"
            :to="{
              name: 'main-group-proteins-edit',
              params: { groupId: activeGroupId, id: item.proteinId },
            }"
          >
            {{ item.proteinName }}
          </router-link>
        </template>
        <template v-slot:item.isPhospho="{ item }">
          <v-icon v-if="item.isPhospho">mdi-check</v-icon>
        </template>
        <template v-slot:item.isPolyclonal="{ item }">
          <v-icon v-if="item.isPolyclonal">mdi-check</v-icon>
        </template>
        <template v-slot:item.validations="{ item }">
          <v-chip
            v-for="validation in item.validations"
            :key="validation.id"
            :color="getValidationStatusColor(validation.status)"
            class="mr-1"
            x-small
            dark
          >
            {{ validation.application }}
          </v-chip>
        </template>
      </v-data-table>
    </v-window-item>
  </v-window>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

import { useGroupStore } from '@/stores/group';
import { useClones } from '@/composables/useClones';

import type { SpeciesDto } from '@/modules/species/types';

// Props
const props = defineProps<{
  species: SpeciesDto;
}>();

// Stores
const groupStore = useGroupStore();
const activeGroupId = computed(() => groupStore.activeGroupId);
const { items: clones, loading } = useClones();

const tab = ref(0);

const rawHeaders = [
  { title: 'Id', key: 'id', align: 'end', width: 80 },
  { title: 'Name', key: 'name' },
  {
    title: 'Protein',
    key: 'protein',
    sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? ''),
  },
  { title: 'Isotype', key: 'isotype' },
  { title: 'Phospho', key: 'isPhospho' },
  { title: 'Polyclonal', key: 'isPolyclonal' },
  { title: 'Validations', key: 'validations' },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))

const hostItems = computed(() =>
  clones.value.filter((c) => c.speciesId === props.species.id)
);

const reactivityItems = computed(() =>
  clones.value.filter((c) => Array.isArray(c.reactivity) && c.reactivity.includes(props.species.id))
);

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
