<template>
  <v-container fluid class="px-1 py-0">
    <router-view />
  </v-container>
</template>

<script lang="ts" setup>
import { computed, onMounted, onBeforeUnmount } from 'vue';
import { useRoute } from 'vue-router';
import { useGroupStore } from '@/stores/group';

const route = useRoute();
const groupStore = useGroupStore();

const group = computed(() => groupStore.activeGroup);

onMounted(async () => {
  const groupId = parseInt(String(route.params.groupId), 10);
  if (!isNaN(groupId)) {
    groupStore.setActiveGroupId(groupId);
    await Promise.all([
      groupStore.getMyMember(groupId),
      groupStore.getGroup(groupId),
    ]);
  }
});

onBeforeUnmount(() => {
  // WebSocketManager.close(); // optionally restore if needed
  groupStore.reset(); // Pinia's built-in state reset
});
</script>
