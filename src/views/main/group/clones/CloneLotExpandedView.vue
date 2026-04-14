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
      <template v-slot:item.id="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-conjugates-edit',
            params: { groupId: activeGroupId, id: item.id },
          }"
        >
          {{ item.id }}
        </router-link>
      </template>

      <template v-slot:item.proteinName="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-proteins-edit',
            params: { groupId: activeGroupId, id: item.proteinId },
          }"
        >
          {{ item.proteinName }}
        </router-link>
      </template>

      <template v-slot:item.cloneName="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-clones-edit',
            params: { groupId: activeGroupId, id: item.cloneId },
          }"
        >
          {{ item.cloneName }}
        </router-link>
      </template>

      <template v-slot:item.tagName="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-tags-edit',
            params: { groupId: activeGroupId, id: item.tagId },
          }"
        >
          {{ item.tagMw ? item.tagName + item.tagMw : item.tagName }}
        </router-link>
      </template>

      <template v-slot:item.userName="{ item }">
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

      <template v-slot:item.status="{ item }">
        <v-chip :color="getConjugateStatusColorWrapper(item.status)" class="mr-1" x-small dark label>
          {{ conjugateStatusToString(item.status) }}
        </v-chip>
      </template>
    </v-data-table>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useGroupStore } from "@/stores/group";
import { getConjugateStatusColor } from "@/utils/converters";
import { conjugateStatusToString } from "@/utils/converters"
import type { LotDto } from "@/modules/lot/types";
import type { ConjugateDto, ConjugateView } from "@/modules/conjugate/types";
import { useConjugates } from "@/composables/useConjugates";

// Props
const props = defineProps<{ lot: LotDto }>();

// Stores
const groupStore = useGroupStore();

// State
const { items, loading } = useConjugates({ lotId: props.lot.id });
const activeGroupId = computed(() => groupStore.activeGroupId);

const rawHeaders = [
  { title: "Id", key: "id", align: "end", width: 80 },
  { title: "Tube Number", key: "tubeNumber", align: "end" },
  {
    title: "Protein",
    key: "proteinName",
    sort: (a: string, b: string) => (a ?? "").localeCompare(b ?? ""),
  },
  {
    title: "Clone",
    key: "cloneName",
    sort: (a: string, b: string) => (a ?? "").localeCompare(b ?? ""),
  },
  {
    title: "Tag",
    key: "tagName",
    sort: (a: string, b: string) => (a ?? "").localeCompare(b ?? ""),
  },
  {
    title: "Labeled by",
    key: "userName",
    sort: (a: string, b: string) => (a ?? "").localeCompare(b ?? ""),
  },
  { title: "Concentration", key: "concentration", align: "end" },
  { title: "Status", key: "status" },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))

function getConjugateStatusColorWrapper(status: number) {
  return getConjugateStatusColor({ status } as ConjugateDto);
}
</script>
