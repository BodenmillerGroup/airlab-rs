import { computed, reactive, ref, toRefs } from "vue"
import { describe, it, expect, beforeEach, afterEach, vi } from "vitest"
import { flushPromises, shallowMount, type VueWrapper } from "@vue/test-utils"

const mocks = vi.hoisted(() => ({
  groupStore: null as any,
  mainStore: null as any,
  userStore: null as any,
  collectionStore: null as any,
  storageStore: null as any,
  cloneStore: null as any,
  conjugateStore: null as any,
  lotStore: null as any,
  memberStore: null as any,
  panelStore: null as any,
  proteinStore: null as any,
  providerStore: null as any,
  speciesStore: null as any,
  tagStore: null as any,
  validationStore: null as any,
}))

vi.mock("vue-router", () => ({
  useRoute: () => ({ params: { groupId: "42" } }),
}))

vi.mock("pinia", async () => {
  const actual = await vi.importActual<typeof import("pinia")>("pinia")
  return {
    ...actual,
    storeToRefs: (store: Record<string, unknown>) => toRefs(store),
  }
})

vi.mock("@/stores/group", async () => {
  const { reactive } = await import("vue")
  if (!mocks.groupStore) {
    mocks.groupStore = reactive({
      activeGroupId: 42,
      isGroupAdmin: true,
      groups: [{ id: 1, name: "Group A" }],
      getGroups: vi.fn().mockResolvedValue([]),
      importGroupData: vi.fn().mockResolvedValue(undefined),
      importAllData: vi.fn().mockResolvedValue(undefined),
      exportGroupData: vi.fn().mockResolvedValue(undefined),
      exportAllData: vi.fn().mockResolvedValue(undefined),
    })
  }
  return { useGroupStore: () => mocks.groupStore }
})

vi.mock("@/stores/main", async () => {
  const { reactive } = await import("vue")
  if (!mocks.mainStore) {
    mocks.mainStore = reactive({
      isAdmin: true,
    })
  }
  return { useMainStore: () => mocks.mainStore }
})

vi.mock("@/stores/user", async () => {
  const { reactive, ref } = await import("vue")
  if (!mocks.userStore) {
    mocks.userStore = reactive({
      users: [{ id: 1, email: "demo1@uzh.ch", name: "Demo One", isActive: true, isAdmin: false }],
      loading: false,
      total: 1,
      limit: 15,
      page: 1,
      getUsers: vi.fn().mockResolvedValue([]),
      getUserById: vi.fn((id: number) => ({ id, name: `User ${id}` })),
    })
  }
  return { useUserStore: () => mocks.userStore }
})

vi.mock("@/stores/collection", async () => {
  const { reactive } = await import("vue")
  if (!mocks.collectionStore) {
    mocks.collectionStore = reactive({
      collections: [{ id: 1, name: "Collection A" }],
      listCollections: [{ id: 1, name: "Collection A" }],
      loading: false,
      total: 1,
      limit: 15,
      page: 1,
      getCollections: vi.fn().mockResolvedValue([]),
      loadListQuery: vi.fn().mockResolvedValue([]),
      reloadListQuery: vi.fn().mockResolvedValue([]),
      updatePage: vi.fn().mockResolvedValue(undefined),
      getCollection: vi.fn((id: number) => ({ id, name: `Collection ${id}` })),
    })
  }
  return { useCollectionStore: () => mocks.collectionStore }
})

vi.mock("@/stores/storage", async () => {
  const { reactive } = await import("vue")
  if (!mocks.storageStore) {
    mocks.storageStore = reactive({
      storages: [{ id: 1, name: "Freezer A" }],
      listStorages: [{ id: 1, name: "Freezer A" }],
      loading: false,
      total: 1,
      limit: 15,
      page: 1,
      getStorages: vi.fn().mockResolvedValue([]),
      loadListQuery: vi.fn().mockResolvedValue([]),
      reloadListQuery: vi.fn().mockResolvedValue([]),
      updatePage: vi.fn().mockResolvedValue(undefined),
    })
  }
  return { useStorageStore: () => mocks.storageStore }
})

