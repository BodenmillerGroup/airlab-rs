<template>
  <v-app id="app">
    <v-main v-if="loggedIn === null">
      <v-container fill-height>
        <v-row align="center" justify="center">
          <v-col>
            <div class="text-center">
              <div class="text-h5 my-12">Loading...</div>
              <v-progress-circular size="100" indeterminate color="primary" />
            </div>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
    <router-view v-else />
    <NotificationsManager />
  </v-app>
</template>

<script lang="ts" setup>
import { onMounted, computed } from 'vue';
import { useMainStore } from '@/stores/main';
import { useResponsiveStore } from '@/stores/responsive';
import NotificationsManager from '@/components/NotificationsManager.vue';
import { reportFrontendError } from '@/telemetry/errors'

const mainStore = useMainStore();
const responsiveStore = useResponsiveStore();

const loggedIn = computed(() => mainStore.isLoggedIn);

// Call on startup
mainStore.checkLoggedIn();

function handleResize() {
  responsiveStore.setResponsive({
    width: window.innerWidth,
    height: window.innerHeight,
  });
}

onMounted(() => {
  // Initial setup
  handleResize();

  // Setup event listener
  window.addEventListener('resize', handleResize);
  window.addEventListener("unhandledrejection", e =>
    reportFrontendError(e.reason, { type: "unhandledrejection" })
  );
  window.onerror = (msg, src, line, col, err) => {
    reportFrontendError(err ?? msg, { src, line, col })
  }
});
</script>

