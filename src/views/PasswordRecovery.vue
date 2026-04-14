<template>
  <v-main>
    <v-container fluid fill-height>
      <v-row align="center" justify="center">
        <v-col xs="12" sm="8" md="4">
          <v-card elevation="12">
            <v-toolbar dark color="primary">
              <v-toolbar-title>{{ appName }} - Password Recovery</v-toolbar-title>
            </v-toolbar>
            <v-card-text>
              <p class="subtitle-3">
                A password recovery email will be sent to the registered account
              </p>
              <v-form v-model="valid" ref="form" @keyup.enter="submit" lazy-validation>
                <v-text-field
                  label="Email"
                  type="text"
                  prepend-icon="mdi-account"
                  v-model="email"
                  :rules="emailRules"
                />
              </v-form>
            </v-card-text>
            <v-card-actions>
              <v-spacer />
              <v-btn @click="cancel">Cancel</v-btn>
              <v-btn @click.prevent="submit" :disabled="!valid">Recover Password</v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { appName } from '@/env';
import { required, email as emailRule } from '@/utils/validators';
import { useMainStore } from '@/stores/main';

const router = useRouter();
const mainStore = useMainStore();

const appNameConst = appName;

const email = ref('');
const valid = ref(true);
const form = ref();

const emailRules = [required, emailRule];

function cancel() {
  router.back();
}

async function submit() {
  if ((form.value as any)?.validate()) {
    await mainStore.passwordRecovery(email.value);
  }
}
</script>
