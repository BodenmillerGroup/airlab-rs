<template>
  <v-card flat>
    <v-card-title>Provider Details</v-card-title>

    <v-card-text>
      <v-tabs v-model="tab">
        <v-tab value="info">Info</v-tab>
      </v-tabs>

      <v-window v-model="tab">
        <v-window-item value="info">
          <ProviderView :provider-id="provider.id" />
        </v-window-item>
      </v-window>
    </v-card-text>
  </v-card>
</template>


<script setup lang="ts">
import { ref } from 'vue';
import ProviderView from '@/views/main/group/providers/ProviderView.vue';
import type { ProviderDto } from '@/modules/provider/types';

// Props
const props = defineProps<{
  provider: ProviderDto;
}>();

const tab = ref<'info'>('info')

</script>