function buildEntityStore(key: keyof typeof mocks) {
  if (!mocks[key]) {
    mocks[key] = reactive({
      loading: false,
      total: 1,
      limit: 15,
      page: 1,
      searchstr: "",
      species: [{ id: 1, name: "Human" }],
      getGroupSpecies: vi.fn().mockResolvedValue(undefined),
      getClone: vi.fn((id: number) => ({ id, name: `Clone ${id}` })),
      getProvider: vi.fn((id: number) => ({ id, name: `Provider ${id}` })),
      getCollection: vi.fn((id: number) => ({ id, name: `Collection ${id}` })),
      getMemberById: vi.fn((id: number) => ({ id, userId: id })),
      getUserById: vi.fn((id: number) => ({ id, name: `User ${id}` })),
      deleteProvider: vi.fn().mockResolvedValue(undefined),
      deleteTag: vi.fn().mockResolvedValue(undefined),
      removeMember: vi.fn().mockResolvedValue(undefined),
      updateCloneArchiveState: vi.fn().mockResolvedValue(undefined),
      updateLotArchiveState: vi.fn().mockResolvedValue(undefined),
      updateLotStatus: vi.fn().mockResolvedValue(undefined),
      updateLotStatusAndNumber: vi.fn().mockResolvedValue(undefined),
      duplicatePanel: vi.fn().mockResolvedValue(undefined),
      updatePanelArchiveState: vi.fn().mockResolvedValue(undefined),
      updateSearch: vi.fn(),
    })
  }
  return mocks[key]
}

vi.mock("@/stores/clone", () => ({ useCloneStore: () => buildEntityStore("cloneStore") }))
vi.mock("@/stores/conjugate", () => ({ useConjugateStore: () => buildEntityStore("conjugateStore") }))
vi.mock("@/stores/lot", () => ({ useLotStore: () => buildEntityStore("lotStore") }))
vi.mock("@/stores/member", () => ({ useMemberStore: () => buildEntityStore("memberStore") }))
vi.mock("@/stores/panel", () => ({ usePanelStore: () => buildEntityStore("panelStore") }))
vi.mock("@/stores/protein", () => ({ useProteinStore: () => buildEntityStore("proteinStore") }))
vi.mock("@/stores/provider", () => ({ useProviderStore: () => buildEntityStore("providerStore") }))
vi.mock("@/stores/species", () => ({ useSpeciesStore: () => buildEntityStore("speciesStore") }))
vi.mock("@/stores/tag", () => ({ useTagStore: () => buildEntityStore("tagStore") }))
vi.mock("@/stores/validation", () => ({ useValidationStore: () => buildEntityStore("validationStore") }))

function buildComposable(items = [{ id: 1 }]) {
  return {
    items: ref(items),
    loading: ref(false),
    reload: vi.fn(),
    cloneValidationMap: computed(() => ({})),
    lotValidationMap: computed(() => ({})),
  }
}

vi.mock("@/composables/useClones", () => ({ useClones: () => buildComposable() }))
vi.mock("@/composables/useConjugates", () => ({ useConjugates: () => buildComposable() }))
vi.mock("@/composables/useLots", () => ({ useLots: () => buildComposable() }))
vi.mock("@/composables/useMembers", () => ({ useMembers: () => buildComposable() }))
vi.mock("@/composables/usePanels", () => ({ usePanels: () => buildComposable() }))
vi.mock("@/composables/useProteins", () => ({ useProteins: () => buildComposable() }))
vi.mock("@/composables/useProviders", () => ({ useProviders: () => buildComposable() }))
vi.mock("@/composables/useSpecies", () => ({ useSpecies: () => buildComposable() }))
vi.mock("@/composables/useTags", () => ({ useTags: () => buildComposable() }))
vi.mock("@/composables/useValidations", () => ({ useValidations: () => buildComposable() }))

import GroupsListView from "@/views/main/admin/group/GroupsListView.vue"
import UsersListView from "@/views/main/admin/user/UsersListView.vue"
import ClonesListView from "@/views/main/group/clones/ClonesListView.vue"
import CollectionListView from "@/views/main/group/collection/CollectionListView.vue"
import ConjugatesListView from "@/views/main/group/conjugates/ConjugatesListView.vue"
import LotsListView from "@/views/main/group/lots/LotsListView.vue"
import MembersListView from "@/views/main/group/members/MembersListView.vue"
import PanelsListView from "@/views/main/group/panels/PanelsListView.vue"
import ProteinsListView from "@/views/main/group/proteins/ProteinsListView.vue"
import ProvidersListView from "@/views/main/group/providers/ProvidersListView.vue"
import SpeciesListView from "@/views/main/group/species/SpeciesListView.vue"
import StorageListView from "@/views/main/group/storage/StorageListView.vue"
import TagsListView from "@/views/main/group/tags/TagsListView.vue"
import ValidationsListView from "@/views/main/group/validations/ValidationsListView.vue"

