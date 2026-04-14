<template>
  <v-snackbar :color="currentNotificationColor" v-model="show" bottom right>
    <v-progress-circular class="ma-2" indeterminate v-if="showProgress" />
    {{ currentNotificationContent }}
    <template v-slot:actions>
      <v-btn text @click="close">Close</v-btn>
    </template>
  </v-snackbar>
</template>

<script lang="ts" setup>
import { ref, computed, watch } from 'vue';
import { useMainStore } from '@/stores/main'; // ✅ Using Pinia store
import type { AppNotification } from '@/modules/main/models';

const show = ref(false);
const showProgress = ref(false);
const currentNotification = ref<AppNotification | null>(null);

const mainStore = useMainStore();

const firstNotification = computed(() => mainStore.notifications[0] || null);

const currentNotificationContent = computed(() => currentNotification.value?.content || '');
const currentNotificationColor = computed(() => currentNotification.value?.color || 'info');

const hide = async () => {
  show.value = false;
  await new Promise<void>((resolve) => setTimeout(resolve, 500));
};

const close = async () => {
  await hide();
  removeCurrentNotification();
};

const removeCurrentNotification = () => {
  if (currentNotification.value) {
    mainStore.notifications = mainStore.notifications.filter(
      (n) => n !== currentNotification.value
    );
  }
};

const setNotification = async (notification: AppNotification | null) => {
  if (show.value) {
    await hide();
  }

  if (notification) {
    currentNotification.value = notification;
    showProgress.value = notification.showProgress || false;
    show.value = true;

    // Auto-remove after timeout
    setTimeout(() => {
      removeCurrentNotification();
    }, 6500);
  } else {
    currentNotification.value = null;
  }
};

watch(firstNotification, async (newNotif, oldNotif) => {
  if (newNotif !== currentNotification.value) {
    await setNotification(newNotif);
  }
});
</script>
