<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { required } from '@/utils/validators';
import { useTagStore } from '@/stores/tag';
import { tagStatusEnum } from '@/utils/enums';
import type { UpdateTagDto } from '@/modules/tag/types';
import { TagStatus } from '@/modules/tag/TagStatus';
import type { TagStatus as TagStatusType } from '@/modules/tag/TagStatus';

const route = useRoute();
const router = useRouter();

const tagStore = useTagStore();

const form = ref();
const valid = ref(true);

const name = ref('');
const isMetal = ref(false);
const isFluorophore = ref(false);
const isEnzyme = ref(false);
const isBiotin = ref(false);
const isOther = ref(false);
const description = ref<string | null>(null);
const mw = ref<number | null>(null);
const emission = ref<number | null>(null);
const excitation = ref<number | null>(null);
const status = ref<TagStatusType>(TagStatus.Stock);

const statuses = tagStatusEnum;

function massRequired(value: any) {
  if (!isMetal.value) return true;
  return !!value || 'Required';
}

const nameRules = [required];
const mwRules = [massRequired];
const emissionRules: any[] = [];
const excitationRules: any[] = [];

const tag = computed(() => tagStore.getTag(Number(route.params.id)));

function cancel() {
  router.back();
}

function reset() {
  form.value?.resetValidation();
  if (tag.value) {
    name.value = tag.value.name;
    isMetal.value = tag.value.isMetal;
    isFluorophore.value = tag.value.isFluorophore;
    isEnzyme.value = tag.value.isEnzyme;
    isBiotin.value = tag.value.isBiotin;
    isOther.value = tag.value.isOther;
    description.value = tag.value.description;
    mw.value = tag.value.mw;
    emission.value = tag.value.emission;
    excitation.value = tag.value.excitation;
    status.value = tag.value.status as TagStatusType;
  }
}

async function submit() {
  if (form.value?.validate()) {
    const data: UpdateTagDto = {
      name: name.value,
      isMetal: isMetal.value,
      isFluorophore: isFluorophore.value,
      isEnzyme: isEnzyme.value,
      isBiotin: isBiotin.value,
      isOther: isOther.value,
      description: description.value,
      mw: mw.value,
      emission: emission.value,
      excitation: excitation.value,
      status: status.value,
    };
    await tagStore.updateTag({ id: tag.value!.id, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  await tagStore.getTag(Number(route.params.id));
  reset();
});
</script>

<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Edit Tag</v-toolbar-title>
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
          <v-text-field label="Name" v-model="name" :rules="nameRules" />
          <v-text-field label="Description" v-model="description" />
          <v-select label="Status" v-model="status" :items="statuses" item-value="value" item-text="text" dense />
          <v-checkbox label="Metal" v-model="isMetal" />
          <v-text-field v-if="isMetal" label="Mass" v-model.number="mw" :rules="mwRules" type="number" />
          <v-checkbox label="Fluorophore" v-model="isFluorophore" />
          <v-checkbox label="Enzyme" v-model="isEnzyme" />
          <v-checkbox label="Biotin" v-model="isBiotin" />
          <v-checkbox label="Other" v-model="isOther" />
          <v-text-field
            v-if="isFluorophore"
            label="Emission"
            v-model.number="emission"
            :rules="emissionRules"
            type="number"
          />
          <v-text-field
            v-if="isFluorophore"
            label="Excitation"
            v-model.number="excitation"
            :rules="excitationRules"
            type="number"
          />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>
