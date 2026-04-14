<template>
  <v-container fluid>
    <v-card class="ma-4 pa-4">
      <v-card-title primary-title>
        <div class="text-h5 primary--text">Edit Group Member</div>
      </v-card-title>
      <v-card-text>
        <v-form v-model="valid" ref="form" lazy-validation>
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
      <v-card-actions>
        <v-spacer />
        <v-btn @click="cancel">Cancel</v-btn>
        <v-btn @click="reset">Reset</v-btn>
        <v-btn @click="submit" :disabled="!valid">Save</v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useMemberStore } from '@/stores/member';
import type { UpdateMemberDto } from '@/modules/member/types';

const router = useRouter();
const route = useRoute();
const memberStore = useMemberStore();

const form = ref();
const valid = ref(true);

const role = ref('0');
const isActive = ref(false);
const allPanels = ref(false);

const memberId = computed(() => +route.params.id);
const member = computed(() => memberStore.getMemberById(memberId.value));

async function reset() {
  form.value?.resetValidation?.();

  if (member.value) {
    role.value = member.value.role.toString();
    isActive.value = member.value.isActive;
    allPanels.value = member.value.allPanels;
  }
}

function cancel() {
  router.back();
}

async function submit() {
  if (form.value?.validate?.() && member.value) {
    const data: UpdateMemberDto = {
      role: role.value ? Number(role.value) : 0,
      isActive: isActive.value,
      allPanels: allPanels.value,
    };

    await memberStore.updateMember({ id: member.value.id, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  await memberStore.getMember(memberId.value);
  reset();
});
</script>
