<template>
  <v-card flat class="my-2">
    <v-card-text class="text-subtitle-2">Lots</v-card-text>
    <v-data-table
      :headers="headers"
      :items="items"
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
            name: 'main-group-lots-edit',
            params: {
              groupId: activeGroupId,
              id: item.id,
            },
          }"
        >
          {{ item.id }}
        </router-link>
      </template>

      <template v-slot:item.clone="{ item }">
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
          {{ cloneNameMap.get(item.cloneId) ?? '—' }}
        </router-link>
      </template>

      <template v-slot:item.status="{ item }">
        <v-chip :color="getLotStatusColor(item.status as LotStatus)" class="mr-1" x-small dark label>
          {{ lotStatusToString(item.status) }}
        </v-chip>
      </template>
    </v-data-table>
  </v-card>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { storeToRefs } from 'pinia';
import { useGroupStore } from '@/stores/group';
import { useProviderStore } from '@/stores/provider';
import { getLotStatusColor, lotStatusToString } from '@/utils/converters';
import { useCloneStore } from '@/stores/clone';
import { LotStatus } from '@/modules/lot/LotStatus';
import type { ProviderDto } from '@/modules/provider/types';
import type { LotDto } from '@/modules/lot/types';

// Props
const props = defineProps<{
  provider: ProviderDto;
}>();

// Stores
const groupStore = useGroupStore();
const providerStore = useProviderStore();
const cloneStore = useCloneStore();
const { activeGroupId } = storeToRefs(groupStore);

// State
const items = ref<LotDto[]>([]);
const loading = ref(true);
const cloneNameMap = computed(() => {
  const map = new Map<number, string>();
  cloneStore.clones.forEach((clone) => {
    map.set(clone.id, clone.name);
  });
  return map;
});

const rawHeaders = [
  { title: 'Id', key: 'id', align: 'end', width: 80 },
  { title: 'Number', key: 'number' },
  { title: 'Reference', key: 'reference' },
  { title: 'Name', key: 'name' },
  {
    title: 'Clone',
    key: 'clone',
    sort: (a: any, b: any) => {
      if (!a) return 1
      if (!b) return -1
      return a.name.localeCompare(b.name)
    },
  },
  { title: 'Status', key: 'status' },
  { title: 'Purpose', key: 'purpose' },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))

// Load data
onMounted(async () => {
  items.value = await providerStore.getProviderLots(props.provider.id);
  const cloneIds = items.value
    .map(item => item.cloneId)
    .filter((id): id is number => typeof id === 'number');
  await cloneStore.fetchByIds(cloneIds);
  loading.value = false;
});
</script>
