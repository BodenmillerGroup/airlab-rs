<template>
  <v-main>
    <v-container fluid fill-height>
      <v-row align="center" justify="center">
        <v-col xs="12" sm="8" md="4">
          <v-card elevation="12">
            <v-toolbar dark color="primary">
              <v-toolbar-title>Create {{ appName }} Account</v-toolbar-title>
              <v-spacer />
            </v-toolbar>
            <v-card-text>
              <v-form @keyup.enter="submit" v-model="valid" ref="form" @submit.prevent="" lazy-validation>
                <v-text-field
                  type="email"
                  v-model="email"
                  prepend-icon="mdi-account"
                  label="Email"
                  :rules="emailRules"
                />
                <v-text-field
                  type="text"
                  label="Name"
                  prepend-icon="mdi-account"
                  v-model="name"
                />
                <v-text-field
                  type="password"
                  ref="password"
                  label="Password"
                  :rules="password1Rules"
                  v-model="password1"
                  prepend-icon="mdi-lock"
                />
                <v-text-field
                  type="password"
                  label="Confirm Password"
                  :rules="password2Rules"
                  v-model="password2"
                  prepend-icon="mdi-lock"
                />
              </v-form>
              <v-row>
                <v-col class="text-caption text-right py-0">
                  <router-link to="/login">Already have an account?</router-link>
                </v-col>
              </v-row>
            </v-card-text>
            <v-card-actions>
              <v-spacer />
              <v-btn @click.prevent="submit">Sign Up</v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue';
import { useUserStore } from '@/stores/user';
import { useMainStore } from '@/stores/main';
import { appName } from '@/env';
import { required, email as emailRule } from '@/utils/validators';

const userStore = useUserStore();
const mainStore = useMainStore();

const valid = ref(true);
const form = ref();
const email = ref('');
const name = ref('');
const password1 = ref('');
const password2 = ref('');

const emailRules = [required, emailRule];
const password1Rules = [required];
const password2Rules = [
  required,
  (v: string) => v === password1.value || 'Password should be the same',
];

const submit = async () => {
  const isValid = form.value?.validate?.();
  if (isValid) {
    const userExists = await checkUserExists();
    if (!userExists) {
      await userStore.signUp({
        email: email.value,
        name: name.value,
        password: password1.value,
      });
    }
  }
};

const checkUserExists = async (): Promise<boolean> => {
  const exists = await userStore.checkUserExists(email.value);
  if (exists) {
    mainStore.addNotification({
      content: 'User with this email already exists',
      color: 'warning',
    });
  }
  return exists;
};

const reset = () => {
  email.value = '';
  name.value = '';
  password1.value = '';
  password2.value = '';
  form.value?.resetValidation?.();
};

</script>
