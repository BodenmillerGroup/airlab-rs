<template>
  <div>
  <v-main>
    <v-container fluid fill-height>
      <v-row align="center" justify="center">
        <v-col xs="12" sm="8" md="4">
          <v-card elevation="12">
            <v-toolbar dark color="primary">
              <v-toolbar-title>{{ appName }}</v-toolbar-title>
              <v-spacer />
            </v-toolbar>
            <v-card-text>
              <v-form @keyup.enter="submit">
                <v-text-field
                  v-model="email"
                  prepend-icon="mdi-account"
                  name="email"
                  label="Email"
                  type="text"
                  :rules="emailRules"
                  @keyup.enter="submit"
                />
                <v-text-field
                  id="password"
                  v-model="password"
                  prepend-icon="mdi-lock"
                  name="password"
                  label="Password"
                  type="password"
                  @keyup.enter="submit"
                />
              </v-form>
              <div v-if="loginError">
                <v-alert :value="loginError" transition="fade-transition" type="error">
                  Incorrect email or password
                </v-alert>
              </div>
              <v-row>
                <v-col class="text-caption text-right py-0">
                  <router-link to="/recover-password">Recover password</router-link>
                </v-col>
              </v-row>
              <v-row>
                <v-col class="text-caption text-right py-0">
                  <router-link to="/signup">Register an account</router-link>
                </v-col>
              </v-row>
            </v-card-text>
            <v-card-actions>
              <v-spacer />
              <v-btn @click.prevent="submit">Login</v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
  <OtpDialog v-model="store.mfaPending" @close="store.mfaPending = false" />
  </div>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue';
import { useMainStore } from '@/stores/main';
import { email as emailRule, required } from '@/utils/validators';
import { appName } from '@/env';
import OtpDialog from '@/components/MfaDialog.vue';

const store = useMainStore();

const email = ref('');
const password = ref('');
const emailRules = [required, emailRule];
const loginError = computed(() => store.error); // reuse error handling

function submit() {
  store.logIn({ username: email.value, password: password.value });
}
</script>
