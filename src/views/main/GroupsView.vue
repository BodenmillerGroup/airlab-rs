<template>
  <v-container fluid>
    <v-card v-if="!mfaEnabled" class="ma-6 pa-6">
      <div class="text-h6 mb-2">Enable MFA To Access Groups</div>
      <p class="text-body-1 mb-4">
        Multi-factor authentication must be enabled before group access is available.
      </p>
      <v-btn color="primary" to="/main/profile/view">Go To Profile</v-btn>
    </v-card>

    <template v-else>
    <v-row class="mt-6 mx-6">
      <v-text-field
        v-model="search"
        append-icon="mdi-magnify"
        label="Search"
        single-line
        hide-details
        clearable
        solo
        class="mt-1"
      />
    </v-row>

    <div class="group-list">
      <div class="group-list-item" v-for="group in filteredGroups" :key="group.id">
        <GroupCard :group="group" :user="userProfile" />
      </div>
    </div>
    </template>
  </v-container>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue';
import GroupCard from '@/views/main/GroupCard.vue';
import { useMainStore } from '@/stores/main';
import { useGroups } from '@/composables/useGroups';

const search = ref('');
const mainStore = useMainStore();

// NEW: use composable instead of store

const userProfile = computed(() => mainStore.userProfile);
const mfaEnabled = computed(() => Boolean(userProfile.value?.mfaEnabled));

const userId = computed(() => userProfile.value?.id);

const { items: groups, loading } = useGroups(userId);

const filteredGroups = computed(() => {
  const term = search.value.toLowerCase();
  return term
    ? groups.value.filter((g) => g.name.toLowerCase().includes(term))
    : groups.value;
});
</script>

<style scoped>
.group-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.group-list-item {
  width: 100%;
}
</style>