const global = {
  stubs: {
    RouterLink: {
      props: ["to"],
      template: `<a :data-to="typeof to === 'string' ? to : JSON.stringify(to)"><slot /></a>`,
    },
    FilterSummary: { template: `<div data-test="filter-summary">Filter Summary</div>` },
    FilterField: {
      props: ["keyName"],
      template: `<div data-test="filter-field">{{ keyName }}</div>`,
    },
    "v-col": { template: `<div><slot /></div>` },
    "v-toolbar": { template: `<div><slot /></div>` },
    "v-toolbar-title": { template: `<h1><slot /></h1>` },
    "v-toolbar-items": { template: `<div><slot /></div>` },
    "v-spacer": { template: `<span />` },
    "v-btn": {
      props: ["to"],
      template: `<button :data-to="typeof to === 'string' ? to : JSON.stringify(to)"><slot /></button>`,
    },
    "v-card": { template: `<section><slot /></section>` },
    "v-card-title": { template: `<div><slot /></div>` },
    "v-card-text": { template: `<div><slot /></div>` },
    "v-text-field": { template: `<input />` },
    "v-data-table": { template: `<div data-test="data-table"><slot /></div>` },
    "v-data-table-server": { template: `<div data-test="data-table-server"><slot /></div>` },
    "v-expansion-panels": { template: `<div><slot /></div>` },
    "v-expansion-panel": { template: `<div><slot /></div>` },
    "v-expansion-panel-title": { template: `<div><slot /></div>` },
    "v-expansion-panel-text": { template: `<div><slot /></div>` },
    "v-menu": { template: `<div><slot /><slot name="activator" :props="{}" /></div>` },
    "v-list": { template: `<div><slot /></div>` },
    "v-list-item": { template: `<div><slot /><slot name="prepend" /></div>` },
    "v-list-item-title": { template: `<span><slot /></span>` },
    "v-icon": { template: `<i><slot /></i>` },
    "v-tooltip": { template: `<div><slot /><slot name="activator" :props="{}" /></div>` },
    "v-navigation-drawer": { template: `<aside><slot /></aside>` },
    "v-divider": { template: `<hr />` },
    "v-select": { template: `<select />` },
    "v-switch": {
      props: ["label"],
      template: `<label><input type="checkbox" />{{ label }}</label>`,
    },
    "v-chip": { template: `<span><slot /></span>` },
    "v-chip-group": { template: `<div><slot /></div>` },
  },
}

function findButtonByText(wrapper: VueWrapper<any>, text: string) {
  return wrapper
    .findAll("button")
    .find((button) => button.text().includes(text))
}

async function mountView(component: unknown) {
  const wrapper = shallowMount(component as any, { global })
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.spyOn(console, "log").mockImplementation(() => undefined)
  mocks.mainStore.isAdmin = true
  mocks.groupStore.activeGroupId = 42
  mocks.groupStore.isGroupAdmin = true

  mocks.groupStore.getGroups.mockClear()
  mocks.userStore.getUsers.mockClear()
  mocks.collectionStore.getCollections.mockClear()
  mocks.collectionStore.loadListQuery.mockClear()
  mocks.collectionStore.reloadListQuery.mockClear()
  mocks.storageStore.getStorages.mockClear()
  mocks.storageStore.loadListQuery.mockClear()
  mocks.storageStore.reloadListQuery.mockClear()
})

afterEach(() => {
  vi.restoreAllMocks()
})

