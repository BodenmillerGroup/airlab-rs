<template>
  <v-card flat>
    <v-card-title>Species Details</v-card-title>
    <v-card-text>
      <v-tabs v-model="tab">
        <v-tab value="info">Info</v-tab>
      </v-tabs>
      <v-window v-model="tab">
        <v-window-item value="info">
          <SpeciesView :species-id="species.id" />
        </v-window-item>
      </v-window>
    </v-card-text>
  </v-card>
</template>


<script setup lang="ts">
import { ref } from 'vue';
import SpeciesView from '@/views/main/group/species/SpeciesView.vue';
import type { SpeciesDto } from '@/modules/species/types';

defineProps<{
  species: SpeciesDto;
}>();

const tab = ref<'info'>('info')
</script>

