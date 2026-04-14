<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Create Validation</v-toolbar-title>
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
          <v-row>
            <v-col>
              <div class="text-subtitle-1">Application</div>
              <v-btn-toggle v-model="application" mandatory>
                <v-btn small value="0">SMC</v-btn>
                <v-btn small value="1">IMC</v-btn>
                <v-btn small value="2">FC</v-btn>
                <v-btn small value="3">IF</v-btn>
                <v-btn small value="4">IHC</v-btn>
                <v-btn small value="5">IHCF</v-btn>
                <v-btn small value="6">WB</v-btn>
              </v-btn-toggle>
            </v-col>
            <v-col>
              <div class="text-subtitle-1">Status</div>
              <v-btn-toggle v-model="status" mandatory>
                <v-btn small value="0">Yes</v-btn>
                <v-btn small value="1">So-So</v-btn>
                <v-btn small value="2">No</v-btn>
                <v-btn small value="3">Undefined</v-btn>
              </v-btn-toggle>
            </v-col>
          </v-row>

          <v-row>
            <v-col>
              <v-autocomplete
                label="Protein"
                v-model="proteinId"
                :items="proteins"
                item-title="name"
                item-value="id"
                :rules="requiredRules"
                clearable
                open-on-clear
                dense
                @update:model-value="handleProteinChange"
              />
            </v-col>
            <v-col>
              <v-autocomplete
                label="Clone"
                v-model="cloneId"
                :items="clones"
                item-title="name"
                item-value="id"
                :rules="requiredRules"
                :disabled="!proteinId"
                clearable
                open-on-clear
                dense
                @update:model-value="handleCloneChange"
              />
            </v-col>
            <v-col>
              <v-autocomplete
                label="Lot"
                v-model="lotId"
                v-model:search-input="lotSearchInput"
                :items="lots"
                item-title="displayName"
                item-value="id"
                :rules="requiredRules"
                :disabled="!cloneId"
                clearable
                open-on-clear
                dense
                @update:model-value="handleLotChange"
              />
            </v-col>
            <v-col>
              <v-autocomplete
                label="Conjugate"
                v-model="conjugateId"
                :items="conjugates"
                :filter="filterConjugates"
                item-title="tubeNumber"
                item-value="id"
                :disabled="!lotId"
                clearable
                open-on-clear
                dense
              />
            </v-col>
          </v-row>

          <v-autocomplete
            label="Species"
            v-model="speciesId"
            :items="species"
            item-title="name"
            item-value="id"
            clearable
            open-on-clear
            dense
          />

          <v-row>
            <v-col>
              <v-text-field label="Positive control" v-model="positiveControl" :rules="requiredRules" />
            </v-col>
            <v-col>
              <v-text-field label="Negative control" v-model="negativeControl" :rules="requiredRules" />
            </v-col>
          </v-row>

          <v-text-field label="Incubation conditions" v-model="incubationConditions" hint="e.g. Overnight, 4°C" />
          <v-text-field label="Tissue" v-model="tissue" hint="e.g. Liver, Lymph node" />

          <v-row>
            <v-col>
              <v-text-field
                label="Concentration"
                v-model="concentration"
                dense
                hint="e.g. 1:100 if dilution or 0.2 if ug/mL"
              />
            </v-col>
            <v-col>
              <v-select
                label="Units"
                v-model="concentrationUnit"
                :items="['dilution', 'ug/mL']"
                clearable
                open-on-clear
                dense
              />
            </v-col>
          </v-row>

          <v-row>
            <v-col>
              <v-select
                label="Fixation"
                v-model="fixation"
                :items="fixations"
                item-title="name"
                item-value="id"
                clearable
                open-on-clear
                dense
              />
            </v-col>
            <v-col>
              <v-text-field label="Fixation notes" v-model="fixationNotes" dense />
            </v-col>
          </v-row>

          <v-text-field label="Notes" v-model="notes" />

          <v-row v-if="application === '1' || application === '3' || application === '4'">
            <v-col>
              <v-select
                label="Protocol"
                v-model="antigenRetrievalType"
                :items="availableARtypes"
                clearable
                open-on-clear
                dense
              />
            </v-col>
            <v-col>
              <v-text-field label="Time" v-model="antigenRetrievalTime" dense hint="minutes" />
            </v-col>
            <v-col>
              <v-text-field label="Temperature" v-model="antigenRetrievalTemperature" dense hint="°C" />
            </v-col>
          </v-row>

          <div v-else>
            <v-row>
              <v-col cols="2">Saponin</v-col>
              <v-col cols="3">
                <v-btn-toggle v-model="saponin" mandatory>
                  <v-btn small value="true">Yes</v-btn>
                  <v-btn small value="false">No</v-btn>
                  <v-btn small value="null">Not tested</v-btn>
                </v-btn-toggle>
              </v-col>
              <v-col>
                <v-text-field label="Concentration" v-model="saponinConcentration" dense hint="ug/mL" />
              </v-col>
            </v-row>

            <v-row>
              <v-col cols="2">Methanol treatment</v-col>
              <v-col cols="3">
                <v-btn-toggle v-model="methanolTreatment" mandatory>
                  <v-btn small value="true">Yes</v-btn>
                  <v-btn small value="false">No</v-btn>
                  <v-btn small value="null">Not tested</v-btn>
                </v-btn-toggle>
              </v-col>
              <v-col>
                <v-text-field label="Concentration" v-model="methanolTreatmentConcentration" dense hint="ug/mL" />
              </v-col>
            </v-row>

            <v-row>
              <v-col cols="2">Surface staining</v-col>
              <v-col cols="3">
                <v-btn-toggle v-model="surfaceStaining" mandatory>
                  <v-btn small value="true">Yes</v-btn>
                  <v-btn small value="false">No</v-btn>
                  <v-btn small value="null">Not tested</v-btn>
                </v-btn-toggle>
              </v-col>
              <v-col>
                <v-text-field label="Concentration" v-model="surfaceStainingConcentration" dense hint="ug/mL" />
              </v-col>
            </v-row>
          </div>
        </v-form>
      </v-card-text>
    </v-card>

    <v-card class="mt-4 px-4">
      <v-card-title primary-title>
        <div class="text-h5">Validation File</div>
      </v-card-title>
      <v-card-text>
        <v-form>
          <v-file-input v-model="file" label="File upload" show-size />
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
import { useConjugateStore } from '@/stores/conjugate';
import { useGroupStore } from '@/stores/group';
import { useLotStore } from '@/stores/lot';
import { useProteinStore } from '@/stores/protein';
import { useSpeciesStore } from '@/stores/species';
import { useValidationStore } from '@/stores/validation';
import { antigenRetrievalTypes } from '@/utils/enums';
import type { CreateValidationDto } from '@/modules/validation/types';

