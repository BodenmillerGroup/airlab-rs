<script setup lang="ts">
import { ref, computed } from 'vue';
import { useGroupStore } from '@/stores/group';
import ConjugateView from '@/views/main/group/conjugates/ConjugateView.vue';
import ValidationView from '@/views/main/group/validations/ValidationView.vue';
import { apiUrl } from '@/env';
import type { ConjugateDto } from '@/modules/conjugate/types';
import { useValidations } from '@/composables/useValidations';

const props = defineProps<{ conjugate: ConjugateDto }>();

const tab = ref(0);
const { items: allValidations } = useValidations();

const groupStore = useGroupStore();
const activeGroupId = computed(() => groupStore.activeGroupId);
const validations = computed(() =>
  allValidations.value.filter(v => v.conjugateId === props.conjugate.id)
);
</script>

<template>
  <v-card flat>
    <v-card-title>Conjugate Details</v-card-title>
    <v-card-text>
      <v-tabs v-model="tab">
        <v-tab :value="0">Info</v-tab>
        <v-tab v-if="validations.length > 0" :value="1">Validations</v-tab>
      </v-tabs>

      <v-tabs-window v-model="tab">
        <v-tabs-window-item :value="0">
          <ConjugateView :conjugate-id="props.conjugate.id" />
        </v-tabs-window-item>

        <v-tabs-window-item v-if="validations.length > 0" :value="1">
          <ValidationView
            v-for="validation in validations"
            :key="validation.id"
            :group-id="activeGroupId"
            :validation="validation"
            :api-url="apiUrl"
          />
        </v-tabs-window-item>
      </v-tabs-window>
    </v-card-text>
  </v-card>
</template>
