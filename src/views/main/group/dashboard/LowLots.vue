<template>
  <v-card>
    <v-card-title>Low Lots</v-card-title>
    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="lowItems"
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
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useLotStore } from '@/stores/lot';
import { useProviderStore } from '@/stores/provider';
import { useCloneStore } from '@/stores/clone';
import type { LotDto } from '@/modules/lot/types';

const route = useRoute();
const lotStore = useLotStore();
const providerStore = useProviderStore();
const cloneStore = useCloneStore();

const lowItems = ref<LotDto[]>([]);
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
  lowItems.value = await lotStore.getLowLots(groupId);
  const providerIds = lowItems.value
    .map((lot) => lot.providerId)
    .filter((id): id is number => typeof id === 'number')
  await providerStore.fetchByIds(groupId, providerIds)
  const cloneIds = lowItems.value
    .map((lot) => lot.cloneId)
    .filter((id): id is number => typeof id === 'number')
  await cloneStore.fetchByIds(cloneIds)
  loading.value = false;
});
</script>
