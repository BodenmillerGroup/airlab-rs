<template>
  <v-card tile elevation="1">
    <v-card-text>
      <div><span class="subheader">ID: </span>{{ lot.id }}</div>
      <div><span class="subheader">Name: </span>{{ lot.name }}</div>
      <div><span class="subheader">Lot Number: </span>{{ lot.number }}</div>
      <div><span class="subheader">Reference: </span>{{ lot.reference }}</div>
      <div>
        <span class="subheader">Status: </span>
        <v-chip :color="getLotStatusColor(lot.status as LotStatus)" class="mr-1" size="small" dark label>
          {{ lotStatusToString(lot.status) }}
        </v-chip>
      </div>
      <div><span class="subheader">Purpose: </span>{{ lot.purpose }}</div>
      <div><span class="subheader">Price: </span>{{ lot.price }}</div>
      <div v-if="provider">
        <span class="subheader">Provider: </span>
        <RouterLink
          class="link"
          :to="{
            name: 'main-group-providers-edit',
            params: { groupId: activeGroupId, id: lot.providerId },
          }"
        >
          {{ provider.name }}
        </RouterLink>
      </div>
      <div v-if="storage">
        <span class="subheader">Storage: </span>
        <RouterLink
          class="link"
          :to="{
            name: 'main-group-storage-edit',
            params: { groupId: activeGroupId, id: lot.storageId },
          }"
        >
          {{ storage.name }}
        </RouterLink>
      </div>
      <div><span class="subheader">Note: </span>{{ lot.note }}</div>
      <div>
        <span class="subheader">URL: </span>
        <a v-if="lot.url" target="_blank" :href="lot.url">{{ lot.url }}</a>
      </div>
    </v-card-text>

    <v-divider class="mx-4" />

    <v-list v-if="historyItems.length > 0" density="compact">
      <v-list-subheader>History</v-list-subheader>
      <v-list-item v-for="entry in historyItems" :key="entry.label">
        <template #prepend>
          <v-icon :color="entry.color">mdi-calendar</v-icon>
        </template>
        <v-list-item-title>{{ entry.label }} {{ formatTimestamp(entry.at) }}</v-list-item-title>
        <v-list-item-subtitle>
          by {{ entry.userName }}
        </v-list-item-subtitle>
      </v-list-item>
    </v-list>

    <!--
    <v-card-actions>
      <v-btn
        color="primary"
        variant="text"
        :to="{
          name: 'main-group-lots-edit',
          params: { groupId: activeGroupId, id: lot.id },
        }"
      >
        Edit
      </v-btn>
      <v-btn
        v-if="isGroupAdmin"
        color="error"
        variant="text"
        @click="deleteLot"
      >
        Delete
      </v-btn>
    </v-card-actions>
    -->
  </v-card>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { lotStatusToString } from '@/utils/converters'
import { useLotStore } from '@/stores/lot';
import { useGroupStore } from '@/stores/group';
import { useProviderStore } from '@/stores/provider';
import { useMemberStore } from '@/stores/member';
import { useUserStore } from '@/stores/user';
import { useStorageStore } from '@/stores/storage';
import { getLotStatusColor } from '@/utils/converters';
import { LotStatus } from '@/modules/lot/LotStatus';

const props = defineProps<{ lotId: number }>();

const lotStore = useLotStore();
const groupStore = useGroupStore();
const providerStore = useProviderStore();
const storageStore = useStorageStore();
const memberStore = useMemberStore();
const userStore = useUserStore();

const lot = computed(() => lotStore.getLot(props.lotId));
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const activeGroupId = computed(() => groupStore.activeGroupId);
const provider = computed(() => {
  if (!lot.value?.providerId) return null;
  return providerStore.getProvider(lot.value.providerId);
});
const storage = computed(() => {
  if (!lot.value?.storageId) return null;
  return storageStore.getStorage(lot.value.storageId);
});
const historyItems = computed(() => {
  if (!lot.value) return [];

  const events = [
    { label: 'Requested', memberId: lot.value.requestedBy, at: lot.value.requestedAt, color: 'blue' },
    { label: 'Approved', memberId: lot.value.approvedBy, at: lot.value.approvedAt, color: 'purple' },
    { label: 'Ordered', memberId: lot.value.orderedBy, at: lot.value.orderedAt, color: 'green' },
    { label: 'Received', memberId: lot.value.receivedBy, at: lot.value.receivedAt, color: 'orange' },
    { label: 'Finished', memberId: lot.value.finishedBy, at: lot.value.finishedAt, color: 'red' },
  ];

  return events
    .filter((event) => event.memberId || event.at)
    .map((event) => {
    const member = event.memberId ? memberStore.getMemberById(event.memberId) : undefined;
    const user = member?.userId ? userStore.getUserById(member.userId) : undefined;

    return {
      ...event,
      userName: user?.name ?? '—',
    };
    });
});

const getLotStatusColorRef = getLotStatusColor;

function formatTimestamp(value: string | null | undefined) {
  if (!value) {
    return '—';
  }
  return new Date(value).toUTCString();
}

async function deleteLot() {
  if (confirm('Are you sure you want to delete the lot?')) {
    if (confirm('All children conjugates will be deleted!')) {
      await lotStore.deleteLot(props.lotId);
    }
  }
}

onMounted(async () => {
  if (props.lotId) {
      console.warn('have lots', props.lotId)
      await lotStore.getLot(props.lotId);
      if (lot.value?.providerId && activeGroupId.value) {
        await providerStore.fetchByIds(activeGroupId.value, [lot.value.providerId]);
      }
      if (lot.value?.storageId) {
        await storageStore.fetchByIds([lot.value.storageId]);
      }
      const memberIds = [
        lot.value?.requestedBy,
        lot.value?.approvedBy,
        lot.value?.orderedBy,
        lot.value?.receivedBy,
        lot.value?.finishedBy,
      ].filter((id): id is number => typeof id === 'number' && id > 0);
      if (memberIds.length > 0) {
        await memberStore.fetchByIds(memberIds);
        const userIds = memberIds
          .map((memberId) => memberStore.getMemberById(memberId)?.userId)
          .filter((id): id is number => typeof id === 'number' && id > 0);
        if (userIds.length > 0) {
          await userStore.fetchByIds(userIds);
        }
      }
  } else {
    console.warn('⚠️ props.lot is undefined or missing id -- in lotview', props.lotId)
  }
});
</script>

<style scoped>
.subheader {
  font-weight: bold;
}
</style>
