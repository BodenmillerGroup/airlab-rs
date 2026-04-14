<template>
  <v-container fluid>
    <v-toolbar density="compact">
      <v-toolbar-title>Edit Protein</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn @click="cancel" variant="text" color="primary">Cancel</v-btn>
        <v-btn @click="reset" variant="text" color="primary">Reset</v-btn>
        <v-btn @click="submit" variant="text" color="primary" :disabled="!valid">Save</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form v-model="valid" ref="form">
          <v-text-field label="Name" v-model="name" :rules="nameRules" />
          <v-text-field label="Description" v-model="description" />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { required } from '@/utils/validators';
import { useProteinStore } from '@/stores/protein';
import type { UpdateProteinDto } from '@/modules/protein/types';
import { storeToRefs } from 'pinia'

// 🚀 Router
const router = useRouter();
const route = useRoute();

// 🧪 Form State
const form = ref();
const valid = ref(false);


const nameRules = [required];

// 🧬 Store
const proteinStore = useProteinStore();
const proteinId = computed(() => Number(route.params.id));
const protein = computed(() => proteinStore.getProteinById(proteinId.value));

const name = ref('');
const description = ref('');

// 🔄 Form Actions
function reset() {
  form.value?.resetValidation();
  if (protein.value) {
    name.value = protein.value.name;
    description.value = protein.value.description;
  }
}

function cancel() {
  router.back();
}

async function submit() {
  if (form.value?.validate()) {
    const data: UpdateProteinDto = {
      name: name.value,
      description: description.value,
    };
    await proteinStore.updateProtein({ id: proteinId.value, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  await proteinStore.getProteinById(proteinId.value);
  reset();
});

</script>

