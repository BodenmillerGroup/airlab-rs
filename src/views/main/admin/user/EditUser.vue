<template>
  <v-container fluid>
    <v-toolbar dense>
      <v-toolbar-title>Edit User</v-toolbar-title>
      <v-spacer />
      <v-toolbar-items>
        <v-btn @click="cancel" text color="primary">Cancel</v-btn>
        <v-btn @click="reset" text color="primary">Reset</v-btn>
        <v-btn @click="submit" text :disabled="!valid" color="primary">Save</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-card class="mt-4 px-4">
      <v-card-text>
        <v-form v-model="valid" ref="form">
          <v-text-field label="Name" v-model="name" />
          <v-text-field label="E-mail" type="email" v-model="emailValue" :rules="emailRules" />
          <v-checkbox label="Active" v-model="isActive" />
          <v-checkbox label="Admin" v-model="isAdmin" />

          <v-row align="center">
            <v-col class="shrink">
              <v-checkbox v-model="setPassword" />
            </v-col>
            <v-col>
              <v-text-field
                :disabled="!setPassword"
                type="password"
                label="Set Password"
                v-model="password1"
                :rules="password1Rules"
              />
              <v-text-field
                v-if="setPassword"
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
import { email, required } from '@/utils/validators'
import type { UpdateUserDto } from '@/modules/user/types'

const route = useRoute()
const router = useRouter()

const userStore = useUserStore()
const userId = Number(route.params.id)

const form = ref()
const valid = ref(true)

const name = ref('')
const emailValue = ref('')
const isActive = ref(false)
const isAdmin = ref(false)

const setPassword = ref(false)
const password1 = ref('')
const password2 = ref('')

const user = computed(() => userStore.getUserById(userId))

const emailRules = [required, email]
const password1Rules = [(v: string) => setPassword.value && !v ? 'Required' : true]
const password2Rules = [
  (v: string) => setPassword.value && !v ? 'Required' : true,
  (v: string) => v !== password1.value ? 'Password should be the same' : true,
]

onMounted(async () => {
  await userStore.getUser(userId)
  reset()
})

function reset() {
  setPassword.value = false
  password1.value = ''
  password2.value = ''
  if (user.value) {
    emailValue.value = user.value.email
    name.value = user.value.name
    isActive.value = user.value.isActive
    isAdmin.value = user.value.isAdmin
  }
  form.value?.resetValidation()
}

function cancel() {
  router.back()
}

async function submit() {
  if (!form.value?.validate()) return

  const data: UpdateUserDto = {
    email: emailValue.value,
    name: name.value,
    password: password1.value,
    isActive: isActive.value,
    isAdmin: isAdmin.value,
  }

  await userStore.updateUser({ id: userId, data })
  router.push(route.path.replace('/create', '').replace(/\/[^/]+\/edit/, ''))
}
</script>
