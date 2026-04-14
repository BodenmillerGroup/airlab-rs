<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Create Lot</v-toolbar-title>
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
              <!--<v-autocomplete
      v-model="selectedId"
      :items="staticItems"
      item-title="name"
      item-value="id"
      label="Test Clone"
      clearable
    />-->
          <!--<v-autocomplete
            v-model="cloneId"
            :items="clones"
            item-title="name"
            item-value="id"
            :loading="cloneLoading"
            label="CloneB"
            clearable
            :search="cloneSearch"
            @update:search="cloneDoSearch"
          />-->
          <v-autocomplete
            label="Clone"
            v-model="cloneId"
            :items="clones"
            item-title="name"
            item-value="id"
            dense
          />
          <v-autocomplete
            label="Protein"
            :model-value="selectedProteinId"
            :items="proteins"
            item-title="name"
            item-value="id"
            disabled
            dense
          />
          <v-autocomplete
            label="Provider"
            v-model="providerId"
            :items="providers"
            item-title="name"
            item-value="id"
            :rules="providerRules"
            dense
          />
          <v-autocomplete
            label="Collection"
            v-model="collectionId"
            :items="collections"
            item-title="name"
            item-value="id"
            clearable
            dense
          />
          <v-autocomplete
            label="Storage"
            v-model="storageId"
            :items="storages"
            item-title="name"
            item-value="id"
            clearable
            dense
          />
          <v-text-field label="Name" v-model="name" :rules="nameRules" />
          <v-text-field label="Catalog Number" v-model="reference" :rules="referenceRules" />
          <v-text-field label="Lot Number" v-model="number" :rules="numberRules" />
          <v-text-field label="URL" v-model="url" :rules="urlRules" />
          <v-text-field label="Purpose" v-model="purpose" :rules="purposeRules" />
          <v-text-field label="Price" v-model="price" />
          <v-text-field label="Note" v-model="note" />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { required } from '@/utils/validators';
import { useCloneStore } from '@/stores/clone';
import { useGroupStore } from '@/stores/group';
import { useLotStore } from '@/stores/lot';
import { useProviderStore } from '@/stores/provider';
import { useCollectionStore } from '@/stores/collection';
import { useProteinStore } from '@/stores/protein';
import { useStorageStore } from '@/stores/storage';
import type { CreateLotDto } from '@/modules/lot/types';

// 🧠 Stores
const cloneStore = useCloneStore();
const groupStore = useGroupStore();
const lotStore = useLotStore();
const providerStore = useProviderStore();
const collectionStore = useCollectionStore();
const proteinStore = useProteinStore();
const storageStore = useStorageStore();


// 🧭 Routing
const router = useRouter();
const route = useRoute();

// ✅ Form state
const form = ref();
const valid = ref(false);

const cloneId = ref<number | null>(null);
const providerId = ref<number | null>(null);
const storageId = ref<number | null>(null);
const collectionId = ref<number | null>(null);
const name = ref('');
const reference = ref('');
const number = ref('Pending');
const url = ref<string | null>(null);
const purpose = ref<string | null>(null);
const price = ref<string | null>(null);
const note = ref<string | null>(null);

const cloneRules = [required];
const providerRules = [required];
const nameRules = [required];
const referenceRules = [required];
const numberRules = [required];
const urlRules: any[] = [];
const purposeRules: any[] = [];

const activeGroupId = computed(() => groupStore.activeGroupId);
const providers = computed(() => providerStore.providers);
const storages = computed(() => storageStore.storages);
const clones = computed(() => cloneStore.clones);
const collections = computed(() => collectionStore.collections);
const proteins = computed(() => proteinStore.proteins);
const selectedClone = computed(() =>
  typeof cloneId.value === 'number' ? cloneStore.getClone(cloneId.value) : undefined
);
const selectedProteinId = computed<number | null>(() =>
  selectedClone.value?.proteinId ?? null
);

function reset() {
  cloneId.value = route.query.cloneId ? Number(route.query.cloneId) : null;
  name.value = '';
  reference.value = '';
  providerId.value = null;
  storageId.value = null;
  collectionId.value = null;
  number.value = 'Pending';
  url.value = null;
  purpose.value = null;
  price.value = null;
  note.value = null;
  form.value?.resetValidation?.();
}

function cancel() {
  router.back();
}

async function submit() {
  if (form.value?.validate() && activeGroupId.value) {
    const currentMemberId =
      groupStore.myMember?.groupId === activeGroupId.value ? groupStore.myMember.id : null;
    if (!currentMemberId) return;

    const payload: CreateLotDto = {
      createdBy: currentMemberId,
      groupId: activeGroupId.value,
      cloneId: Number(cloneId.value),
      providerId: Number(providerId.value),
      storageId: storageId.value,
      collectionId: collectionId.value,
      name: name.value,
      reference: reference.value,
      number: number.value,
      url: url.value,
      purpose: purpose.value,
      price: price.value,
      note: note.value,
    };
    await lotStore.createLot(payload);
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  reset();
  const groupId = +route.params.groupId;
  const requestedCloneId = route.query.cloneId ? Number(route.query.cloneId) : null;
  await Promise.all([
    groupStore.getMyMember(groupId),
    requestedCloneId ? cloneStore.getCloneById(requestedCloneId) : Promise.resolve(),
    proteinStore.fetchGroupProteins(groupId),
    providerStore.getGroupProviders(groupId),
    storageStore.getStorages(),
    collectionStore.getCollections(),
  ]);
});
</script>
