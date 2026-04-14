<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Edit Provider</v-toolbar-title>
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
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { required } from '@/utils/validators';
import { useProviderStore } from '@/stores/provider';
import type { UpdateProviderDto } from '@/modules/provider/types';

// Router & store
const route = useRoute();
const router = useRouter();
const providerStore = useProviderStore();

// Refs
const valid = ref(true);
const name = ref('');
const description = ref<string | null>(null);
const url = ref<string | null>(null);
const form = ref<any | null>(null);

// Rules
const nameRules = [required];
const urlRules = [];

// Route param
const providerId = computed(() => Number(route.params.id));

// Current provider
const provider = computed(() => providerStore.getProvider(providerId.value));

// Reset form from store state
function reset() {
  name.value = provider.value?.name || '';
  description.value = provider.value?.description || null;
  url.value = provider.value?.url || null;
  form.value?.resetValidation();
}

// Cancel edit
function cancel() {
  router.back();
}

// Submit update
async function submit() {
  if (form.value?.validate() && provider.value) {
    const data: UpdateProviderDto = {
      name: name.value,
      description: description.value,
      url: url.value,
    };
    await providerStore.updateProvider({ id: provider.value.id, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

// Load data on mount
onMounted(async () => {
  await providerStore.getProvider(providerId.value);
  reset();
});
</script>
