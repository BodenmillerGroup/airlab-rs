import RouterComponent from "@/components/RouterComponent.vue";
import { createRouter, createWebHistory } from "vue-router";
import { useMainStore } from '@/stores/main';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      component: () => import(/* webpackChunkName: "start" */ "@/views/main/Start.vue"),
      children: [
        {
          path: "login",
          component: () => import(/* webpackChunkName: "login" */ "@/views/Login.vue"),
        },
        {
          path: "signup",
          component: () => import(/* webpackChunkName: "login" */ "@/views/SignUp.vue"),
        },
        {
          path: "recover-password",
          component: () => import(/* webpackChunkName: "recover-password" */ "@/views/PasswordRecovery.vue"),
        },
        {
          path: "reset-password",
          component: () => import(/* webpackChunkName: "reset-password" */ "@/views/ResetPassword.vue"),
        },
        {
          path: "main",
          component: () => import(/* webpackChunkName: "main" */ "@/views/main/Main.vue"),
          redirect: "/main/groups/",
          meta: { requiresAuth: true },
          children: [
            {
              path: "groups",
              name: "main-groups",
              component: () => import(/* webpackChunkName: "main-groups" */ "@/views/main/GroupsView.vue"),
            },
            {
              path: "groups/:groupId",
              name: "main-group",
              component: () => import(/* webpackChunkName: "main-group" */ "@/views/main/group/GroupView.vue"),
              redirect: (to) => {
                return `/main/groups/${to.params.groupId}/dashboard`;
              },
              children: [
                {
                  path: "dashboard",
                  name: "main-group-dashboard",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-dashboard" */ "@/views/main/group/dashboard/DashboardView.vue"
                    ),
                },
                {
                  path: "proteins",
                  name: "main-group-proteins",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-proteins" */ "@/views/main/group/proteins/ProteinsListView.vue"
                    ),
                },
                {
                  path: "proteins/create",
                  name: "main-group-proteins-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-proteins-create" */ "@/views/main/group/proteins/CreateProtein.vue"
                    ),
                },
                {
                  path: "proteins/:id/edit",
                  name: "main-group-proteins-edit",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-proteins-edit" */ "@/views/main/group/proteins/EditProtein.vue"
                    ),
                },

                {
                  path: "clones",
                  name: "main-group-clones",
                  component: () =>
                    import(/* webpackChunkName: "main-group-clones" */ "@/views/main/group/clones/ClonesListView.vue"),
                },
                {
                  path: "clones/create",
                  name: "main-group-clones-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-clones-create" */ "@/views/main/group/clones/CreateClone.vue"
                    ),
                },
                {
                  path: "clones/:id/edit",
                  name: "main-group-clones-edit",
                  component: () =>
                    import(/* webpackChunkName: "main-group-clones-edit" */ "@/views/main/group/clones/EditClone.vue"),
                },
                {
                  path: "panels",
                  name: "main-group-panels",
                  component: () =>
                    import(/* webpackChunkName: "main-group-panels" */ "@/views/main/group/panels/PanelsListView.vue"),
                },
                {
                  path: "panels/create",
                  name: "main-group-panels-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-panels-create" */ "@/views/main/group/panels/CreatePanel.vue"
                    ),
                },
                {
                  path: "panels/:id/edit",
                  name: "main-group-panels-edit",
                  component: () =>
                    import(/* webpackChunkName: "main-group-panels-edit" */ "@/views/main/group/panels/EditPanel.vue"),
                },
                {
                  path: "panels/:id",
                  name: "main-group-panels-view",
                  component: () =>
                    import(/* webpackChunkName: "main-group-panels-view" */ "@/views/main/group/panels/ViewPanel.vue"),
                },
                {
                  path: "conjugates",
                  name: "main-group-conjugates",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-conjugates" */ "@/views/main/group/conjugates/ConjugatesListView.vue"
                    ),
                },
                {
                  path: "conjugates/create",
                  name: "main-group-conjugates-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-conjugates-create" */ "@/views/main/group/conjugates/CreateConjugate.vue"
                    ),
                },
                {
                  path: "conjugates/:id/edit",
                  name: "main-group-conjugates-edit",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-conjugates-edit" */ "@/views/main/group/conjugates/EditConjugate.vue"
                    ),
                },

                {
                  path: "validations",
                  name: "main-group-validations",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-validations" */ "@/views/main/group/validations/ValidationsListView.vue"
                    ),
                },
                {
                  path: "validations/create",
                  name: "main-group-validations-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-validations-create" */ "@/views/main/group/validations/CreateValidation.vue"
                    ),
                },
                {
                  path: "validations/:id/edit",
                  name: "main-group-validations-edit",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-validations-edit" */ "@/views/main/group/validations/EditValidation.vue"
                    ),
                },

                {
                  path: "lots",
                  name: "main-group-lots",
                  component: () =>
                    import(/* webpackChunkName: "main-group-lots" */ "@/views/main/group/lots/LotsListView.vue"),
                },
                {
                  path: "lots/create",
                  name: "main-group-lots-create",
                  component: () =>
                    import(/* webpackChunkName: "main-group-lots-create" */ "@/views/main/group/lots/CreateLot.vue"),
                },
                {
                  path: "lots/:id/edit",
                  name: "main-group-lots-edit",
                  component: () =>
                    import(/* webpackChunkName: "main-group-lots-edit" */ "@/views/main/group/lots/EditLot.vue"),
                },

                {
                  path: "tags",
                  name: "main-group-tags",
                  component: () =>
                    import(/* webpackChunkName: "main-group-tags" */ "@/views/main/group/tags/TagsListView.vue"),
                },
                {
                  path: "tags/create",
                  name: "main-group-tags-create",
                  component: () =>
                    import(/* webpackChunkName: "main-group-tags-create" */ "@/views/main/group/tags/CreateTag.vue"),
                },
                {
                  path: "tags/:id/edit",
                  name: "main-group-tags-edit",
                  component: () =>
                    import(/* webpackChunkName: "main-group-tags-edit" */ "@/views/main/group/tags/EditTag.vue"),
                },

                {
                  path: "species",
                  name: "main-group-species",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-species" */ "@/views/main/group/species/SpeciesListView.vue"
                    ),
                },
                {
                  path: "species/create",
                  name: "main-group-species-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-species-create" */ "@/views/main/group/species/CreateSpecies.vue"
                    ),
                },
                {
                  path: "species/:id/edit",
                  name: "main-group-species-edit",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-species-edit" */ "@/views/main/group/species/EditSpecies.vue"
                    ),
                },
                {
                  path: "storage",
                  name: "main-group-storage",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-storage" */ "@/views/main/group/storage/StorageListView.vue"
                    ),
                },
                {
                  path: "storage/create",
                  name: "main-group-storage-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-storage-create" */ "@/views/main/group/storage/CreateStorage.vue"
                    ),
                },
                {
                  path: "storage/:id/edit",
                  name: "main-group-storage-edit",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-storage-edit" */ "@/views/main/group/storage/EditStorage.vue"
                    ),
                },
                {
                  path: "collections",
                  name: "main-group-collections",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-collections" */ "@/views/main/group/collection/CollectionListView.vue"
                    ),
                },
                {
                  path: "collections/create",
                  name: "main-group-collections-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-collections-create" */ "@/views/main/group/collection/CreateCollection.vue"
                    ),
                },
                {
                  path: "collections/:id/edit",
                  name: "main-group-collections-edit",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-collections-edit" */ "@/views/main/group/collection/EditCollection.vue"
                    ),
                },
                {
                  path: "providers",
                  name: "main-group-providers",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-providers" */ "@/views/main/group/providers/ProvidersListView.vue"
                    ),
                },
                {
                  path: "providers/create",
                  name: "main-group-providers-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-providers-create" */ "@/views/main/group/providers/CreateProvider.vue"
                    ),
                },
                {
                  path: "providers/:id/edit",
                  name: "main-group-providers-edit",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-providers-edit" */ "@/views/main/group/providers/EditProvider.vue"
                    ),
                },
                {
                  path: "members",
                  name: "main-group-members",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-members" */ "@/views/main/group/members/MembersListView.vue"
                    ),
                },
                {
                  path: "members/create",
                  name: "main-group-members-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-members-create" */ "@/views/main/group/members/CreateMember.vue"
                    ),
                },
                {
                  path: "members/:id/edit",
                  name: "main-group-members-edit",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-group-members-edit" */ "@/views/main/group/members/EditMember.vue"
                    ),
                },
              ],
            },
            {
              path: "profile",
              component: RouterComponent,
              redirect: "profile/view",
              children: [
                {
                  path: "view",
                  component: () =>
                    import(/* webpackChunkName: "main-profile" */ "@/views/main/profile/UserProfile.vue"),
                },
                {
                  path: "edit",
                  component: () =>
                    import(/* webpackChunkName: "main-profile-edit" */ "@/views/main/profile/UserProfileEdit.vue"),
                },
                {
                  path: "password",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-profile-password" */ "@/views/main/profile/UserProfileEditPassword.vue"
                    ),
                },
              ],
            },
            {
              path: "admin",
              component: () => import(/* webpackChunkName: "main-admin" */ "@/views/main/admin/Admin.vue"),
              redirect: "/main/admin/users",
              //redirect: "admin/users",
              children: [
                {
                  path: "users",
                  component: () =>
                    import(/* webpackChunkName: "main-admin-users" */ "@/views/main/admin/user/UsersListView.vue"),
                },
                {
                  path: "users/:id/edit",
                  name: "main-admin-users-edit",
                  component: () =>
                    import(/* webpackChunkName: "main-admin-users-edit" */ "@/views/main/admin/user/EditUser.vue"),
                },
                {
                  path: "users/create",
                  name: "main-admin-users-create",
                  component: () =>
                    import(/* webpackChunkName: "main-admin-user-create" */ "@/views/main/admin/user/CreateUser.vue"),
                },

                {
                  path: "groups",
                  component: () =>
                    import(/* webpackChunkName: "main-admin-groups" */ "@/views/main/admin/group/GroupsListView.vue"),
                },
                {
                  path: "groups/create",
                  name: "main-admin-groups-create",
                  component: () =>
                    import(
                      /* webpackChunkName: "main-admin-groups-create" */ "@/views/main/admin/group/CreateGroup.vue"
                    ),
                },
                {
                  path: "groups/:id/edit",
                  name: "main-admin-groups-edit",
                  component: () =>
                    import(/* webpackChunkName: "main-admin-groups-edit" */ "@/views/main/admin/group/EditGroup.vue"),
                },
              ],
            },
          ]
        },
      ],
    },
    {
      path: "/*",
      redirect: "/",
    },
  ],
});

router.beforeEach(async (to) => {
  const mainStore = useMainStore();
  const requiresAuth = to.matched.some((record) => record.meta.requiresAuth);

  if (requiresAuth || to.path === '/login' || to.path === '/') {
    await mainStore.checkLoggedIn();
  }

  if (requiresAuth && !mainStore.isLoggedIn) {
    await mainStore.removeLogIn();
    return { path: '/login', query: { redirect: to.fullPath } };
  }

  if ((to.path === '/login' || to.path === '/') && mainStore.isLoggedIn) {
    return { path: '/main' };
  }

  return true;
});


export default router;
