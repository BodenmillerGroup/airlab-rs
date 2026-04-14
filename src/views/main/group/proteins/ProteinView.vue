<template>
  <v-card elevation="1" flat>
    <v-card-text>
      <div><span class="subheader">ID:</span> {{ protein.id }}</div>
      <div><span class="subheader">Name:</span> {{ protein.name }}</div>
      <div><span class="subheader">Description:</span> {{ protein.description }}</div>
      <div><span class="subheader">Created:</span> {{ new Date(protein.createdAt).toUTCString() }}</div>
    </v-card-text>

    <v-card-actions>
      <v-btn
        color="primary"
        variant="text"
        :to="{
          name: 'main-group-proteins-edit',
          params: {
            groupId: activeGroupId,
            id: protein.id,
          },
        }"
      >
        Edit
      </v-btn>

      <v-btn
        v-if="isGroupAdmin"
        color="secondary"
        variant="text"
        @click="deleteProtein"
      >
        Delete
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts" setup>
import { computed, onMounted } from 'vue';
import { useGroupStore } from '@/stores/group';
import { useProteinStore } from '@/stores/protein';
import { useRoute } from 'vue-router';

interface Props {
  proteinId: number;
}
const props = defineProps<Props>();

// stores
const groupStore = useGroupStore();
const proteinStore = useProteinStore();

const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const activeGroupId = computed(() => groupStore.activeGroupId);

const protein = computed(() => proteinStore.getProteinById(props.proteinId));

const deleteProtein = async () => {
  if (confirm('Are you sure you want to delete the protein?')) {
    if (confirm('All children entities will be deleted!')) {
      await proteinStore.deleteProtein(props.proteinId);
    }
  }
};

onMounted(async () => {
  await proteinStore.getProteinById(props.proteinId);
});
</script>

<style scoped>
.subheader {
  font-weight: bold;
}
</style>
