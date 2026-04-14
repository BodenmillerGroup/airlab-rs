<template>
  <v-card>
    <v-card-title>Finished Lots</v-card-title>
    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="finishedItems"
        :loading="loading"
        disable-filtering
        disable-sort
        disable-pagination
        hide-default-footer
        dense
      >
        <template v-slot:item.provider="{ item }">
          {{ providerStore.getProvider(item.providerId)?.name || 'Loading...' }}
        </template>

        <template v-slot:item.clone="{ item }">
          {{ cloneStore.getClone(item.cloneId)?.name || 'Loading...' }}
        </template>

      </v-data-table>
    </v-card-text>
  </v-card>
</template>


<script lang="ts" setup>
import { ref, onMounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import { useLotStore } from '@/stores/lot';
import { useProviderStore } from '@/stores/provider';
import { useCloneStore } from '@/stores/clone';
import type { LotDto } from '@/modules/lot/types';

// FIXME: provider and clone are not loaded in pinia; make sure that the lotStore loads the canonical lots, and not the complex views; same for recent orders and low lots.

const route = useRoute();
const lotStore = useLotStore();
const providerStore = useProviderStore();
const cloneStore = useCloneStore();

const finishedItems = ref<LotDto[]>([]);
const loading = ref(true);

const rawHeaders = [
    {
      title: "Id",
      key: "id",
      align: "end",
      width: "80",
    },
    {
      title: "Name",
      key: "name",
    },
    {
      title: "Reference",
      key: "reference",
    },
    {
      title: "Provider",
      key: "provider",
    },
    {
      title: "Clone",
      key: "clone",
    },
  ] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))

onMounted(async () => {
  const groupId = +route.params.groupId;
  finishedItems.value = await lotStore.getFinishedLots(groupId);
  const providerIds = finishedItems.value
    .map((lot) => lot.providerId)
    .filter((id): id is number => typeof id === 'number')
  await providerStore.fetchByIds(groupId, providerIds)
  const cloneIds = finishedItems.value
    .map((lot) => lot.cloneId)
    .filter((id): id is number => typeof id === 'number')
  await cloneStore.fetchByIds(cloneIds)
  loading.value = false;
});
</script>
