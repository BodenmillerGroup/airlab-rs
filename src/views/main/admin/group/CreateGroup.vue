<script setup lang="ts">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { required } from '@/utils/validators';
import { useGroupStore } from '@/stores/group';
import type { CreateGroupDto } from '@/modules/group/types';

const router = useRouter();
const route = useRoute();
const groupStore = useGroupStore();

const form = ref();
const valid = ref(true);

const name = ref('');
const institution = ref('');
const url = ref<string | null>(null);
const isOpen = ref(false);

const nameRules = [required];

function reset() {
  name.value = '';
  institution.value = '';
  url.value = null;
  isOpen.value = false;
  form.value?.resetValidation();
}

function cancel() {
  router.back();
}

async function submit() {
  if (form.value?.validate()) {
    const params: CreateGroupDto = {
      name: name.value,
      institution: institution.value,
      url: url.value,
      isOpen: isOpen.value,
    };
    await groupStore.createGroup(params);
    router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''));
  }
}
</script>

<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Create Group</v-toolbar-title>
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
          <v-text-field label="Institution" v-model="institution" />
          <v-text-field label="URL" v-model="url" />
          <v-checkbox label="Public" v-model="isOpen" />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>
