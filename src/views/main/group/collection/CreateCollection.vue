<template>
  <v-container fluid>
    <v-toolbar density="compact">
      <v-toolbar-title>Create Collection</v-toolbar-title>
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
import { useGroupStore } from '@/stores/group';
import { useMainStore } from '@/stores/main';
import { required } from '@/utils/validators';

import type { CreateCollectionDto } from '@/modules/collection/types';

const router = useRouter();
const route = useRoute();

const collectionStore = useCollectionStore();
const groupStore = useGroupStore();
const mainStore = useMainStore();

const formRef = ref();
const valid = ref(true);

const name = ref('');
const description = ref('');

const requiredRules = [required];
const currentMemberId = computed(() => groupStore.myMember?.id ?? null);

function reset() {
  name.value = '';
  description.value = '';
  formRef.value?.resetValidation();
}

function cancel() {
  router.back();
}

async function submit() {
  const result = await formRef.value?.validate();
  if (!result?.valid) return;

  if (!currentMemberId.value) {
    mainStore.addNotification({ content: 'Could not resolve current group member', color: 'error' });
    return;
  }

  const data: CreateCollectionDto = {
    name: name.value,
    description: description.value.trim() || null,
    created_by: currentMemberId.value,
  };

  await collectionStore.createCollection(data);
  router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''));
}

onMounted(async () => {
  reset();
  const groupId = Number(route.params.groupId);
  if (!Number.isNaN(groupId)) {
    await groupStore.getMyMember(groupId);
  }
});
</script>
