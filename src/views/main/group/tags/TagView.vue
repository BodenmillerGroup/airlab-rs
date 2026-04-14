<template>
  <v-card v-if="tag" tile elevation="1">
    <v-card-text>
      <div><span class="subheader">ID: </span>{{ tag.id }}</div>
      <div><span class="subheader">Name: </span>{{ tag.name }}</div>
      <div>
        <span class="subheader">Status: </span>
        <v-chip :color="getTagStatusColor(tag)" class="mr-1" size="x-small" label>
          {{ tagStatusToString(tag.status) }}
        </v-chip>
      </div>
      <div v-if="tag.isMetal"><span class="subheader">Mass: </span>{{ tag.mw }}</div>
      <div v-if="tag.isFluorophore"><span class="subheader">Emission: </span>{{ tag.emission }}</div>
      <div v-if="tag.isFluorophore"><span class="subheader">Excitation: </span>{{ tag.excitation }}</div>
      <div><span class="subheader">Created: </span>{{ new Date(tag.createdAt).toUTCString() }}</div>

      <v-checkbox label="Metal" v-model="tag.isMetal" readonly hide-details />
      <v-checkbox label="Fluorophore" v-model="tag.isFluorophore" readonly hide-details />
      <v-checkbox label="Enzyme" v-model="tag.isEnzyme" readonly hide-details />
      <v-checkbox label="Biotin" v-model="tag.isBiotin" readonly hide-details />
      <v-checkbox label="Other" v-model="tag.isOther" readonly hide-details />

      <div v-if="tag.description"><span class="subheader">Description: </span>{{ tag.description }}</div>
    </v-card-text>

    <v-card-actions>
      <v-btn
        color="primary"
        variant="text"
        :to="{
          name: 'main-group-tags-edit',
          params: {
            groupId: activeGroupId,
            id: tag.id,
          },
        }"
      >
        Edit
      </v-btn>
      <v-btn v-if="isGroupAdmin" color="secondary" variant="text" @click="deleteTag">
        Delete
      </v-btn>
    </v-card-actions>
  </v-card>
</template>


<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useTagStore } from "@/stores/tag";
import { useGroupStore } from "@/stores/group";
import { getTagStatusColor, tagStatusToString } from "@/utils/converters";

// Props
const props = defineProps<{ tagId: number }>();

// Stores
const groupStore = useGroupStore();
const tagStore = useTagStore();

// State
const tag = computed(() => tagStore.getTag(props.tagId));
const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);

async function deleteTag() {
  if (confirm("Are you sure you want to delete the tag?")) {
    if (confirm("All children entities will be deleted!")) {
      await tagStore.deleteTag(props.tagId);
    }
  }
}

onMounted(async () => {
  await tagStore.getTag(props.tagId);
});
</script>
