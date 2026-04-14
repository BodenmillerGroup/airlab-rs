<template>
  <v-main>
    <v-container fluid fill-height>
      <v-row align="center" justify="center">
        <v-col xs="12" sm="8" md="4">
          <v-card elevation="12">
            <v-toolbar dark color="primary">
              <v-toolbar-title>{{ appName }} - Reset Password</v-toolbar-title>
            </v-toolbar>
            <v-card-text>
              <p class="subtitle-3">Enter your new password below</p>
              <v-form ref="form" v-model="valid" @keyup.enter="submit" lazy-validation>
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
              <v-btn @click="clear">Clear</v-btn>
              <v-btn @click="submit" :disabled="!valid">Save</v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { appName } from '@/env';
import { required } from '@/utils/validators';
import { useMainStore } from '@/stores/main';

const route = useRoute();
const router = useRouter();
const mainStore = useMainStore();

const form = ref();
const valid = ref(true);
const password1 = ref('');
const password2 = ref('');

const appNameLocal = appName;

const password1Rules = [required];
const password2Rules = [
  required,
  (v: string) => v === password1.value || 'Password should be the same',
];

const token = computed(() => route.query.token as string | undefined);

function checkToken() {
  if (!token.value) {
    mainStore.addNotification({
      content: 'No token provided in the URL, start a new password recovery',
      color: 'error',
    });
    router.push('/recover-password');
    return false;
  }
  return true;
}

onMounted(() => {
  checkToken();
});

function clear() {
  password1.value = '';
  password2.value = '';
  (form.value as any)?.resetValidation();
}

function cancel() {
  router.push('/');
}

async function submit() {
  if ((form.value as any)?.validate() && checkToken()) {
    await mainStore.resetPassword({
      token: token.value!,
      password: password1.value,
    });
    router.push('/');
  }
}
</script>
