<template>
  <v-card flat class="my-2">
    <v-card-text class="text-subtitle-2">Conjugate Elements</v-card-text>
    <v-data-table
      :headers="headers"
      :items="items"
      :loading="loading"
      :items-per-page="-1"
      hide-default-footer
      density="compact"
    >
      <template #item.dilutionType="{ item }">
        {{ dilutionTypeToString(item.dilutionType) }}
      </template>

      <template #item.tubeNumber="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-conjugates-edit',
            params: { groupId: activeGroupId, id: item.conjugateId },
          }"
        >
          {{ item.tubeNumber }}
        </router-link>
      </template>

      <template #item.lot="{ item }">
        <router-link
          class="link"
          :to="{
            name: 'main-group-lots-edit',
            params: { groupId: activeGroupId, id: item.lotId },
          }"
        >
          {{ item.lotNumber }}
        </router-link>
      </template>
    </v-data-table>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useGroupStore } from '@/stores/group'
import { dilutionTypeToString } from '@/utils/converters'
import type { PanelDto } from '@/modules/panel/types'
import { usePanelElements } from '@/composables/usePanelElements'

// Props
const props = defineProps<{
  panel: PanelDto
}>()

// Stores
const groupStore = useGroupStore()
const panelId = computed(() => props.panel.id)
const groupId = computed(() => groupStore.activeGroupId ?? null)
const { items, loading } = usePanelElements(panelId, groupId)

// Computed
const activeGroupId = computed(() => groupStore.activeGroupId)

const rawHeaders = [
  {
    text: 'Tube Number',
    value: 'tubeNumber',
  },
  {
    text: 'Lot',
    value: 'lot',
  },
  {
    text: 'Dilution Type',
    value: 'dilutionType',
  },
  {
    text: 'Concentration',
    value: 'concentration',
    align: 'end',
  },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))

</script>
