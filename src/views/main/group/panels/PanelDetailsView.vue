<template>
  <v-card flat>
    <v-card-title>Panel Details</v-card-title>
    <v-card-text>
      <v-tabs v-model="tab" bg-color="transparent">
        <v-tab :value="0">Info</v-tab>
      </v-tabs>

      <v-tabs-window v-model="tab">
        <v-tabs-window-item :value="0">
          <PanelView :panel-id="panel.id" />
        </v-tabs-window-item>
      </v-tabs-window>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import PanelView from '@/views/main/group/panels/PanelView.vue'
import type { PanelDto } from '@/modules/panel/types'

const props = defineProps<{
  panel: PanelDto
}>()

const tab = ref(0)
</script>
