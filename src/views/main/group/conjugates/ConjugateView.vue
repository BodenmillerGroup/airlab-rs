<template>
  <v-card tile elevation="1">
    <v-card-text v-if="conjugate">
      <div><span class="subheader">ID: </span>{{ conjugate.id }}</div>
      <div><span class="subheader">Tube Number: </span>{{ conjugate.tubeNumber }}</div>
      <div>
        <span class="subheader">Tag: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-group-tags-edit',
            params: {
              groupId: activeGroupId,
              id: conjugate.tagId,
            },
          }"
        >
          {{ conjugate.tagMw ? conjugate.tagName + conjugate.tagMw : conjugate.tagName }}
        </router-link>
      </div>
      <div>
        <span class="subheader">Lot: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-group-lots-edit',
            params: {
              groupId: activeGroupId,
              id: conjugate.lotId,
            },
          }"
        >
          {{ conjugate.lotName }}
        </router-link>
      </div>
      <div>
        <span class="subheader">Clone: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-group-clones-edit',
            params: {
              groupId: activeGroupId,
              id: conjugate.cloneId,
            },
          }"
        >
          {{ conjugate.cloneName }}
        </router-link>
      </div>
      <div>
        <span class="subheader">Protein: </span>
        <router-link
          class="link"
          :to="{
            name: 'main-group-proteins-edit',
            params: {
              groupId: activeGroupId,
              id: conjugate.proteinId,
            },
          }"
        >
          {{ conjugate.proteinName }}
        </router-link>
      </div>
      <div><span class="subheader">Concentration: </span>{{ conjugate.concentration }}</div>
      <div><span class="subheader">Description: </span>{{ conjugate.description }}</div>
      <div><span class="subheader">Custom ID: </span>{{ conjugate.customId }}</div>
      <div><span class="subheader">Created: </span>{{ new Date(conjugate.createdAt).toUTCString() }}</div>
      <div><span class="subheader">Updated: </span>{{ new Date(conjugate.updatedAt).toUTCString() }}</div>
    </v-card-text>
    <v-card-actions>
      <v-btn
        color="primary"
        text
        :to="{
          name: 'main-group-conjugates-edit',
          params: {
            groupId: activeGroupId,
            id: conjugate?.id ?? props.conjugateId,
          },
        }"
      >
        Edit
      </v-btn>
      <v-btn v-if="isGroupAdmin" color="secondary" text @click="deleteConjugate()">Delete</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { useGroupStore } from "@/stores/group";
import { useConjugateStore } from "@/stores/conjugate";
import { useConjugates } from "@/composables/useConjugates";

const props = defineProps<{ conjugateId: number }>();
const groupStore = useGroupStore();
const conjugateStore = useConjugateStore();
const router = useRouter();
const { items } = useConjugates();

const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const activeGroupId = computed(() => groupStore.activeGroupId);
const conjugate = computed(() => items.value.find(item => item.id === props.conjugateId));

async function deleteConjugate() {
  if (confirm("Are you sure you want to delete the conjugate?") && confirm("All children entities will be deleted!")) {
    await conjugateStore.deleteConjugate(props.conjugateId);
    router.back();
  }
}

</script>

<style scoped>
.subheader {
  font-weight: bold;
}
</style>
