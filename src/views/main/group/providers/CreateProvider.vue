<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Create Provider</v-toolbar-title>
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
          <v-text-field label="Description" v-model="description" />
          <v-text-field label="URL" v-model="url" :rules="urlRules" />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { storeToRefs } from 'pinia';
import { required } from '@/utils/validators';
import { useGroupStore } from '@/stores/group';
import { useProviderStore } from '@/stores/provider';
import type { CreateProviderDto } from '@/modules/provider/types';

const router = useRouter();
const route = useRoute();

// Stores
const groupStore = useGroupStore();
const providerStore = useProviderStore();
const { activeGroupId } = storeToRefs(groupStore);

// Form state
const valid = ref(true);
const form = ref();
const name = ref('');
const description = ref<string | null>(null);
const url = ref<string | null>(null);

// Validation rules
const nameRules = [required];
const urlRules = []; // Add URL validation if needed

// Actions
const reset = () => {
  name.value = '';
  description.value = null;
  url.value = null;
  form.value?.resetValidation();
};

const cancel = () => {
  router.back();
};

const submit = async () => {
  if (form.value?.validate() && activeGroupId.value) {
    const data: CreateProviderDto = {
      groupId: activeGroupId.value,
      name: name.value,
      description: description.value,
      url: url.value,
    };
    await providerStore.createProvider(data);
    router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''));
  }
};
</script>