// Routing
const route = useRoute();
const router = useRouter();

// Stores
const groupStore = useGroupStore();
const validationStore = useValidationStore();
const cloneStore = useCloneStore();
const lotStore = useLotStore();
const conjugateStore = useConjugateStore();
const speciesStore = useSpeciesStore();
const proteinStore = useProteinStore();

// Refs & State
const form = ref();
const valid = ref(false);
const file = ref<File | File[] | null>(null);

const proteinId = ref<number | null>(null);
const cloneId = ref<number | null>(null);
const lotId = ref<number | null>(null);
const conjugateId = ref<number | null>(null);
const speciesId = ref<number | null>(null);
const application = ref('1');
const status = ref('3');
const lotSearchInput = ref('');
const conjugateSearchInput = ref('');

const positiveControl = ref<string | null>(null);
const negativeControl = ref<string | null>(null);
const incubationConditions = ref<string | null>(null);
const concentration = ref<string | null>(null);
const concentrationUnit = ref<string | null>(null);
const tissue = ref<string | null>(null);
const fixation = ref<number | null>(null);
const fixationNotes = ref<string | null>(null);
const notes = ref<string | null>(null);
const antigenRetrievalType = ref<string | null>(null);
const antigenRetrievalTime = ref<string | null>(null);
const antigenRetrievalTemperature = ref<string | null>(null);

