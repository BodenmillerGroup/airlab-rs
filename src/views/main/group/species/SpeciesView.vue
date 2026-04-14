<template>
  <v-card tile elevation="1">
    <v-card-text>
      <div><span class="subheader">ID: </span>{{ species.id }}</div>
      <div><span class="subheader">Name: </span>{{ species.name }}</div>
      <div><span class="subheader">Acronym: </span>{{ species.acronym }}</div>
      <div><span class="subheader">Created: </span>{{ new Date(String(species.createdAt)).toUTCString() }}</div>
    </v-card-text>
    <v-card-actions>
      <v-btn
        color="primary"
        text
        :to="{
          name: 'main-group-species-edit',
          params: {
            groupId: activeGroupId,
            id: species.id,
          },
        }"
      >
        Edit
      </v-btn>
      <v-btn v-if="isGroupAdmin" color="secondary" text @click="deleteSpecies">Delete</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useSpeciesStore } from '@/stores/species';
import { useGroupStore } from '@/stores/group';

const props = defineProps<{
  speciesId: number;
}>();

const router = useRouter();
const route = useRoute();

const speciesStore = useSpeciesStore();
const groupStore = useGroupStore();

const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const species = computed(() => speciesStore.getSpecies(props.speciesId));

async function deleteSpecies() {
  if (confirm('Are you sure you want to delete the species?')) {
    if (confirm('All children entities will be deleted!')) {
      await speciesStore.deleteSpecies(props.speciesId);
    }
  }
}

onMounted(async () => {
  await speciesStore.getSpeciesById(props.speciesId);
});
</script>

<style scoped>
.subheader {
  font-weight: bold;
}
</style>
