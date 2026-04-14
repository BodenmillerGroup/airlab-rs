<template>
  <v-container fluid>
    <v-toolbar density="compact">
      <v-toolbar-title>Edit Collection</v-toolbar-title>
      <v-spacer />
      <v-btn text @click="cancel" color="primary">Cancel</v-btn>
      <v-btn text @click="reset" color="primary">Reset</v-btn>
      <v-btn text @click="submit" :disabled="!valid" color="primary">Save</v-btn>
    </v-toolbar>

    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form ref="formRef" v-model="valid" validate-on="lazy">
          <v-text-field v-model="name" label="Name" :rules="requiredRules" />
          <v-textarea v-model="description" label="Description" rows="3" auto-grow />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { useCollectionStore } from '@/stores/collection';
import { required } from '@/utils/validators';

import type { UpdateCollectionDto } from '@/modules/collection/types';

const route = useRoute();
const router = useRouter();
const collectionStore = useCollectionStore();

const id = computed(() => Number(route.params.id));
const collection = computed(() => collectionStore.getCollection(id.value));

const formRef = ref();
const valid = ref(true);

const name = ref('');
const description = ref('');

const requiredRules = [required];

onMounted(async () => {
  await collectionStore.getCollectionById(id.value);
  reset();
});

function reset() {
  name.value = collection.value?.name ?? '';
  description.value = collection.value?.description ?? '';
  formRef.value?.resetValidation();
}

function cancel() {
  router.back();
}

async function submit() {
  const result = await formRef.value?.validate();
  if (!result?.valid) return;

  const data: UpdateCollectionDto = {
    name: name.value,
    description: description.value.trim() || null,
  };

  await collectionStore.updateCollection({ id: id.value, data });
  router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''));
}
</script>
