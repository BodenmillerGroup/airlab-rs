<template>
  <v-card flat class="my-2">
    <v-card-text class="text-subtitle-2">Conjugates</v-card-text>
    <v-data-table
      :headers="headers"
      :items="items"
      :loading="loading"
      :items-per-page="-1"
      hide-default-footer
      disable-filtering
      dense
    >
      <template #item.id="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-conjugates-edit',
            params: {
              groupId: activeGroupId,
              id: item.id,
            },
          }"
        >
          {{ item.id }}
        </router-link>
      </template>

      <template #item.protein="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-proteins-edit',
            params: {
              groupId: activeGroupId,
              id: item.proteinId,
            },
          }"
        >
          {{ item.proteinName }}
        </router-link>
      </template>

      <template #item.clone="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-clones-edit',
            params: {
              groupId: activeGroupId,
              id: item.cloneId,
            },
          }"
        >
          {{ item.cloneName }}
        </router-link>
      </template>

      <template #item.tag="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-tags-edit',
            params: {
              groupId: activeGroupId,
              id: item.tagId,
            },
          }"
        >
          {{ item.tagMw ? item.tagName + item.tagMw : item.tagName }}
        </router-link>
      </template>

      <template #item.user="{ item }">
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

      <template #item.status="{ item }">
        <v-chip :color="getConjugateStatusColorWrapper(item.status)" class="mr-1" x-small dark label>
          {{ conjugateStatusToString(item.status) }}
        </v-chip>
      </template>
    </v-data-table>
  </v-card>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useLotStore } from '@/stores/lot';
import { useGroupStore } from '@/stores/group';
import { getConjugateStatusColor, conjugateStatusToString } from '@/utils/converters';
import type { LotDto } from '@/modules/lot/types';
import type { ConjugateDto } from '@/modules/conjugate/types';

const props = defineProps<{
  lot: LotDto;
}>();

// Stores
const lotStore = useLotStore();
const groupStore = useGroupStore();

const activeGroupId = computed(() => groupStore.activeGroupId);

const getConjugateStatusColorRef = getConjugateStatusColor;

import { useConjugates } from '@/composables/useConjugates'
const { items, loading, reload } = useConjugates( { lotId: props.lot.id })

function getConjugateStatusColorWrapper(status: number) {
  return getConjugateStatusColor({ status } as ConjugateDto);
}


const rawHeaders = [
  { title: 'Id', key: 'id', align: 'end', sortable: true, width: 80 },
  { title: 'Tube Number', key: 'tubeNumber', align: 'end' },

  {
    title: 'Protein',
    key: 'protein',
    sort: (a: any, b: any) =>
      a && b ? a.name.localeCompare(b.name) : a ? -1 : 1,
  },

  {
    title: 'Clone',
    key: 'clone',
    sort: (a: any, b: any) =>
      a && b ? a.name.localeCompare(b.name) : a ? -1 : 1,
  },

  {
    title: 'Tag',
    key: 'tag',
    sort: (a: any, b: any) =>
      a && b ? a.name.localeCompare(b.name) : a ? -1 : 1,
  },

  {
    title: 'Labeled by',
    key: 'user',
    sort: (a: any, b: any) =>
      a && b ? a.name.localeCompare(b.name) : a ? -1 : 1,
  },

  { title: 'Concentration', key: 'concentration', align: 'end' },
  { title: 'Status', key: 'status', sortable: false },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))


</script>
