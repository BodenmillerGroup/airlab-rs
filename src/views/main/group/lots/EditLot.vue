<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Edit Lot</v-toolbar-title>
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
            label="Clone"
            v-model="cloneId"
            :items="clones"
            item-title="name"
            item-value="id"
            :rules="cloneRules"
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
import { useRoute, useRouter } from 'vue-router';
import { required } from '@/utils/validators';
import { useCloneStore } from '@/stores/clone';
import { useGroupStore } from '@/stores/group';
import { useLotStore } from '@/stores/lot';
import { useProviderStore } from '@/stores/provider';
import { useCollectionStore } from '@/stores/collection';
import { useStorageStore } from '@/stores/storage';
import type { UpdateLotDto } from '@/modules/lot/types';

const route = useRoute();
const router = useRouter();

const groupStore = useGroupStore();
const lotStore = useLotStore();
const cloneStore = useCloneStore();
const providerStore = useProviderStore();
const collectionStore = useCollectionStore();
const storageStore = useStorageStore();

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
const nameRules = [required];
const referenceRules = [required];
const providerRules = [required];
const numberRules = [required];
const urlRules: any[] = [];
const purposeRules: any[] = [];

const activeGroupId = computed(() => groupStore.activeGroupId);
const clones = computed(() => cloneStore.clones);
const providers = computed(() => providerStore.providers);
const storages = computed(() => storageStore.storages);
const collections = computed(() => collectionStore.collections);
const lot = computed(() => lotStore.getLot(Number(route.params.id)));

function cancel() {
  router.back();
}

function reset() {
  form.value?.resetValidation();
  if (lot.value) {
    cloneId.value = lot.value.cloneId;
    name.value = lot.value.name;
    reference.value = lot.value.reference;
    providerId.value = lot.value.providerId;
    storageId.value = lot.value.storageId ?? null;
    collectionId.value = lot.value.collectionId ?? null;
    number.value = lot.value.number;
    url.value = lot.value.url;
    purpose.value = lot.value.purpose;
    price.value = lot.value.price;
    note.value = lot.value.note;
  }
}

async function submit() {
  if (form.value?.validate() && lot.value) {
    const data: UpdateLotDto = {
      cloneId: cloneId.value!,
      name: name.value,
      reference: reference.value,
      providerId: providerId.value!,
      storageId: storageId.value,
      collectionId: collectionId.value,
      number: number.value,
      url: url.value,
      purpose: purpose.value,
      price: price.value,
      note: note.value,
    };
    await lotStore.updateLot(lot.value.id, data);
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  const id = Number(route.params.id);
  const groupId = Number(route.params.groupId);
  await Promise.all([
    lotStore.getLot(id),
    cloneStore.getGroupClones(groupId),
    providerStore.getGroupProviders(groupId),
    storageStore.getStorages(),
    collectionStore.getCollections(),
  ]);
  reset();
});
</script>
