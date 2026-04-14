<template>
  <div>
    <v-navigation-drawer v-model="showDrawer" fixed app width="180">
      <v-list nav density="compact" class="main-drawer-list">
        <template v-if="activeGroupId">
          <v-list-subheader>Group</v-list-subheader>
          <v-list-item :to="`/main/groups/${activeGroupId}/dashboard`">
            <template #prepend><v-icon>mdi-view-dashboard-outline</v-icon></template>
            <v-list-item-title>Dashboard</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/clones`">
            <template #prepend><v-icon>mdi-test-tube</v-icon></template>
            <v-list-item-title>Clones</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/lots`">
            <template #prepend><v-icon>mdi-pound-box-outline</v-icon></template>
            <v-list-item-title>Lots</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/conjugates`">
            <template #prepend><v-icon>mdi-label-percent-outline</v-icon></template>
            <v-list-item-title>Conjugates</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/panels`">
            <template #prepend><v-icon>mdi-clipboard-outline</v-icon></template>
            <v-list-item-title>Panels</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/validations`">
            <template #prepend><v-icon>mdi-ab-testing</v-icon></template>
            <v-list-item-title>Validations</v-list-item-title>
          </v-list-item>

          <v-divider class="my-2" />

          <v-list-subheader>Details</v-list-subheader>
          <v-list-item :to="`/main/groups/${activeGroupId}/proteins`">
            <template #prepend><v-icon>mdi-dna</v-icon></template>
            <v-list-item-title>Proteins</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/tags`">
            <template #prepend><v-icon>mdi-tag-outline</v-icon></template>
            <v-list-item-title>Tags</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/providers`">
            <template #prepend><v-icon>mdi-domain</v-icon></template>
            <v-list-item-title>Providers</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/species`">
            <template #prepend><v-icon>mdi-rabbit</v-icon></template>
            <v-list-item-title>Species</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/storage`">
            <template #prepend><v-icon>mdi-fridge-outline</v-icon></template>
            <v-list-item-title>Storage</v-list-item-title>
          </v-list-item>

          <v-list-item :to="`/main/groups/${activeGroupId}/collections`">
            <template #prepend><v-icon>mdi-view-grid-outline</v-icon></template>
            <v-list-item-title>Collections</v-list-item-title>
          </v-list-item>

          <v-list-item v-if="isGroupAdmin" :to="`/main/groups/${activeGroupId}/members`">
            <template #prepend><v-icon>mdi-account-multiple-outline</v-icon></template>
            <v-list-item-title>Members</v-list-item-title>
          </v-list-item>
        </template>

        <template v-if="isAdmin">
          <v-divider class="my-2" />
          <v-list-subheader>Admin</v-list-subheader>
          <v-list-item to="/main/admin/users">
            <template #prepend><v-icon>mdi-account-outline</v-icon></template>
            <v-list-item-title>Users</v-list-item-title>
          </v-list-item>
          <v-list-item to="/main/admin/groups">
            <template #prepend><v-icon>mdi-account-multiple-outline</v-icon></template>
            <v-list-item-title>Groups</v-list-item-title>
          </v-list-item>
        </template>
      </v-list>
    </v-navigation-drawer>

    <v-app-bar app dense dark color="primary">
      <v-app-bar-nav-icon @click.stop="toggleDrawer" />
      <v-toolbar-title
        @click.stop="$router.push({ name: 'main-groups' })"
        class="toolbar-title"
      >
        {{ appName }}
      </v-toolbar-title>
      <v-spacer />
      <v-menu bottom left offset-y>
        <template v-slot:activator="{ props }">
          <v-btn icon v-bind="props">
            <v-icon>mdi-dots-vertical</v-icon>
          </v-btn>
        </template>
        <v-list>
          <v-list-item to="/main/profile/view">
            <v-list-item-title>Profile</v-list-item-title>
            <v-list-item-action>
              <v-icon>mdi-account</v-icon>
            </v-list-item-action>
          </v-list-item>
          <v-list-item @click="logout">
            <v-list-item-title>Logout</v-list-item-title>
            <v-list-item-action>
              <v-icon>mdi-logout-variant</v-icon>
            </v-list-item-action>
          </v-list-item>
        </v-list>
      </v-menu>

      <template v-slot:extension>
<ToolbarProgressBar
        :processing="processing"
        :progress="processingProgress"
        :indeterminate="false"
        color="light-blue lighten-2"

      />
</template>
    </v-app-bar>

    <v-main>
      <router-view />
    </v-main>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { useMainStore } from '@/stores/main';
import { useGroupStore } from '@/stores/group';
import { useRouter } from 'vue-router';
import { useRoute } from "vue-router";
import { watch } from "vue";
import ToolbarProgressBar from '@/components/ToolbarProgressBar.vue';
import { appName } from '@/env';
import { BroadcastManager } from '@/utils/BroadcastManager';
import { storeToRefs } from 'pinia'


const mainStore = useMainStore();
const groupStore = useGroupStore();
const router = useRouter();
const route = useRoute();

watch(
  () => route.params.groupId,
  (newGroupId) => {
    const id = Number(newGroupId);
    if (!isNaN(id)) {
      groupStore.activeGroupId = id;
    }
  },
  { immediate: true } // fire on load
);

const showDrawer = computed({
  get: () => mainStore.dashboardShowDrawer,
  set: (val: boolean) => mainStore.setDashboardShowDrawer(val),
});

function toggleDrawer() {
  mainStore.setDashboardShowDrawer(!mainStore.dashboardShowDrawer);
}

const isAdmin = computed(() => mainStore.isAdmin);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const { activeGroupId } = storeToRefs(groupStore);
const processing = computed(() => mainStore.processing);
const processingProgress = computed(() => mainStore.processingProgress);

async function logout() {
  await mainStore.userLogOut();
}

// Lifecycle
onMounted(() => {
  BroadcastManager.init();
});
onBeforeUnmount(() => {
  BroadcastManager.close();
});

// Navigation guards
const routeGuardMain = (to, from, next) => {
  if (to.path === '/main') {
    next('/main/groups');
  } else {
    next();
};
};

</script>

<script lang="ts">
export default {
  beforeRouteEnter(to, from, next) {
    if (to.path === '/main') {
      next('/main/groups');
    } else {
      next();
    }
  },
  beforeRouteUpdate(to, from, next) {
    if (to.path === '/main') {
      next('/main/groups');
    } else {
      next();
    }
  },
};
</script>

<style scoped>
.toolbar-title {
  cursor: pointer;
}

:deep(.main-drawer-list .v-list-item) {
  padding-inline-start: 0 !important;
}
</style>

<style>
.link {
  text-decoration: none;
}
</style>
