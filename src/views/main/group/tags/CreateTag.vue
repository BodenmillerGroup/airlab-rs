<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Create Tag</v-toolbar-title>
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
          <v-select
            label="Status"
            v-model="status"
            :items="statuses"
            item-value="value"
            item-text="text"
            dense
          />

          <v-checkbox label="Metal" v-model="isMetal" />
          <v-text-field
            v-if="isMetal"
            label="Mass"
            v-model.number="mw"
            :rules="mwRules"
            type="number"
          />

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

<script setup lang="ts">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { storeToRefs } from 'pinia';

import { required } from '@/utils/validators';
import { useGroupStore } from '@/stores/group';
import { useTagStore } from '@/stores/tag';
import { tagStatusEnum } from '@/utils/enums';
import type { CreateTagDto } from '@/modules/tag/types';
import { TagStatus } from '@/modules/tag/TagStatus';
import type { TagStatus as TagStatusType } from '@/modules/tag/TagStatus';

// Stores
const router = useRouter();
const route = useRoute();
const groupStore = useGroupStore();
const tagStore = useTagStore();
const { activeGroupId } = storeToRefs(groupStore);

// Reactive form state
const valid = ref(true);
const form = ref();
const name = ref('');
const description = ref<string | null>(null);
const status = ref<TagStatusType>(TagStatus.Stock);
const isMetal = ref(false);
const isFluorophore = ref(false);
const isEnzyme = ref(false);
const isBiotin = ref(false);
const isOther = ref(false);
const mw = ref<number | null>(null);
const emission = ref<number | null>(null);
const excitation = ref<number | null>(null);

// Validation rules
const nameRules = [required];
const mwRules = [
  (value: number | null) => !isMetal.value || !!value || 'Mass is required for metal tag',
];
const emissionRules: any[] = [];
const excitationRules: any[] = [];

// Status enum for v-select
const statuses = tagStatusEnum;

// Actions
const reset = () => {
  name.value = '';
  description.value = null;
  status.value = TagStatus.Stock;
  isMetal.value = false;
  isFluorophore.value = false;
  isEnzyme.value = false;
  isBiotin.value = false;
  isOther.value = false;
  mw.value = null;
  emission.value = null;
  excitation.value = null;
  form.value?.resetValidation();
};

const cancel = () => {
  router.back();
};

const submit = async () => {
  if (form.value?.validate() && activeGroupId.value) {
    const dto: CreateTagDto = {
      groupId: activeGroupId.value,
      name: name.value,
      description: description.value,
      status: status.value,
      isMetal: isMetal.value,
      isFluorophore: isFluorophore.value,
      isEnzyme: isEnzyme.value,
      isBiotin: isBiotin.value,
      isOther: isOther.value,
      mw: mw.value,
      emission: emission.value,
      excitation: excitation.value,
    };
    await tagStore.createTag(dto);
    router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''));
  }
};
</script>
