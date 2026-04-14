<template>
  <v-card v-if="panel" tile elevation="1">
    <v-card-text>
      <div><span class="subheader">ID: </span>{{ panel.id }}</div>
      <div><span class="subheader">Name: </span>{{ panel.name }}</div>
      <div><span class="subheader">Description: </span>{{ panel.description }}</div>
      <div>
        <span class="subheader">Created By: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-admin-users-edit',
            params: { id: panel.userId },
          }"
        >
          {{ panel.userName }}
        </router-link>
      </div>
      <div v-if="panel.application">
        <span class="subheader">Application: </span>
        <v-chip color="blue-lighten-3" size="small" class="chip" disabled>
          {{ applicationToString(panel.application) }}
        </v-chip>
      </div>
      <v-checkbox label="Fluorophore" v-model="panel.isFluorophore" readonly hide-details />
      <v-checkbox label="Locked" v-model="panel.isLocked" readonly hide-details />
    </v-card-text>

    <v-card-actions>
      <v-btn
        color="primary"
        variant="text"
        :to="{
          name: 'main-group-panels-edit',
          params: {
            groupId: activeGroupId,
            id: panel.id,
          },
        }"
      >
        Edit
      </v-btn>
      <v-btn
        v-if="isGroupAdmin"
        color="secondary"
        variant="text"
        @click="deletePanel"
      >
        Delete
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useGroupStore } from '@/stores/group'
import { usePanelStore } from '@/stores/panel'
import { applicationToString } from '@/utils/converters'
import { usePanels } from '@/composables/usePanels'

const props = defineProps<{
  panelId: number
}>()

// Stores
const groupStore = useGroupStore()
const panelStore = usePanelStore()

// Computed
const activeGroupId = computed(() => groupStore.activeGroupId)
const isGroupAdmin = computed(() => groupStore.isGroupAdmin)
const { items: panels } = usePanels()
const panel = computed(() => panels.value.find(p => p.id === props.panelId))

// Lifecycle
onMounted(async () => {
  await panelStore.getPanelById(props.panelId)
})

// Delete handler
async function deletePanel() {
  if (confirm('Are you sure you want to delete the panel?')) {
    if (confirm('All children entities will be deleted!')) {
      await panelStore.deletePanel(props.panelId)
    }
  }
}
</script>

<style scoped>
.subheader {
  font-weight: bold;
}
.chip {
  opacity: 1;
}
</style>
