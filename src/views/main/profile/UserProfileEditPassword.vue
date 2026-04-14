<template>
  <v-container fluid>
    <v-card class="ma-4 pa-4">
      <v-card-title>
        <div class="text-h5 primary--text">Set Password</div>
      </v-card-title>
      <v-card-text>
        <div class="my-4">
          <div class="subtitle-1 primary--text text--lighten-2">User</div>
          <div class="text-h6 primary--text text--darken-2">
            {{ userProfile.email }}
          </div>
        </div>
        <v-form ref="form">
          <v-text-field
            type="password"
            label="Password"
            v-model="password1"
            :rules="password1Rules"
          />
          <v-text-field
            type="password"
            label="Confirm Password"
            v-model="password2"
            :rules="password2Rules"
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
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import { useMainStore } from "@/stores/main";
import { required } from "@/utils/validators";
import type { UpdatePasswordDto } from "@/modules/user/types";

const mainStore = useMainStore();
const router = useRouter();

const form = ref();
const password1 = ref("");
const password2 = ref("");
const valid = computed(() => password1.value !== "" && password2.value !== "" && password1.value === password2.value);

const password1Rules = [required];
const password2Rules = [
  required,
  (v: string) => v === password1.value || "Password should be the same"
];

const userProfile = computed(() => mainStore.userProfile);

function reset() {
  password1.value = "";
  password2.value = "";
  form.value?.resetValidation();
}

function cancel() {
  router.back();
}

async function submit() {
  if (form.value?.validate()) {
    const data: UpdatePasswordDto = {
      password: password1.value
    };
    await mainStore.updatePassword(data);
    router.back();
  }
}
</script>

