<template>
  <router-view />
</template>

<script lang="ts" setup>
import { onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useMainStore } from '@/stores/main'

const router = useRouter()
const route = useRoute()
const mainStore = useMainStore()

onMounted(async () => {
  await mainStore.checkLoggedIn()

  if (mainStore.isLoggedIn) {
    if (route.path === '/' || route.path === '/login') {
      router.replace('/main')
    }
  } else {
    if (route.path === '/' || route.path.startsWith('/main')) {
      router.replace('/login')
    }
  }
})
</script>