const saponin = ref('null');
const saponinConcentration = ref<string | null>(null);
const methanolTreatment = ref('null');
const methanolTreatmentConcentration = ref<string | null>(null);
const surfaceStaining = ref('null');
const surfaceStainingConcentration = ref<string | null>(null);

// Fixation + AR types
const fixations = [
  { id: 0, name: 'FFPE' },
  { id: 1, name: 'OCT' },
  { id: 2, name: 'Bouin' },
  { id: 3, name: 'Zinc' },
  { id: 4, name: 'Methanol' },
  { id: 5, name: "Saccomano's" },
  { id: 6, name: 'Methanol/Acetone' },
  { id: 7, name: 'Other' },
];
const availableARtypes = antigenRetrievalTypes;

// Computed Values
const activeGroupId = computed(() => groupStore.activeGroupId);
const proteins = computed(() => {
  const cloneProteinIds = new Set(cloneStore.clones.map((clone) => clone.proteinId));
  return proteinStore.proteins.filter((protein) => cloneProteinIds.has(protein.id));
});
const clones = computed(() =>
  typeof proteinId.value === 'number'
    ? cloneStore.clones.filter((clone) => clone.proteinId === proteinId.value)
    : []
);
const species = computed(() => speciesStore.species);
const selectedClone = computed(() =>
  typeof cloneId.value === 'number' ? cloneStore.getClone(cloneId.value) : undefined
);
const selectedProtein = computed(() =>
  typeof proteinId.value === 'number' ? proteinStore.getProtein(proteinId.value) : undefined
);

const lots = computed(() => {
  return lotStore.lots
    .filter((lot) => lot.cloneId === cloneId.value)
    .map((lot) => ({
      ...lot,
      displayName: lot.number ? `${lot.number} · ${lot.name}` : lot.name,
    }));
});

const conjugates = computed(() => {
  return conjugateStore.conjugates.filter((conjugate) => conjugate.lotId === lotId.value);
});

// Rules
const requiredRules = [required];

// Helpers
function resetLotId() {
  lotId.value = null;
  lotSearchInput.value = '';
  conjugateId.value = null;
  conjugateSearchInput.value = '';
}

function resetConjugateId() {
  conjugateId.value = null;
  conjugateSearchInput.value = '';
}

function handleProteinChange(value: number | null) {
  proteinId.value = value;
  cloneId.value = null;
  resetLotId();
}

function handleCloneChange(value: number | null) {
  cloneId.value = value;
  resetLotId();
}

function handleLotChange(value: number | null) {
  lotId.value = value;
  resetConjugateId();
}

function filterConjugates(item: any, query: string, itemText: string) {
  return itemText.toLowerCase().includes(query.toLowerCase());
}

// Navigation
function cancel() {
  router.back();
}

