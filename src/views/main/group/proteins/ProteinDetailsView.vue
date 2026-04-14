<template>
  <v-card flat>
    <v-card-title>Protein Details</v-card-title>
    <v-card-text>
      <v-tabs v-model="tab" class="mb-2">
        <v-tab>Info</v-tab>
      </v-tabs>

      <v-window v-model="tab">
        <v-window-item :value="0">
          <ProteinView :protein-id="protein.id" />
        </v-window-item>
      </v-window>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import type { ProteinDto } from '@/modules/protein/types';
import ProteinView from '@/views/main/group/proteins/ProteinView.vue';

interface Props {
  protein: ProteinDto;
}

const props = defineProps<Props>();

const tab = ref(0);
</script>