describe("ListView templates", () => {
  it("renders GroupsListView admin actions and table", async () => {
    const wrapper = await mountView(GroupsListView)

    expect(wrapper.text()).toContain("Groups")
    expect(wrapper.text()).toContain("Export All Data")
    expect(wrapper.text()).toContain("Import All Data")
    expect(wrapper.text()).toContain("Import Group")
    expect(findButtonByText(wrapper, "Create Group")?.attributes("data-to")).toBe("/main/admin/groups/create")
    expect(wrapper.find('[data-test="data-table"]').exists()).toBe(true)
    expect(mocks.groupStore.getGroups).toHaveBeenCalledOnce()
  })

  it("renders UsersListView admin action and table", async () => {
    const wrapper = await mountView(UsersListView)

    expect(wrapper.text()).toContain("Users")
    expect(findButtonByText(wrapper, "Create User")?.attributes("data-to")).toBe("/main/admin/users/create")
    expect(wrapper.find('[data-test="data-table"]').exists()).toBe(true)
    expect(mocks.userStore.getUsers).toHaveBeenCalledOnce()
  })

  it("renders ClonesListView toolbar, filters, and table shell", async () => {
    const wrapper = await mountView(ClonesListView)

    expect(wrapper.text()).toContain("Clones")
    expect(wrapper.text()).toContain("Export CSV")
    expect(findButtonByText(wrapper, "Create Clone")?.attributes("data-to")).toBe("/main/groups/42/clones/create")
    expect(wrapper.find("input").exists()).toBe(true)
    expect(wrapper.find('[data-test="filter-summary"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })

  it("renders CollectionListView add action and table", async () => {
    const wrapper = await mountView(CollectionListView)

    expect(wrapper.text()).toContain("Collections")
    expect(findButtonByText(wrapper, "Add Collection")?.attributes("data-to")).toBe("/main/groups/42/collections/create")
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
    expect(mocks.collectionStore.loadListQuery).toHaveBeenCalledOnce()
  })

  it("renders ConjugatesListView toolbar, filters, and table shell", async () => {
    const wrapper = await mountView(ConjugatesListView)

    expect(wrapper.text()).toContain("Conjugates")
    expect(findButtonByText(wrapper, "Create Conjugate")?.attributes("data-to")).toBe("/main/groups/42/conjugates/create")
    expect(wrapper.find('[data-test="filter-summary"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })

  it("renders LotsListView toolbar, filters, and table shell", async () => {
    const wrapper = await mountView(LotsListView)

    expect(wrapper.text()).toContain("Lots")
    expect(wrapper.text()).toContain("Export CSV")
    expect(findButtonByText(wrapper, "Create Lot")?.attributes("data-to")).toBe("/main/groups/42/lots/create")
    expect(wrapper.find('[data-test="filter-summary"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })

  it("renders MembersListView toolbar and table shell", async () => {
    const wrapper = await mountView(MembersListView)

    expect(wrapper.text()).toContain("Group Members")
    expect(findButtonByText(wrapper, "Create Member")?.attributes("data-to")).toBe("/main/groups/42/members/create")
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })

  it("renders PanelsListView toolbar, filters, and table shell", async () => {
    const wrapper = await mountView(PanelsListView)

    expect(wrapper.text()).toContain("Panels")
    expect(wrapper.text()).toContain("Show all panels")
    expect(findButtonByText(wrapper, "Create Panel")?.attributes("data-to")).toBe("/main/groups/42/panels/create")
    expect(wrapper.find('[data-test="filter-summary"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })

  it("renders ProteinsListView toolbar, filters, and table shell", async () => {
    const wrapper = await mountView(ProteinsListView)

    expect(wrapper.text()).toContain("Proteins")
    expect(findButtonByText(wrapper, "Create Protein")?.attributes("data-to")).toBe("/main/groups/42/proteins/create")
    expect(wrapper.find('[data-test="filter-summary"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })

  it("renders ProvidersListView toolbar, filters, and table shell", async () => {
    const wrapper = await mountView(ProvidersListView)

    expect(wrapper.text()).toContain("Providers")
    expect(findButtonByText(wrapper, "Create Provider")?.attributes("data-to")).toBe("/main/groups/42/providers/create")
    expect(wrapper.find('[data-test="filter-summary"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })

  it("renders SpeciesListView toolbar, filters, and table shell", async () => {
    const wrapper = await mountView(SpeciesListView)

    expect(wrapper.text()).toContain("Species")
    expect(findButtonByText(wrapper, "Create Species")?.attributes("data-to")).toBe("/main/groups/42/species/create")
    expect(wrapper.find('[data-test="filter-summary"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })

  it("renders StorageListView add action and table", async () => {
    const wrapper = await mountView(StorageListView)

    expect(wrapper.text()).toContain("Storage")
    expect(findButtonByText(wrapper, "Add Storage")?.attributes("data-to")).toBe("/main/groups/42/storage/create")
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
    expect(mocks.storageStore.loadListQuery).toHaveBeenCalledOnce()
  })

  it("renders TagsListView toolbar, filters, and table shell", async () => {
    const wrapper = await mountView(TagsListView)

    expect(wrapper.text()).toContain("Tags")
    expect(findButtonByText(wrapper, "Create Tag")?.attributes("data-to")).toBe("/main/groups/42/tags/create")
    expect(wrapper.find('[data-test="filter-summary"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })

  it("renders ValidationsListView toolbar, filters, and table shell", async () => {
    const wrapper = await mountView(ValidationsListView)

    expect(wrapper.text()).toContain("Validations")
    expect(wrapper.text()).toContain("Export CSV")
    expect(findButtonByText(wrapper, "Create Validation")?.attributes("data-to")).toBe("/main/groups/42/validations/create")
    expect(wrapper.find('[data-test="filter-summary"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="data-table-server"]').exists()).toBe(true)
  })
})
