<template>
  <v-container fluid>
    <v-toolbar density="compact">
      <v-toolbar-title>Edit Storage</v-toolbar-title>
      <v-spacer />
      <v-btn text @click="cancel" color="primary">Cancel</v-btn>
      <v-btn text @click="reset" color="primary">Reset</v-btn>
      <v-btn text @click="submit" :disabled="!valid" color="primary">Save</v-btn>
    </v-toolbar>

    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form ref="formRef" v-model="valid" validate-on="lazy">
          <v-text-field v-model="name" label="Name" :rules="requiredRules" />
          <v-text-field v-model="storageType" label="Type" :rules="requiredRules" />
          <v-text-field v-model="location" label="Location" :rules="requiredRules" />
          <v-text-field
            v-model="temperature_c"
            label="Temperature (C)"
            type="number"
            :rules="temperatureRules"
          />
          <v-switch v-model="active" label="Active" color="primary" />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { useStorageStore } from '@/stores/storage';
import { required } from '@/utils/validators';

import type { UpdateStorageDto } from '@/modules/storage/types';

const route = useRoute();
const router = useRouter();
const storageStore = useStorageStore();

const id = computed(() => Number(route.params.id));
const storage = computed(() => storageStore.getStorage(id.value));

const formRef = ref();
const valid = ref(true);

const name = ref('');
const storageType = ref('');
const location = ref('');
const temperature_c = ref<string>('');
const active = ref(true);

const requiredRules = [required];
const temperatureRules = [
  (value: string | number) => `${value}`.trim() !== '' || 'Required',
  (value: string | number) => !Number.isNaN(Number(value)) || 'Must be a number',
];

onMounted(async () => {
  await storageStore.getStorageById(id.value);
  reset();
});

function reset() {
  name.value = storage.value?.name ?? '';
  storageType.value = storage.value?.type ?? '';
  location.value = storage.value?.location ?? '';
  temperature_c.value = storage.value ? String(storage.value.temperature_c) : '';
  active.value = storage.value?.active ?? true;
  formRef.value?.resetValidation();
}

function cancel() {
  router.back();
}

async function submit() {
  const result = await formRef.value?.validate();
  if (!result?.valid) return;

  const data: UpdateStorageDto = {
    name: name.value,
    type: storageType.value,
    location: location.value,
    temperature_c: Number(temperature_c.value),
    active: active.value,
  };

  await storageStore.updateStorage({ id: id.value, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
}
</script>
