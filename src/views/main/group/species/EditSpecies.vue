<template>
  <v-container fluid>
    <v-toolbar density="compact">
      <v-toolbar-title>Edit Species</v-toolbar-title>
      <v-spacer />
      <v-btn text @click="cancel" color="primary">Cancel</v-btn>
      <v-btn text @click="reset" color="primary">Reset</v-btn>
      <v-btn text @click="submit" :disabled="!valid" color="primary">Save</v-btn>
    </v-toolbar>

    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form v-model="valid" ref="formRef" validate-on="lazy">
          <v-text-field
            label="Name"
            v-model="name"
            :rules="nameRules"
          />
          <v-text-field
            label="Acronym"
            v-model="acronym"
            :rules="acronymRules"
          />
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useSpeciesStore } from "@/stores/species";
import { required } from "@/utils/validators";
import type { UpdateSpeciesDto } from "@/modules/species/types";

// Composables
const router = useRouter();
const route = useRoute();
const speciesStore = useSpeciesStore();

const id = computed(() => Number(route.params.id));
const tag = computed(() => speciesStore.getSpecies(id.value));

// Form State
const formRef = ref();
const valid = ref(true);
const name = ref("");
const acronym = ref("");

// Validation Rules
const nameRules = [required];
const acronymRules = [required];

// Lifecycle
onMounted(async () => {
  await speciesStore.getSpeciesById(id.value);
  reset();
});

// Methods
function reset() {
  name.value = tag.value?.name || "";
  acronym.value = tag.value?.acronym || "";
  formRef.value?.resetValidation();
}

function cancel() {
  router.back();
}

async function submit() {
  const isValid = await formRef.value?.validate();
  if (isValid) {
    const data: UpdateSpeciesDto = {
      name: name.value,
      acronym: acronym.value,
    };
    await speciesStore.updateSpecies({ id: id.value, data });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}
</script>
