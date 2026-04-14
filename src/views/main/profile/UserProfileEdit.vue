<template>
  <v-container fluid>
    <v-card class="ma-4 pa-4">
      <v-card-title>
        <div class="text-h5 primary--text">Edit User Profile</div>
      </v-card-title>
      <v-card-text>
        <v-form v-model="valid" ref="form" lazy-validation>
          <v-text-field label="Name" v-model="name" />
          <v-text-field
            label="E-mail"
            type="email"
            v-model="email"
            :rules="emailRules"
          />
        </v-form>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn @click="cancel">Cancel</v-btn>
        <v-btn @click="reset">Reset</v-btn>
        <v-btn @click="submit" :disabled="!valid">Save</v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useMainStore } from "@/stores/main";
import { email as emailRule, required } from "@/utils/validators";
import type { UpdateProfileDto } from "@/modules/user/types";

const mainStore = useMainStore();
const router = useRouter();

const emailRules = [required, emailRule];

const form = ref();
const valid = ref(true);

const name = ref("");
const email = ref("");

const userProfile = computed(() => mainStore.userProfile);

function reset() {
  if (userProfile.value) {
    name.value = userProfile.value.name;
    email.value = userProfile.value.email;
  }
}

function cancel() {
  router.back();
}

async function submit() {
  if (form.value?.validate()) {
    const updatedProfile: UpdateProfileDto = {
      name: name.value,
      email: email.value,
    };
    await mainStore.updateUserProfile(updatedProfile);
    router.push("/main/profile");
  }
}

onMounted(() => {
  reset();
});
</script>
