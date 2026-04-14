<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { required } from '@/utils/validators';
import { useGroupStore } from '@/stores/group';
import type { UpdateGroupDto } from '@/modules/group/types';

const route = useRoute();
const router = useRouter();
const groupStore = useGroupStore();

const form = ref();
const valid = ref(true);

const name = ref('');
const institution = ref('');
const url = ref('');
const isOpen = ref(false);

const nameRules = [required];

const group = computed(() => groupStore.getGroupById(Number(route.params.id)) ?? null)

function cancel() {
  router.back();
}

function reset() {
  name.value = '';
  institution.value = '';
  url.value = '';
  isOpen.value = false;
  form.value?.resetValidation();
  if (group.value) {
    name.value = group.value.name;
    institution.value = group.value.institution;
    url.value = group.value.url;
    isOpen.value = group.value.isOpen;
  }
}

async function submit() {
  if (form.value?.validate() && group.value) {
    const data: UpdateGroupDto = {
      name: name.value,
      institution: institution.value,
      url: url.value,
      isOpen: isOpen.value,
    };
    await groupStore.updateGroup({ id: group.value.id, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  await groupStore.getGroup(Number(route.params.id));
  reset();
});
</script>

<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Edit Group</v-toolbar-title>
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
