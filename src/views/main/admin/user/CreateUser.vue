<template>
  <v-container fluid>
    <v-toolbar density="compact">
      <v-toolbar-title>Create User</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn @click="cancel" text color="primary">Cancel</v-btn>
        <v-btn @click="reset" text color="primary">Reset</v-btn>
        <v-btn @click="submit" text :disabled="!valid" color="primary">Save</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form v-model="valid" ref="form" lazy-validation>
          <v-text-field label="Name" v-model="name" />
          <v-text-field
            label="E-mail"
            type="email"
            v-model="email"
            :rules="emailRules"
          />

          <v-row align="center">
            <v-col>
              <v-text-field
                type="password"
                label="Set Password"
                v-model="password1"
                :rules="password1Rules"
              />
              <v-text-field
                type="password"
                label="Confirm Password"
                v-model="password2"
                :rules="password2Rules"
              />
            </v-col>
          </v-row>
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { required, email } from '@/utils/validators'
import type { CreateUserDto } from '@/modules/user/types'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const form = ref()
const valid = ref(false)

const name = ref('')
const emailValue = ref('')
const password1 = ref('')
const password2 = ref('')

// Validation Rules
const emailRules = [required, email]
const password1Rules = [required]
const password2Rules = [
  required,
  (v: string) => v === password1.value || 'Password should be the same',
]

// Lifecycle
onMounted(() => {
  reset()
})

// Methods
function reset() {
  name.value = ''
  emailValue.value = ''
  password1.value = ''
  password2.value = ''
  form.value?.resetValidation()
}

function cancel() {
  router.back()
}

async function submit() {
  if (!form.value?.validate()) return

  const newUser: CreateUserDto = {
    name: name.value,
    email: emailValue.value,
    password: password1.value,
  }

  await userStore.createUser(newUser)
  router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''))
}
</script>
