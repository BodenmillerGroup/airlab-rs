<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Create Protein</v-toolbar-title>
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
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useProteinStore } from '@/stores/protein';
import { useGroupStore } from '@/stores/group';
import { required } from '@/utils/validators';
import type { CreateProteinDto } from '@/modules/protein/types';

// Stores & router
const proteinStore = useProteinStore();
const groupStore = useGroupStore();
const router = useRouter();
const route = useRoute();

// Form data
const valid = ref(true);
const form = ref<InstanceType<typeof HTMLFormElement> | null>(null);
const name = ref('');
const description = ref('');
const nameRules = [required];

// Actions
const reset = () => {
  name.value = '';
  description.value = '';
  form.value?.resetValidation?.();
};

const cancel = () => {
  router.back();
};

const submit = async () => {
  const isValid = form.value?.validate?.();
  if (!isValid || !groupStore.activeGroupId) return;
  if (groupStore.myMember?.groupId !== groupStore.activeGroupId) {
    await groupStore.getMyMember(groupStore.activeGroupId);
  }
  const currentMemberId =
    groupStore.myMember?.groupId === groupStore.activeGroupId ? groupStore.myMember.id : null;
  if (!currentMemberId) return;

  const payload: CreateProteinDto = {
    createdBy: currentMemberId,
    groupId: groupStore.activeGroupId,
    name: name.value,
    description: description.value,
  };

  await proteinStore.createProtein(payload);
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
};

if (route.params.groupId && groupStore.myMember?.groupId !== Number(route.params.groupId)) {
  void groupStore.getMyMember(Number(route.params.groupId));
}
</script>
