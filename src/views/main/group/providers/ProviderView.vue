<template>
  <v-card tile elevation="1">
    <v-card-text>
      <div><span class="subheader">ID: </span>{{ provider.id }}</div>
      <div><span class="subheader">Name: </span>{{ provider.name }}</div>
      <div><span class="subheader">Description: </span>{{ provider.description }}</div>
      <div>
        <span class="subheader">URL: </span>
        <a target="_blank" :href="provider.url">{{ provider.url }}</a>
      </div>
      <div><span class="subheader">Created: </span>{{ new Date(provider.createdAt).toUTCString() }}</div>
    </v-card-text>
    <v-card-actions>
      <v-btn
        color="primary"
        text
        :to="{
          name: 'main-group-providers-edit',
          params: {
            groupId: activeGroupId,
            id: provider.id,
          },
        }"
      >
        Edit
      </v-btn>
      <v-btn v-if="isGroupAdmin" color="secondary" text @click="deleteProvider">
        Delete
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import { useProviderStore } from '@/stores/provider';
import { useGroupStore } from '@/stores/group';

// Props
const props = defineProps<{
  providerId: number;
}>();

// Stores
const providerStore = useProviderStore();
const groupStore = useGroupStore();

// Load on mount
onMounted(async () => {
  await providerStore.getProvider(props.providerId);
});

// Computed
const provider = computed(() => providerStore.getProvider(props.providerId));
const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);

// Methods
async function deleteProvider() {
  const confirm1 = confirm('Are you sure you want to delete the provider?');
  const confirm2 = confirm('All children entities will be deleted!');
  if (confirm1 && confirm2) {
    await providerStore.deleteProvider(props.providerId);
  }
}
</script>

<style scoped>
.subheader {
  font-weight: bold;
}
</style>
