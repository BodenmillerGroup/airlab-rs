<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useGroupStore } from '@/stores/group';
import { useLotStore } from '@/stores/lot';
import { useTagStore } from '@/stores/tag';
import { useMemberStore } from '@/stores/member';
import { useUserStore } from '@/stores/user';
import { useConjugateStore } from '@/stores/conjugate';
import { useStorageStore } from '@/stores/storage';
import { required } from '@/utils/validators';
import type { UpdateConjugateDto } from '@/modules/conjugate/types';

const route = useRoute();
const router = useRouter();

const groupStore = useGroupStore();
const lotStore = useLotStore();
const tagStore = useTagStore();
const memberStore = useMemberStore();
const userStore = useUserStore();
const conjugateStore = useConjugateStore();
const storageStore = useStorageStore();

const valid = ref(false);
const form = ref();

const lotId = ref<number | null>(null);
const tagId = ref<number | null>(null);
const storageId = ref<number | null>(null);
const labeledBy = ref<number | null>(null);
const concentration = ref<number | null>(null);
const description = ref<string | null>(null);
const customId = ref<string | null>(null);

const lotRules = [required];
const tagRules = [required];
const concentrationRules = [required];

const lots = computed(() => lotStore.lots);
function formatMemberDisplayName(fullName: string | undefined, memberId: number) {
  const trimmed = fullName?.trim();
  if (!trimmed) return `Member ${memberId}`;

  const parts = trimmed.split(/\s+/);
  if (parts.length === 1) return parts[0];

  return `${parts[0]} ${parts[parts.length - 1]}`;
}

const memberOptions = computed(() =>
  memberStore.members.map((member) => ({
    id: member.id,
    name: formatMemberDisplayName(userStore.getUserById(member.userId)?.name, member.id),
  }))
);
const storages = computed(() => storageStore.storages);
const tags = computed(() =>
  tagStore.tags.map((item) => ({
    id: item.id,
    name: item.mw ? item.name + item.mw : item.name,
  }))
);
const activeGroupId = computed(() => groupStore.activeGroupId);

const conjugate = computed(() =>
  conjugateStore.getConjugate(Number(route.params.id))
);

function cancel() {
  router.back();
}

function reset() {
  if (form.value) {
    form.value.resetValidation();
  }
  if (conjugate.value) {
    lotId.value = conjugate.value.lotId;
    tagId.value = conjugate.value.tagId;
    storageId.value = conjugate.value.storageId;
    labeledBy.value = conjugate.value.labeledBy;
    concentration.value = conjugate.value.concentration;
    description.value = conjugate.value.description;
    customId.value = conjugate.value.customId;
  }
}

async function submit() {
  if (form.value?.validate() && conjugate.value) {
    const data: UpdateConjugateDto = {
      lotId: Number(lotId.value),
      tagId: Number(tagId.value),
      storageId: storageId.value ?? null,
      concentration: concentration.value,
      description: description.value,
      customId: customId.value,
      labeledBy: labeledBy.value ?? null,
    };
    await conjugateStore.updateConjugate({ id: conjugate.value.id, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  const id = Number(route.params.id);
  const groupId = Number(route.params.groupId);
  await Promise.all([
    conjugateStore.getConjugate(id),
    lotStore.getGroupLots(groupId),
    tagStore.getGroupTags(groupId),
    memberStore.getGroupMembers(groupId),
    storageStore.getStorages(),
  ]);
  await userStore.fetchByIds(memberStore.members.map((m) => m.userId));
  reset();
});
</script>

<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Edit Conjugate</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn @click="cancel" text color="primary">Cancel</v-btn>
        <v-btn @click="reset" text color="primary">Reset</v-btn>
        <v-btn @click="submit" text :disabled="!valid" color="primary">Save</v-btn>
      </v-toolbar-items>
    </v-toolbar>
    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form v-model="valid" ref="form">
          <v-autocomplete
            label="Lot"
            v-model="lotId"
            :items="lots"
            item-title="name"
            item-value="id"
            :rules="lotRules"
            dense
          />
          <v-autocomplete
            label="Tag"
            v-model="tagId"
            :items="tags"
            item-title="name"
            item-value="id"
            :rules="tagRules"
            dense
          />
          <v-autocomplete
            label="Storage"
            v-model="storageId"
            :items="storages"
            item-title="name"
            item-value="id"
            dense
            clearable
            open-on-clear
          />
          <v-text-field
            label="Concentration (in µg/mL)"
            v-model.number="concentration"
            :rules="concentrationRules"
            type="number"
          />
          <v-autocomplete
            label="Labeled by"
            v-model="labeledBy"
            :items="memberOptions"
            item-title="name"
            item-value="id"
            dense
            clearable
            open-on-clear
          />
          <v-text-field label="Description" v-model="description" />
          <v-text-field label="Custom ID" v-model="customId" />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>
