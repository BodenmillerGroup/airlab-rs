<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Edit Clone</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn @click="cancel" text color="primary">Cancel</v-btn>
        <v-btn @click="reset" text color="primary">Reset</v-btn>
        <v-btn @click="submit" text :disabled="!valid" color="primary">Save</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form v-model="valid" ref="formRef">
          <v-text-field label="Clone Name" v-model="name" :rules="nameRules" />
          <v-autocomplete
            label="Protein"
            v-model="proteinId"
            :items="proteins"
            item-title="name"
            item-value="id"
            :rules="proteinRules"
            dense
          />
          <v-autocomplete
            label="Host"
            v-model="speciesId"
            :items="species"
            item-title="name"
            item-value="id"
            :rules="hostRules"
            dense
          />
          <v-text-field label="Epitope" v-model="epitope" />
          <v-text-field label="Isotype" v-model="isotype" />
          <v-checkbox label="Polyclonal" v-model="isPolyclonal" />
          <v-checkbox label="Phosphoantibody" v-model="isPhospho" />

          <v-row>
            <v-col cols="4">
              <div class="text-subtitle-1">Reactivity</div>
              <v-chip-group v-model="reactivity" multiple column class="reactivity-group">
                <v-chip v-for="item in species" :key="item.id" :value="item.id" class="reactivity-chip" small label>
                  {{ item.name }}
                </v-chip>
              </v-chip-group>
            </v-col>

            <v-col />
            <v-col cols="7">
              <div class="text-subtitle-1">Application</div>
              <v-row>
                <v-col v-for="(label, key) in applicationFields" :key="key">
                  <div class="subtitle-3">{{ label }}</div>
                  <v-btn-toggle v-model="applications[key]" class="application-toggle">
                    <v-btn small value="true" class="true">Yes</v-btn>
                    <v-btn small value="false" class="false">No</v-btn>
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
import { required } from '@/utils/validators';
import { applicationNameToId } from '@/utils/enums';
import { useCloneStore } from '@/stores/clone';
import { useProteinStore } from '@/stores/protein';
import { useSpeciesStore } from '@/stores/species';
import type { UpdateCloneDto } from '@/modules/clone/types';

const route = useRoute();
const router = useRouter();

const formRef = ref();
const valid = ref(false);

const cloneStore = useCloneStore();
const proteinStore = useProteinStore();
const speciesStore = useSpeciesStore();

const cloneId = computed(() => Number(route.params.id));
const groupId = computed(() => Number(route.params.groupId));
const clone = computed(() => cloneStore.getClone(cloneId.value));

const name = ref('');
const proteinId = ref<number | null>(null);
const epitope = ref('');
const isotype = ref('');
const isPhospho = ref(false);
const isPolyclonal = ref(false);
const speciesId = ref<number | null>(null);
const reactivity = ref<number[]>([]);

const applications = ref<Record<string, string | null>>({
  sMC: null,
  iMC: null,
  FC: null,
  IF: null,
  IHC: null,
  IHCF: null,
  WB: null
});

const nameRules = [required];
const proteinRules = [required];
const hostRules = [required];

const proteins = computed(() =>
  proteinStore.getGroupProteins(groupId.value)
);
const species = computed(() => speciesStore.species);

const applicationFields = {
  sMC: 'SMC',
  iMC: 'IMC',
  FC: 'FC',
  IF: 'IF',
  IHC: 'IHC',
  IHCF: 'IHC-F',
  WB: 'WB'
};

const reset = () => {
  if (formRef.value) formRef.value.resetValidation();

  if (clone.value) {
    name.value = clone.value.name;
    proteinId.value = clone.value.proteinId;
    epitope.value = clone.value.epitope;
    isotype.value = clone.value.isotype;
    isPolyclonal.value = clone.value.isPolyclonal;
    isPhospho.value = clone.value.isPhospho;
    speciesId.value = clone.value.speciesId;
    reactivity.value = clone.value.reactivity || [];

    Object.keys(applications.value).forEach((key) => {
      const appKey = applicationNameToId[key as keyof typeof applicationNameToId];
      applications.value[key] =
        clone.value?.application?.[appKey] !== undefined
          ? clone.value.application[appKey].toString()
          : null;
    });
  }
};

const submit = async () => {
  const form = formRef.value;
  if (!form || !(await form.validate()) || !clone.value) return;

  const application: Record<number, boolean> = {};
  for (const [key, value] of Object.entries(applications.value)) {
    if (value !== null) {
      application[applicationNameToId[key as keyof typeof applicationNameToId]] = value === 'true';
    }
  }

  const data: UpdateCloneDto = {
    name: name.value,
    proteinId: proteinId.value!,
    epitope: epitope.value,
    isotype: isotype.value,
    isPhospho: isPhospho.value,
    isPolyclonal: isPolyclonal.value,
    speciesId: speciesId.value!,
    reactivity: reactivity.value,
    application
  };

  await cloneStore.updateClone({ id: cloneId.value, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
};

const cancel = () => {
  router.back();
};

onMounted(async () => {
  await Promise.all([
    cloneStore.getClone(cloneId.value),
    speciesStore.getGroupSpecies(groupId.value)
  ]);
  if (proteins.value.length === 0) {
    await proteinStore.fetchGroupProteins(groupId.value);
  }
  reset();
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
