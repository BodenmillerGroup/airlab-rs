<template>
  <v-card flat>
    <v-card-title>Clone Details</v-card-title>
    <v-card-text>
      <v-tabs v-model="tab">
        <v-tab>Info</v-tab>
        <v-tab v-if="validations.length > 0">Validations</v-tab>
      </v-tabs>
      <v-window v-model="tab" class="mt-4">

        <v-window-item value="info">
          <CloneView :clone-id="clone.id" />
        </v-window-item>

        <v-window-item value="validations">
          <!--<ValidationView
            v-for="validation in validations"
            :key="validation.id"
            :group-id="activeGroupId"
            :validation="validation"
            :api-url="apiUrl"
          />-->
        </v-window-item>
      </v-window>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useCloneStore } from "@/stores/clone";
import { useGroupStore } from "@/stores/group";
import CloneView from "@/views/main/group/clones/CloneView.vue";
import ValidationView from "@/views/main/group/validations/ValidationView.vue";
import { apiUrl } from "@/env";
import type { CloneDto } from "@/modules/clone/types";
import type { ValidationDto } from "@/modules/validation/types";

// Props
const props = defineProps<{
  clone: CloneDto;
}>();

// Store
const cloneStore = useCloneStore();
const groupStore = useGroupStore();

// Reactive state
const tab = ref(0);
const validations = ref<ValidationDto[]>([]);

// Computed
const activeGroupId = computed(() => groupStore.activeGroupId);

// Lifecycle
onMounted(async () => {
  validations.value = await cloneStore.getCloneValidations(props.clone.id);
});
</script>
