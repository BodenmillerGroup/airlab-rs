<template>
  <v-container fluid>
    <v-card class="ma-4 pa-4">
      <v-card-title>
        <div class="text-h5">User Profile</div>
      </v-card-title>
      <v-card-text>
        <div class="my-6">
          <div class="text-subtitle-1 text--lighten-2">Name</div>
          <div class="text-h6 text--darken-2" v-if="userProfile?.name">
            {{ userProfile.name }}
          </div>
          <div class="text-h6 text--darken-2" v-else>-----</div>
        </div>
        <div class="my-4">
          <div class="text-subtitle-1 text--lighten-2">Email</div>
          <div class="text-h6 text--darken-2" v-if="userProfile?.email">
            {{ userProfile.email }}
          </div>
          <div class="text-h6 text--darken-2" v-else>-----</div>
        </div>
      </v-card-text>
      <v-card-actions>
        <v-btn to="/main/profile/edit">Edit</v-btn>
        <v-btn to="/main/profile/password">Change password</v-btn>
        <MfaSetupDialog v-model:modelValue="showMfaDialog" :email="userProfile?.email" />
        <v-btn color="info" @click="showMfaDialog = true">Enable MFA</v-btn>
        <v-spacer />
        <v-btn color="secondary" @click="resetSettings">Reset Settings</v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { useMainStore } from "@/stores/main";
import { useSettingsStore } from "@/stores/settings";
import { ref, computed } from "vue";

const mainStore = useMainStore();
const settingsStore = useSettingsStore();

const userProfile = computed(() => mainStore.userProfile);


import MfaSetupDialog from "@/components/MfaSetupDialog.vue";

const showMfaDialog = ref(false);


function resetSettings() {
  if (confirm("Reset all settings?")) {
    settingsStore.resetSettings();
  }
}
</script>
