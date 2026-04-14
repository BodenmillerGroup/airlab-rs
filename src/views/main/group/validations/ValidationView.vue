<template>
  <v-card tile elevation="1" class="mb-3">
    <v-card-text>
      <div><span class="subheader">ID: </span>{{ validation.id }}</div>

      <div>
        <span class="subheader">Application: </span>
        <v-chip class="mr-1" small label>
          {{ applicationToString(validation.application ) }}
        </v-chip>
      </div>

      <div>
        <span class="subheader">Works: </span>
        <v-chip :color="getColor(validation)" class="mr-1" dark x-small label>
          {{ validationStatusToString(validation.status) }}
        </v-chip>
      </div>

      <div>
        <span class="subheader">Clone: </span>
        <router-link
          v-if="validation.cloneId"
          class="link"
          :to="{
            name: 'main-group-clones-edit',
            params: { groupId, id: validation.cloneId },
          }"
        >
          {{ validation.cloneName }}
        </router-link>
        <span v-else>{{ validation.cloneName }}</span>
      </div>

      <div v-if="validation.lotId">
        <span class="subheader">Lot: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-group-lots-edit',
            params: { groupId, id: validation.lotId },
          }"
        >
          {{ validation.lotName }}
        </router-link>
      </div>

      <div v-if="validation.conjugateId">
        <span class="subheader">Conjugate: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-group-lots-edit',
            params: { groupId, id: validation.conjugateId },
          }"
        >
          {{ validation.tubeNumber }}
        </router-link>
      </div>

      <div>
        <span class="subheader">Created by: </span>
        <router-link
          v-if="validation.userId"
          class="link"
          :to="{
            name: 'main-admin-users-edit',
            params: { id: validation.userId },
          }"
        >
          {{ validation.userName }}
        </router-link>
        <span v-else>{{ validation.userName ?? '—' }}</span>
      </div>

      <div><span class="subheader">Positive control: </span>{{ validation.positiveControl }}</div>
      <div><span class="subheader">Negative control: </span>{{ validation.negativeControl }}</div>

      <div v-if="validation.speciesId">
        <span class="subheader">Species tested: </span>{{ validation.speciesName }}
      </div>

      <div><span class="subheader">Tissue: </span>{{ validation.tissue }}</div>
      <div><span class="subheader">Incubation conditions: </span>{{ validation.incubationConditions }}</div>
      <div><span class="subheader">Concentration: </span>{{ validation.concentration }}</div>
      <div><span class="subheader">Concentration unit: </span>{{ validation.concentrationUnit }}</div>
      <div><span class="subheader">Fixation: </span>{{ validation.fixation }}</div>

      <div v-if="validation.fixationNotes">
        <span class="subheader">Fixation notes: </span>{{ validation.fixationNotes }}
      </div>

      <div v-if="validation.notes"><span class="subheader">Notes: </span>{{ validation.notes }}</div>

      <div v-if="validation.antigenRetrievalType">
        <span class="subheader">Protocol: </span>{{ validation.antigenRetrievalType }}
      </div>

      <div v-if="validation.antigenRetrievalTime">
        <span class="subheader">Antigen retrieval time: </span>{{ validation.antigenRetrievalTime }}
      </div>

      <div v-if="validation.antigenRetrievalTemperature">
        <span class="subheader">Antigen retrieval temperature: </span>{{ validation.antigenRetrievalTemperature }}
      </div>

      <div v-if="validation.saponin !== null">
        <v-checkbox label="Saponin" v-model="validation.saponin" readonly hide-details />
        <span class="subheader">Saponin concentration: </span>{{ validation.saponinConcentration }}
      </div>

      <div v-if="validation.methanolTreatment !== null">
        <v-checkbox label="Methanol treatment" v-model="validation.methanolTreatment" readonly hide-details />
        <span class="subheader">Methanol treatment concentration: </span>{{ validation.methanolTreatmentConcentration }}
      </div>

      <div v-if="validation.surfaceStaining !== null">
        <v-checkbox label="Surface staining" v-model="validation.surfaceStaining" readonly hide-details />
        <span class="subheader">Surface staining concentration: </span>{{ validation.surfaceStainingConcentration }}
      </div>

      <div v-for="file in validation.validationFiles" :key="file.id">
        <iframe :src="`${apiUrl}/validationFiles/${file.id}/serve`" allowfullscreen class="iframe" />
        <a target="_blank" :href="`${apiUrl}/validationFiles/${file.id}/serve`">{{ file.name }}</a>
      </div>
    </v-card-text>

    <v-card-actions>
      <v-btn
        v-if="validation.id"
        color="primary"
        text
        :to="{
          name: 'main-group-validations-edit',
          params: { groupId, id: validation.id },
        }"
      >
        Edit
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import type { ValidationView as ValidationViewDto } from "@/modules/validation/types";
import { applicationToString, getStatusColor, validationStatusToString } from '@/utils/converters';

// Props
const props = defineProps<{
  groupId: number;
  validation: ValidationViewDto;
  apiUrl: string;
}>();

function getColor(validation: ValidationViewDto): string {
  return getStatusColor(validation);
}
</script>

<style scoped>
.iframe {
  width: 100%;
  height: 400px;
  border: 0;
}
.subheader {
  font-weight: bold;
}
</style>
