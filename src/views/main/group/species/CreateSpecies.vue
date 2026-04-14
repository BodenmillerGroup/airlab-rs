<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Create Species</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn @click="cancel" text color="primary">Cancel</v-btn>
        <v-btn @click="reset" text color="primary">Reset</v-btn>
        <v-btn @click="submit" text :disabled="!valid" color="primary">Save</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form v-model="valid" ref="form" lazy-validation>
          <v-text-field label="Name" v-model="name" :rules="nameRules" />
          <v-text-field label="Acronym" v-model="acronym" :rules="acronymRules" />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useSpeciesStore } from '@/stores/species';
import { useGroupStore } from '@/stores/group';
import type { CreateSpeciesDto } from '@/modules/species/types';
import { required } from '@/utils/validators';

// Stores
const speciesStore = useSpeciesStore();
const groupStore = useGroupStore();
const router = useRouter();
const route = useRoute();

// Form state
const name = ref('');
const acronym = ref('');
const valid = ref(true);

// Rules
const nameRules = [required];
const acronymRules = [required];

// Refs
const form = ref<any | null>(null);

// Computed
const activeGroupId = computed(() => groupStore.activeGroupId);

// Handlers
function reset() {
  name.value = '';
  acronym.value = '';
  form.value?.resetValidation();
}

function cancel() {
  router.back();
}

async function submit() {
  if (form.value?.validate() && activeGroupId.value) {
    const data: CreateSpeciesDto = {
      groupId: activeGroupId.value,
      name: name.value,
      acronym: acronym.value,
    };
    await speciesStore.createSpecies(data);
    router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''));
  }
}
</script>