// Reset Form
function reset() {
  const requestedCloneId = route.query.cloneId ? Number(route.query.cloneId) : route.params.cloneId ? +route.params.cloneId : null;
  const requestedLotId = route.query.lotId ? Number(route.query.lotId) : route.params.lotId ? +route.params.lotId : null;
  const requestedConjugateId = route.query.conjugateId ? Number(route.query.conjugateId) : route.params.conjugateId ? +route.params.conjugateId : null;
  const requestedClone = typeof requestedCloneId === 'number' ? cloneStore.getClone(requestedCloneId) : undefined;
  const requestedLot = typeof requestedLotId === 'number' ? lotStore.getLot(requestedLotId) : undefined;
  const requestedConjugate = typeof requestedConjugateId === 'number'
    ? conjugateStore.getConjugate(requestedConjugateId)
    : undefined;

  proteinId.value = requestedClone?.proteinId ?? null;
  cloneId.value = requestedClone?.id ?? null;
  lotId.value =
    requestedLot && requestedClone && requestedLot.cloneId === requestedClone.id
      ? requestedLot.id
      : null;
  conjugateId.value =
    requestedConjugate && requestedLot && requestedConjugate.lotId === requestedLot.id
      ? requestedConjugate.id
      : null;
  speciesId.value = null;
  application.value = '1';
  status.value = '3';

  positiveControl.value = null;
  negativeControl.value = null;
  incubationConditions.value = null;
  concentration.value = null;
  concentrationUnit.value = null;
  tissue.value = null;
  fixation.value = null;
  fixationNotes.value = null;
  notes.value = null;
  antigenRetrievalType.value = null;
  antigenRetrievalTime.value = null;
  antigenRetrievalTemperature.value = null;
  saponin.value = 'null';
  saponinConcentration.value = null;
  methanolTreatment.value = 'null';
  methanolTreatmentConcentration.value = null;
  surfaceStaining.value = 'null';
  surfaceStainingConcentration.value = null;

  lotSearchInput.value = '';
  conjugateSearchInput.value = '';
  form.value?.resetValidation?.();
}

// Submit Handler
async function submit() {
  if (form.value?.validate() && activeGroupId.value && proteinId.value && cloneId.value && lotId.value) {
    const currentMemberId =
      groupStore.myMember?.groupId === activeGroupId.value ? groupStore.myMember.id : null;
    if (!currentMemberId) return;

    const dto: CreateValidationDto = {
      createdBy: currentMemberId,
      groupId: activeGroupId.value,
      cloneId: cloneId.value,
      lotId: lotId.value,
      conjugateId: conjugateId.value ?? null,
      speciesId: speciesId.value ?? null,
      application: Number(application.value),
      status: Number(status.value),
      positiveControl: positiveControl.value,
      negativeControl: negativeControl.value,
      incubationConditions: incubationConditions.value,
      concentration: concentration.value,
      concentrationUnit: concentrationUnit.value,
      tissue: tissue.value,
      fixation: fixation.value !== null ? Number(fixation.value) : null,
      fixationNotes: fixationNotes.value,
      notes: notes.value,
      antigenRetrievalType: antigenRetrievalType.value,
      antigenRetrievalTime: antigenRetrievalTime.value,
      antigenRetrievalTemperature: antigenRetrievalTemperature.value,
      saponin: saponin.value === 'true' ? true : saponin.value === 'false' ? false : null,
      saponinConcentration: saponinConcentration.value,
      methanolTreatment: methanolTreatment.value === 'true' ? true : methanolTreatment.value === 'false' ? false : null,
      methanolTreatmentConcentration: methanolTreatmentConcentration.value,
      surfaceStaining: surfaceStaining.value === 'true' ? true : surfaceStaining.value === 'false' ? false : null,
      surfaceStainingConcentration: surfaceStainingConcentration.value,
    };

    const validation = await validationStore.createValidation(dto);

    const selectedFile = Array.isArray(file.value) ? file.value[0] ?? null : file.value;

    if (validation?.id && selectedFile) {
      const formData = new FormData();
      formData.append('groupId', activeGroupId.value.toString());
      formData.append('file', selectedFile, selectedFile.name);
      await validationStore.uploadValidationFile({
        validationId: validation.id,
        formData,
      });
    }

    router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''));
  }
}

// Init
onMounted(async () => {
  reset();
  await Promise.all([
    groupStore.getMyMember(activeGroupId.value),
    cloneStore.getGroupClones(activeGroupId.value),
    proteinStore.fetchGroupProteins(activeGroupId.value),
    lotStore.getGroupLots(activeGroupId.value),
    conjugateStore.getGroupConjugates(activeGroupId.value),
    speciesStore.getGroupSpecies(activeGroupId.value),
  ]);
});
</script>
