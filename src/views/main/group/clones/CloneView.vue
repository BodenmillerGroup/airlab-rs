<template>
  <v-card tile elevation="1">
    <v-card-text>
      <div><span class="subheader">ID: </span>{{ clone.id }}</div>
      <div><span class="subheader">Name: </span>{{ clone.name }}</div>
      <div>
        <span class="subheader">Protein: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-group-proteins-edit',
            params: { groupId: activeGroupId, id: clone.proteinId },
          }"
        >
          {{ proteinName }}
        </router-link>
      </div>
      <div v-if="clone.speciesId">
        <span class="subheader">Host: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-group-species-edit',
            params: { groupId: activeGroupId, id: clone.speciesId },
          }"
        >
          {{ speciesName }}
        </router-link>
      </div>
      <div v-if="createdByUser">
        <span class="subheader">Created by: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-admin-users-edit',
            params: { id: createdByUser.id },
          }"
        >
          {{ createdByUser.name }}
        </router-link>
      </div>
      <div><span class="subheader">Isotype: </span>{{ clone.isotype }}</div>
      <div><span class="subheader">Epitope: </span>{{ clone.epitope }}</div>
      <div><span class="subheader">Polyclonal: </span>{{ clone.isPolyclonal }}</div>
      <div><span class="subheader">Phospho: </span>{{ clone.isPhospho }}</div>

      <div class="subheader">Application:</div>
      <v-chip-group multiple column active-class="primary--text">
        <v-chip v-for="key in Object.keys(applicationMap)" :key="key" :color="getApplicationColor(applicationMap[key])" small disabled class="chip">
          {{ key }}
        </v-chip>
      </v-chip-group>

      <div class="subheader">Reactivity:</div>
      <v-chip-group :value="clone.reactivity" multiple column active-class="primary--text">
        <v-chip v-for="s in species" :key="s.id" :value="s.id" label small disabled class="chip">
          {{ s.name }}
        </v-chip>
      </v-chip-group>
    </v-card-text>

    <v-card-actions>
      <v-btn
        color="primary"
        text
        :to="{
          name: 'main-group-clones-edit',
          params: { groupId: activeGroupId, id: clone.id },
        }"
      >
        Edit
      </v-btn>
      <v-btn v-if="isGroupAdmin" color="secondary" text @click="deleteClone">Delete</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useGroupStore } from "@/stores/group";
import { useCloneStore } from "@/stores/clone";
import { useSpeciesStore } from "@/stores/species";
import { useProteinStore } from "@/stores/protein";
import { useMemberStore } from "@/stores/member";
import { useUserStore } from "@/stores/user";
import { applicationNameToId } from "@/utils/enums";

// Props
const props = defineProps<{ cloneId: number }>();

// Stores
const route = useRoute();
const groupStore = useGroupStore();
const cloneStore = useCloneStore();
const speciesStore = useSpeciesStore();
const proteinStore = useProteinStore();
const memberStore = useMemberStore();
const userStore = useUserStore();

// Constants
const applicationMap = applicationNameToId;

// State
const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const clone = computed(() => cloneStore.getClone(props.cloneId));
const species = computed(() => speciesStore.species);
const createdByMember = computed(() => {
  if (!clone.value) return null;
  return memberStore.getMemberById(clone.value.createdBy);
});
const createdByUser = computed(() => {
  if (!createdByMember.value) return null;
  return userStore.getUserById(createdByMember.value.userId);
});
const proteinName = computed(() => {
  if (!clone.value) return '—';
  return proteinStore.getProtein(clone.value.proteinId)?.name ?? '—';
});
const speciesName = computed(() => {
  if (!clone.value?.speciesId) return '—';
  return speciesStore.getSpecies(clone.value.speciesId)?.name ?? '—';
});

function getApplicationColor(application: number) {
  return clone.value?.application?.hasOwnProperty(application)
    ? clone.value.application[application]
      ? "green lighten-2"
      : "red lighten-2"
    : "grey lighten-2";
}

async function deleteClone() {
  if (confirm("Are you sure you want to delete the clone?")) {
    if (confirm("All children lots and conjugates will be deleted!")) {
      await cloneStore.deleteClone(props.cloneId);
    }
  }
}

onMounted(async () => {
  const groupId = Number(route.params.groupId);
  const cloneData = await cloneStore.getClone(props.cloneId);
  if (cloneData) {
    await memberStore.fetchByIds([cloneData.createdBy]);
    const createdByUserId = memberStore.getMemberById(cloneData.createdBy)?.userId;

    await Promise.all([
      speciesStore.getGroupSpecies(groupId),
      proteinStore.fetchByIds(groupId, [cloneData.proteinId]),
      createdByUserId ? userStore.fetchByIds([createdByUserId]) : Promise.resolve(),
    ]);
  }
});
</script>

<style scoped>
.subheader {
  font-weight: bold;
}
.chip {
  opacity: 1;
}
</style>
