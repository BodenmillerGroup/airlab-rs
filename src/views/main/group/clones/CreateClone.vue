<template>
  <v-container fluid>
    <v-toolbar density="compact">
      <v-toolbar-title>Create Clone</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn @click="cancel" variant="text" color="primary">Cancel</v-btn>
        <v-btn @click="reset" variant="text" color="primary">Reset</v-btn>
        <v-btn @click="submit" variant="text" color="primary">Save</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form v-model="valid" ref="form" validate-on="submit lazy">
          <v-text-field label="Clone Name" v-model="name" :rules="nameRules" />

          <v-autocomplete
            label="Protein"
            v-model="proteinId"
            :items="proteins"
            item-title="name"
            item-value="id"
            :rules="proteinRules"
            density="compact"
          />

          <v-autocomplete
            label="Host"
            v-model="speciesId"
            :items="species"
            item-title="name"
            item-value="id"
            :rules="hostRules"
            density="compact"
          />

          <v-text-field label="Epitope" v-model="epitope" />
          <v-text-field label="Isotype" v-model="isotype" />

          <v-checkbox label="Polyclonal" v-model="isPolyclonal" />
          <v-checkbox label="Phosphoantibody" v-model="isPhospho" />

          <v-row>
            <v-col cols="4">
              <div class="text-subtitle-1">Reactivity</div>
              <v-chip-group v-model="reactivity" multiple column class="reactivity-group">
                <v-chip
                  v-for="item in species"
                  :key="item.id"
                  :value="item.id"
                  class="reactivity-chip"
                  size="small"
                  label
                >
                  {{ item.name }}
                </v-chip>
              </v-chip-group>
            </v-col>

            <v-col />
            <v-col cols="7">
              <div class="text-subtitle-1">Application</div>
              <v-row v-for="app in appOptions" :key="app.key">
                <v-col>
                  <div class="subtitle-3">{{ app.label }}</div>
                  <v-btn-toggle v-model="app.model.value" class="application-toggle">
                    <v-btn size="small" :value="'true'" class="true">Yes</v-btn>
                    <v-btn size="small" :value="'false'" class="false">No</v-btn>
                  </v-btn-toggle>
                </v-col>
              </v-row>
            </v-col>
          </v-row>
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useCloneStore } from '@/stores/clone';
import { useGroupStore } from '@/stores/group';
import { useProteinStore } from '@/stores/protein';
import { useSpeciesStore } from '@/stores/species';
import { applicationNameToId } from '@/utils/enums';
import { required } from '@/utils/validators';
import type { CreateCloneDto } from '@/modules/clone/types';
import { storeToRefs } from 'pinia'


const router = useRouter();
const route = useRoute();

const cloneStore = useCloneStore();
const groupStore = useGroupStore();
const proteinStore = useProteinStore();
const speciesStore = useSpeciesStore();

// ✅ Form refs and state
const form = ref();
const valid = ref(false);

const name = ref('');
const proteinId = ref<number | null>(null);
const epitope = ref('');
const isotype = ref('');
const isPolyclonal = ref(false);
const isPhospho = ref(false);
const speciesId = ref<number | null>(null);
const reactivity = ref<number[]>([]);

// ✅ Application toggles
const smcApplication = ref<string | null>(null);
const imcApplication = ref<string | null>(null);
const fcApplication = ref<string | null>(null);
const ifApplication = ref<string | null>(null);
const ihcApplication = ref<string | null>(null);
const ihcfApplication = ref<string | null>(null);
const wbApplication = ref<string | null>(null);

// ✅ Validation
const nameRules = [(v: string) => !!v || 'Name is required'];
const proteinRules = [required];
const hostRules = [required];

// ✅ Computed data
const { activeGroupId } = storeToRefs(groupStore);

const proteins = computed(() => proteinStore.proteins);
const species = computed(() => speciesStore.species);

const appOptions = [
  { key: 'sMC', label: 'SMC', model: smcApplication },
  { key: 'iMC', label: 'IMC', model: imcApplication },
  { key: 'FC', label: 'FC', model: fcApplication },
  { key: 'IF', label: 'IF', model: ifApplication },
  { key: 'IHC', label: 'IHC', model: ihcApplication },
  { key: 'IHCF', label: 'IHC-F', model: ihcfApplication },
  { key: 'WB', label: 'WB', model: wbApplication },
];

// ✅ Form actions
function cancel() {
  router.back();
}

function reset() {
  name.value = '';
  proteinId.value = route.params.proteinId ? +route.params.proteinId : null;
  epitope.value = '';
  isotype.value = '';
  isPolyclonal.value = false;
  isPhospho.value = false;
  speciesId.value = null;
  reactivity.value = [];

  smcApplication.value = null;
  imcApplication.value = null;
  fcApplication.value = null;
  ifApplication.value = null;
  ihcApplication.value = null;
  ihcfApplication.value = null;
  wbApplication.value = null;

  form.value?.resetValidation();
}

async function submit() {
  if (form.value?.validate() && activeGroupId.value) {
    const currentMemberId =
      groupStore.myMember?.groupId === activeGroupId.value ? groupStore.myMember.id : null;
    if (!currentMemberId) return;

    const application: Record<number, boolean> = {};

    appOptions.forEach(({ key, model }) => {
      if (model.value !== null) {
        application[applicationNameToId[key]] = model.value === 'true';
      }
    });

    const data: CreateCloneDto = {
      createdBy: currentMemberId,
      groupId: activeGroupId.value,
      name: name.value,
      proteinId: Number(proteinId.value),
      epitope: epitope.value,
      isotype: isotype.value,
      isPhospho: isPhospho.value,
      isPolyclonal: isPolyclonal.value,
      speciesId: Number(speciesId.value),
      reactivity: reactivity.value,
      application,
    };

    await cloneStore.createClone(data);
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  reset();
  const groupId = Number(route.params.groupId);
  await Promise.all([
    groupStore.getMyMember(groupId),
    proteinStore.fetchGroupProteins(groupId),
    //proteinStore.getGroupProteins(groupId),
    speciesStore.getGroupSpecies(groupId),
  ]);
});
</script>

<style scoped>
.reactivity-group :deep(.reactivity-chip) {
  background: #e5e7eb !important;
  color: #374151 !important;
}

.reactivity-group :deep(.reactivity-chip.v-chip--selected) {
  background: #96ff96 !important;
  color: #14532d !important;
}

.application-toggle :deep(.v-btn) {
  background: #d1d5db !important;
  color: #374151 !important;
}

.application-toggle :deep(.true.v-btn--active) {
  background: #96ff96 !important;
  color: #14532d !important;
}

.application-toggle :deep(.false.v-btn--active) {
  background: #fca5a5 !important;
  color: #7f1d1d !important;
}
</style>
