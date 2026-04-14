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
      show-expand
    >
      <template v-slot:item.id="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-lots-edit',
            params: { groupId: activeGroupId, id: item.id },
          }"
        >
          {{ item.id }}
        </router-link>
      </template>

      <template v-slot:item.provider="{ item }">
        <router-link
          v-if="item.providerId"
          class="link"
          :to="{
            name: 'main-group-providers-edit',
            params: { groupId: activeGroupId, id: item.providerId },
          }"
        >
          {{ providerStore.getProvider(item.providerId)?.name ?? '—' }}
        </router-link>
      </template>

      <template v-slot:item.status="{ item }">
        <v-chip :color="getLotStatusColor(item.status as LotStatus)" class="mr-1" x-small dark label>
          {{ lotStatusToString(item.status) }}
        </v-chip>
      </template>

      <template v-slot:expanded-row="{ columns, item }">
        <td :colspan="columns.length">
          <CloneLotExpandedView :lot="item" />
        </td>
      </template>
    </v-data-table>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useGroupStore } from "@/stores/group";
import { getLotStatusColor } from "@/utils/converters";
import CloneLotExpandedView from "@/views/main/group/clones/CloneLotExpandedView.vue";
import type { CloneDto } from "@/modules/clone/types";
import { lotStatusToString } from '@/utils/converters'
import { useLots } from '@/composables/useLots'
import { useProviderStore } from '@/stores/provider'
import { LotStatus } from '@/modules/lot/LotStatus'

const props = defineProps<{ clone: CloneDto }>();

const groupStore = useGroupStore();
const providerStore = useProviderStore();

const activeGroupId = computed(() => groupStore.activeGroupId);

const { items, loading, reload } = useLots( { cloneId: props.clone.id })

const rawHeaders = [
  { title: "Id", key: "id", align: "end", filterable: false, width: "80" },
  { title: "Number", key: "number" },
  { title: "Reference", key: "reference" },
  { title: "Name", key: "name" },
  {
    title: "Provider",
    key: "provider",
    sort: (a: any, b: any) => {
      if (a === null) return 1;
      if (b === null) return -1;
      return a.name.localeCompare(b.name);
    },
  },
  { title: "Status", key: "status" },
  { title: "Purpose", key: "purpose" },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))


</script>
