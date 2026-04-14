<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useLotStore } from '@/stores/lot';
import { useCloneStore } from '@/stores/clone';
import { useProteinStore } from '@/stores/protein';
import { useTagStore } from '@/stores/tag';
import { useGroupStore } from '@/stores/group';
import { useMainStore } from '@/stores/main';
import { useMemberStore } from '@/stores/member';
import { useUserStore } from '@/stores/user';
import { useConjugateStore } from '@/stores/conjugate';
import { useStorageStore } from '@/stores/storage';
import { required } from '@/utils/validators';
import type { CreateConjugateDto } from '@/modules/conjugate/types';

const router = useRouter();
const route = useRoute();

const lotStore = useLotStore();
const cloneStore = useCloneStore();
const proteinStore = useProteinStore();
const tagStore = useTagStore();
const groupStore = useGroupStore();
const mainStore = useMainStore();
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
const selectedLot = computed(() =>
  typeof lotId.value === 'number' ? lotStore.getLot(lotId.value) : undefined
);
const selectedClone = computed(() =>
  selectedLot.value ? cloneStore.getClone(selectedLot.value.cloneId) : undefined
);
const selectedProtein = computed(() =>
  selectedClone.value ? proteinStore.getProtein(selectedClone.value.proteinId) : undefined
);
function formatPersonName(fullName: string | undefined, fallback: string) {
  const trimmed = fullName?.trim();
  if (!trimmed) return fallback;
  const parts = trimmed.split(/\s+/);
  if (parts.length === 1) return parts[0];
  return `${parts[0]} ${parts[parts.length - 1]}`;
}

const activeGroupMembers = computed(() =>
  activeGroupId.value ? memberStore.getMembersForGroup(activeGroupId.value) : []
);

const memberOptions = computed(() =>
  activeGroupMembers.value.map((member) => ({
    id: member.id,
    name: formatPersonName(userStore.getUserById(member.userId)?.name, `Member ${member.id}`),
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
const currentMemberId = computed(() => {
  if (groupStore.myMember?.id) return groupStore.myMember.id;

  const userId = mainStore.userProfile?.id;
  if (!userId) return null;

  return memberStore.members.find((m) => m.userId === userId)?.id ?? null;
});

function cancel() {
  router.back();
}

function reset() {
  lotId.value = route.query.lotId ? Number(route.query.lotId) : route.params.lotId ? +route.params.lotId : null;
  tagId.value = null;
  storageId.value = null;
  labeledBy.value = null;
  concentration.value = null;
  description.value = null;
  customId.value = null;
  form.value?.resetValidation();
}

async function submit() {
  if (form.value?.validate() && activeGroupId.value) {
    if (!currentMemberId.value) {
      mainStore.addNotification({ content: 'Could not resolve current group member', color: 'error' });
      return;
    }

    const data: CreateConjugateDto = {
      createdBy: currentMemberId.value,
      groupId: activeGroupId.value,
      lotId: Number(lotId.value),
      tagId: Number(tagId.value),
      storageId: storageId.value ?? null,
      concentration: concentration.value ?? null,
      description: description.value,
      customId: customId.value,
      labeledBy: labeledBy.value ?? null,
    };
    await conjugateStore.createConjugate(data);
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  reset();
  const groupId = +route.params.groupId;
  await Promise.all([
    lotStore.getGroupLots(groupId),
    cloneStore.getGroupClones(groupId),
    proteinStore.fetchGroupProteins(groupId),
    tagStore.getGroupTags(groupId),
    groupStore.getMyMember(groupId),
    memberStore.getGroupMembers(groupId),
    storageStore.getStorages(),
  ]);
  await userStore.fetchByIds(memberStore.members.map((m) => m.userId));
});
</script>

<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Create Conjugate</v-toolbar-title>
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
            label="Lot"
            v-model="lotId"
            :items="lots"
            item-title="name"
            item-value="id"
            :rules="lotRules"
            dense
          />
          <v-text-field
            label="Clone"
            :model-value="selectedClone?.name ?? ''"
            readonly
          />
          <v-text-field
            label="Protein"
            :model-value="selectedProtein?.name ?? ''"
            readonly
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
