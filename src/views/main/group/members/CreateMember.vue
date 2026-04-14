<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Create Group Member</v-toolbar-title>
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
          <v-autocomplete
            label="User"
            v-model="userId"
            :items="userOptions"
            item-text="name"
            item-value="id"
            :rules="userRules"
            dense
          />
          <div class="text-subtitle-1">Role</div>
          <v-btn-toggle v-model="role">
            <v-btn small value="100">Admin</v-btn>
            <v-btn small value="10">Standard</v-btn>
            <v-btn small value="0">Guest</v-btn>
          </v-btn-toggle>
          <v-checkbox label="Active" v-model="isActive" hint="Access is permited" />
          <v-row>
            <v-checkbox label="Can access all panels" v-model="allPanels" class="mr-4" />
          </v-row>
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { required } from '@/utils/validators';

import { useUserStore } from '@/stores/user';
import { useGroupStore } from '@/stores/group';
import { useMemberStore } from '@/stores/member';

import type { CreateMemberDto } from '@/modules/member/types';

const router = useRouter();
const route = useRoute();

const userStore = useUserStore();
const groupStore = useGroupStore();
const memberStore = useMemberStore();

// Form state
const valid = ref(true);
const form = ref();

const userId = ref<number | null>(null);
const role = ref('0');
const isActive = ref(false);
const allPanels = ref(false);

// Validators
const userRules = [required];

// Computed users list for v-autocomplete
const userOptions = computed(() =>
  userStore.users.map((item) => ({
    id: item.id,
    name: `${item.name} [${item.email}]`,
  }))
);

// Group ID
const activeGroupId = computed(() => groupStore.activeGroupId);

// Reset form fields
function reset() {
  userId.value = null;
  role.value = '0';
  isActive.value = false;
  allPanels.value = false;
  form.value?.resetValidation?.();
}

// Go back
function cancel() {
  router.back();
}

// Submit form
async function submit() {
  if (form.value?.validate?.() && activeGroupId.value) {
    const data: CreateMemberDto = {
      groupId: activeGroupId.value,
      userId: userId.value!,
      role: role.value ? Number(role.value) : 0,
      isActive: isActive.value,
      allPanels: allPanels.value,
    };
    await memberStore.createMember(data);
    router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''));
  }
}

// On component mount
onMounted(async () => {
  await userStore.getUsers();
});
</script>
