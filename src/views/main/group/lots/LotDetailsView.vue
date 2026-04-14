<template>
  <v-card flat>
    <v-card-title>Lot Details</v-card-title>
    <v-card-text>
      <v-tabs v-model="tab">
        <v-tab>Info</v-tab>
        <v-tab v-if="validations.length > 0">Validations</v-tab>
        <v-tab v-if="lot.meta">Metadata</v-tab>
      </v-tabs>
      <v-window v-model="tab" class="mt-4">
        <v-window-item value="info">

          <LotView :lot-id="lot.id" />
        </v-window-item>

        <v-window-item v-if="validations.length > 0">
          <p>Validations go heere -- fixme</p>
          <!--
          <ValidationView
            v-for="validation in validations"
            :key="validation.id"
            :group-id="activeGroupId"
            :validation="validation"
            :api-url="apiUrl"
          />
            -->

        <!--<v-tab-item v-if="lot.meta">
          <InfoView :item="lot" />
        </v-tab-item>-->
          </v-window-item>
        </v-window>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import InfoView from '@/components/InfoView.vue';
import LotView from '@/views/main/group/lots/LotView.vue';
import ValidationView from '@/views/main/group/validations/ValidationView.vue';

import type { LotDto } from '@/modules/lot/types';
import type { ValidationDto } from '@/modules/validation/types';

import { useGroupStore } from '@/stores/group';
import { useLotStore } from '@/stores/lot';
import { apiUrl } from '@/env';

// ✅ Props
const props = defineProps<{
  lot: LotDto;
}>();

// 📦 Stores
const groupStore = useGroupStore();
const lotStore = useLotStore();

// 🔢 State
const tab = ref(0);
const validations = ref<ValidationDto[]>([]);

// 🧠 Computed
const activeGroupId = computed(() => groupStore.activeGroupId);

// 🌐 API reference
const apiUrlRef = apiUrl;

// 🧬 Fetch validations on mount
onMounted(async () => {
  if (props.lot && props.lot.id) {
    validations.value = await lotStore.getLotValidations(props.lot.id)
  } else {
    console.warn('⚠️ props.lot is undefined or missing id', props.lot)
  }
});
</script>
