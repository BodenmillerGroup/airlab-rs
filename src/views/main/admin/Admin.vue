<template>
  <router-view />
</template>

<script lang="ts" setup>
import { useMainStore } from '@/stores/main'
import { useRouter, useRoute, onBeforeRouteUpdate } from 'vue-router'
import { onMounted } from 'vue'

const mainStore = useMainStore()
const router = useRouter()
const route = useRoute()

const routeGuardAdmin = async () => {
  if (!mainStore.isAdmin) {
    await router.replace('/main')
  }
}

onMounted(async () => {
  await routeGuardAdmin()
})

onBeforeRouteUpdate(async () => {
  await routeGuardAdmin()
})
</script>
