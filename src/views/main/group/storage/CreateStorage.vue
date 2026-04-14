<template>
  <v-container fluid>
    <v-toolbar density="compact">
      <v-toolbar-title>Create Storage</v-toolbar-title>
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
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { useStorageStore } from '@/stores/storage';
import { required } from '@/utils/validators';

import type { CreateStorageDto } from '@/modules/storage/types';

const router = useRouter();
const route = useRoute();
const storageStore = useStorageStore();

const formRef = ref();
const valid = ref(true);

const name = ref('');
const storageType = ref('');
const location = ref('');
const temperature_c = ref<string>('4');
const active = ref(true);

const requiredRules = [required];
const temperatureRules = [
  (value: string | number) => `${value}`.trim() !== '' || 'Required',
  (value: string | number) => !Number.isNaN(Number(value)) || 'Must be a number',
];

function reset() {
  name.value = '';
  storageType.value = '';
  location.value = '';
  temperature_c.value = '4';
  active.value = true;
  formRef.value?.resetValidation();
}

function cancel() {
  router.back();
}

async function submit() {
  const result = await formRef.value?.validate();
  if (!result?.valid) return;

  const data: CreateStorageDto = {
    name: name.value,
    type: storageType.value,
    location: location.value,
    temperature_c: Number(temperature_c.value),
    active: active.value,
  };

  await storageStore.createStorage(data);
  router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''));
}
</script>
