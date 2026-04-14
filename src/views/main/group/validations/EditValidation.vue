<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { required } from '@/utils/validators';
import { useGroupStore } from '@/stores/group';
import { useLotStore } from '@/stores/lot';
import { useConjugateStore } from '@/stores/conjugate';
import { useValidationStore } from '@/stores/validation';
import { useValidationFileStore } from '@/stores/validation_file';
import { useCloneStore } from '@/stores/clone';
import { useSpeciesStore } from '@/stores/species';
import { antigenRetrievalTypes } from '@/utils/enums';
import type { UpdateValidationDto } from '@/modules/validation/types';

const route = useRoute();
const router = useRouter();

const groupStore = useGroupStore();
const validationStore = useValidationStore();
const validationFileStore = useValidationFileStore();
const cloneStore = useCloneStore();
const lotStore = useLotStore();
const conjugateStore = useConjugateStore();
const speciesStore = useSpeciesStore();

const form = ref();
const valid = ref(false);
const file = ref<File | File[] | null>(null);

const cloneId = ref<number | null>(null);
const lotId = ref<number | null>(null);
const lotSearchInput = ref('');
const conjugateId = ref<number | null>(null);
const speciesId = ref<number | null>(null);

const application = ref('1');
const status = ref('3');
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

const requiredRules = [required];
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

const antigenRetrievalTypesList = antigenRetrievalTypes;

const activeGroupId = computed(() => groupStore.activeGroupId);
const clones = computed(() => cloneStore.clones);
const species = computed(() => speciesStore.species);

const lots = computed(() => {
  let items = lotStore.lots;
  if (cloneId.value) items = items.filter((item) => item.cloneId === cloneId.value);
  return items;
});

const conjugates = computed(() => {
  let items = conjugateStore.conjugates;
  if (lotId.value) items = items.filter((item) => item.lotId === lotId.value);
  return items;
});

const validation = computed(() => validationStore.getValidation(Number(route.params.id)));
const validationFiles = computed(() => {
  const id = Number(route.params.id);
  return validationFileStore.byValidationId[id] ?? [];
});

function resetLotId() {
  lotId.value = null;
  lotSearchInput.value = '';
  conjugateId.value = null;
}

function resetConjugateId() {
  conjugateId.value = null;
}

function filterConjugates(item: any, query: string, itemText: string) {
  if (!query) return true;
  const term = query.toLowerCase().trim();
  return String(item?.tubeNumber ?? '').toLowerCase().includes(term);
}

function cancel() {
  router.back();
}

function reset() {
  if (form.value) form.value.resetValidation();
  if (validation.value) {
    const v = validation.value;
    cloneId.value = v.cloneId;
    lotId.value = v.lotId ?? null;
    conjugateId.value = v.conjugateId ?? null;
    speciesId.value = v.speciesId;
    application.value = v.application.toString();
    status.value = v.status.toString();
    positiveControl.value = v.positiveControl;
    negativeControl.value = v.negativeControl;
    incubationConditions.value = v.incubationConditions;
    concentration.value = v.concentration;
    concentrationUnit.value = v.concentrationUnit;
    tissue.value = v.tissue;
    fixation.value = v.fixation;
    fixationNotes.value = v.fixationNotes;
    notes.value = v.notes;
    antigenRetrievalType.value = v.antigenRetrievalType;
    antigenRetrievalTime.value = v.antigenRetrievalTime;
    antigenRetrievalTemperature.value = v.antigenRetrievalTemperature;
    saponin.value = v.saponin === true ? 'true' : v.saponin === false ? 'false' : 'null';
    saponinConcentration.value = v.saponinConcentration;
    methanolTreatment.value = v.methanolTreatment === true ? 'true' : v.methanolTreatment === false ? 'false' : 'null';
    methanolTreatmentConcentration.value = v.methanolTreatmentConcentration;
    surfaceStaining.value = v.surfaceStaining === true ? 'true' : v.surfaceStaining === false ? 'false' : 'null';
    surfaceStainingConcentration.value = v.surfaceStainingConcentration;
  }
}

async function submit() {
  if (form.value?.validate() && validation.value && cloneId.value) {
    const data: UpdateValidationDto = {
      cloneId: cloneId.value,
      lotId: lotId.value ?? null,
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
      fixation: fixation.value ?? null,
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
    await validationStore.updateValidation({ id: validation.value.id, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

async function upload() {
  const selectedFile = Array.isArray(file.value) ? file.value[0] ?? null : file.value;

  if (activeGroupId.value && validation.value && selectedFile) {
    const formData = new FormData();
    formData.append('groupId', activeGroupId.value.toString());
    formData.append('file', selectedFile, selectedFile.name);
    await validationStore.uploadValidationFile({
      validationId: validation.value.id,
      formData,
    });
    await validationFileStore.getValidationFiles(validation.value.id);
    file.value = null;
  }
}

async function deleteFile(f: any) {
  if (confirm('Are you sure you want to delete the validation file?')) {
    await validationStore.deleteValidationFile(f.id);
    if (validation.value) {
      await validationFileStore.getValidationFiles(validation.value.id);
    }
  }
}

onMounted(async () => {
  const id = Number(route.params.id);
  const groupId = Number(route.params.groupId);
  await Promise.all([
    validationStore.getValidation(id),
    cloneStore.getGroupClones(groupId),
    lotStore.getGroupLots(groupId),
    conjugateStore.getGroupConjugates(groupId),
    speciesStore.getGroupSpecies(groupId),
    validationFileStore.getValidationFiles(id),
  ]);
  reset();
});
</script>

<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Edit Validation</v-toolbar-title>
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
                label="Clone"
                v-model="cloneId"
                :items="clones"
                item-text="name"
                item-value="id"
                :rules="requiredRules"
                dense
                @change="resetLotId"
              />
            </v-col>
            <v-col>
              <v-autocomplete
                label="Lot"
                v-model="lotId"
                :search-input.sync="lotSearchInput"
                :items="lots"
                item-text="name"
                item-value="id"
                clearable
                open-on-clear
                dense
                @change="resetConjugateId"
              />
            </v-col>
            <v-col>
              <v-autocomplete
                label="Conjugate"
                v-model="conjugateId"
                :items="conjugates"
                :filter="filterConjugates"
                item-text="tubeNumber"
                item-value="id"
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
            item-text="name"
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
                item-value="id"
                item-text="name"
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
                :items="antigenRetrievalTypesList"
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
        <div class="text-h5">Validation Files</div>
      </v-card-title>
      <v-card-text>
        <v-list dense>
          <template v-for="(f, index) in validationFiles" :key="index">
            <v-list-item dense two-line>
              <v-col cols="3">{{ f.id }}</v-col>
              <v-col>{{ f.name }}</v-col>
              <v-col cols="1">
                <v-tooltip bottom>
                  <template #activator="{ props }">
                    <v-btn v-bind="props" fab x-small color="secondary lighten-3" @click.stop="deleteFile(f)">
                      <v-icon>mdi-minus</v-icon>
                    </v-btn>
                  </template>
                  <span>Delete file</span>
                </v-tooltip>
              </v-col>
            </v-list-item>
          </template>
        </v-list>
      </v-card-text>
      <v-card-text>
        <v-form>
          <v-file-input v-model="file" label="File upload" show-size />
        </v-form>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn @click="upload" :disabled="!file">Upload</v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
