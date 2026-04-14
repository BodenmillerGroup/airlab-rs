  <template>
    <v-col>
      <v-toolbar dense class="toolbar">
        <v-toolbar-title>Clones</v-toolbar-title>
        <v-spacer />
        <v-toolbar-items>
          <v-btn text @click="exportFile" color="primary">Export CSV</v-btn>
          <v-btn text :to="`/main/groups/${activeGroupId}/clones/create`" color="primary">Create Clone</v-btn>
        </v-toolbar-items>
      </v-toolbar>

      <v-expansion-panels>
      <v-expansion-panel>
        <v-expansion-panel-title>
          <FilterSummary />
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <FilterField key-name="proteinName" class="mb-1" />
          <FilterField key-name="cloneName" class="mb-1" />
          <FilterField key-name="speciesName" class="mb-1" />
          <FilterField key-name="validationApplication" class="mb-1" />
          <FilterField key-name="validationStatus" class="mb-1" />
          <v-select
            v-model="reactivityFilter"
              :items="speciesOptions"
              item-title="label"
              item-value="value"
              label="Reactivity"
              multiple
              clearable
              dense
              variant="solo"
              prepend-icon="mdi-filter-outline"
              class="mb-1"
            />
          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>

      <v-text-field
        v-model="globalSearch"
        label="Search all clone fields"
        variant="solo"
        density="comfortable"
        clearable
        prepend-inner-icon="mdi-magnify"
        class="mt-3 mb-3"
      />

      <v-card>

        <v-data-table-server
          :headers="headers"
          :items="items"
          :loading="loading"
          :items-length="cloneStore.total"
          v-model:items-per-page="tableItemsPerPage"
          v-model:page="tablePage"
          v-model:sort-by="sortBy"
          :footer-props="tableFooterProps"
          show-expand
          v-model:expanded="expanded"
          item-value="id"
        >
          <template #item.protein="{ item }">
            <router-link
              class="link"
              :to="{
                name: 'main-group-proteins-edit',
                params: { groupId: item.groupId, id: item.proteinId }
              }"
            >
              {{ item.proteinName }}
            </router-link>
          </template>

          <template #item.species="{ item }">
            <router-link
              v-if="item.speciesId"
              class="link"
              :to="{ name: 'main-group-species-edit', params: { id: item.speciesId } }"
            >
              {{ item.speciesName }}
            </router-link>
          </template>

          <template #item.isPhospho="{ item }">
            <v-icon v-if="item.isPhospho">mdi-check</v-icon>
          </template>

          <template #item.isPolyclonal="{ item }">
            <v-icon v-if="item.isPolyclonal">mdi-check</v-icon>
          </template>

          <template #item.application="{ item }">
            <v-chip-group v-if="item.application && typeof item.application === 'object'" multiple column class="px-0">
              <v-chip v-if="item.application?.[applicationMap.sMC]" size="x-small" pill disabled class="mr-1">SMC</v-chip>
              <v-chip v-if="item.application?.[applicationMap.iMC]" size="x-small" pill disabled class="mr-1">IMC</v-chip>
              <v-chip v-if="item.application?.[applicationMap.FC]" size="x-small" pill disabled class="mr-1">FC</v-chip>
              <v-chip v-if="item.application?.[applicationMap.IF]" size="x-small" pill disabled class="mr-1">IF</v-chip>
              <v-chip v-if="item.application?.[applicationMap.IHC]" size="x-small" pill disabled class="mr-1">IHC</v-chip>
              <v-chip v-if="item.application?.[applicationMap.IHCF]" size="x-small" pill disabled class="mr-1">IHC-F</v-chip>
              <v-chip v-if="item.application?.[applicationMap.WB]" size="x-small" pill disabled class="mr-1">WB</v-chip>
            </v-chip-group>
          </template>

        <template #item.validations="{ item }">
            <template v-if="filteredCloneValidationMap[item.id]?.length">
              <v-chip
                v-for="validation in filteredCloneValidationMap[item.id]"
                :key="validation.id"
                :color="getStatusColor(validation)"
                class="mr-1"
                size="x-small"
                @click.stop="showValidation(validation.id)"
              >
                {{ applicationToString(validation.application) }}
              </v-chip>
            </template>
            <span v-else>—</span>
          </template>

          <template #item.action="{ item }">
            <v-menu location="bottom">
              <template #activator="{ props }">
                <v-btn icon v-bind="props">
                  <v-icon>mdi-dots-vertical</v-icon>
                </v-btn>
              </template>

              <v-list density="compact">
                <v-list-item
                  :to="{ name: 'main-group-lots-create', params: { groupId: activeGroupId }, query: { cloneId: item.id } }"
                >
                  <v-icon start color="primary">mdi-plus-circle-outline</v-icon>
                  <v-list-item-title>Add Lot</v-list-item-title>
                </v-list-item>

                <v-list-item
                  :to="{ name: 'main-group-validations-create', params: { groupId: activeGroupId }, query: { cloneId: item.id } }"
                >
                  <v-icon start color="primary">mdi-plus-circle-outline</v-icon>
                  <v-list-item-title>Add Validation</v-list-item-title>
                </v-list-item>

                <v-divider />

                <v-list-item
                  :to="{ name: 'main-group-clones-edit', params: { groupId: activeGroupId, id: item.id } }"
                >
                  <v-icon start color="grey">mdi-pencil-outline</v-icon>
                  <v-list-item-title>Edit</v-list-item-title>
                </v-list-item>

                <v-list-item
                  v-if="isGroupAdmin"
                  @click="cloneStore.updateCloneArchiveState({ id: item.id, data: { state: !item.isArchived } })"
                >
                  <v-icon start color="red">
                    {{ item.isArchived ? 'mdi-archive-arrow-up-outline' : 'mdi-archive-arrow-down-outline' }}
                  </v-icon>
                  <v-list-item-title>
                    {{ item.isArchived ? 'Unarchive' : 'Archive' }}
                  </v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>

            <v-tooltip location="bottom">
              <template #activator="{ props }">
                <v-btn icon v-bind="props" @click.stop="showDetails(item)">
                  <v-icon>mdi-information-outline</v-icon>
                </v-btn>
              </template>
              <span>Show details</span>
            </v-tooltip>
          </template>

          <template #expanded-row="{ item, columns }">
            <td :colspan="columns?.length || 9">
              <CloneExpandedView :clone="cloneStore.getClone(item.id)" />
            </td>
          </template>

        </v-data-table-server>
      </v-card>

      <v-navigation-drawer v-model="drawer" right fixed temporary width="600">
        <CloneDetailsView v-if="drawer" :clone="detailsItem" />
      </v-navigation-drawer>
      <v-navigation-drawer v-model="validationDrawer" right fixed temporary width="600">
        <ValidationDetailsView v-if="validationDrawer" :validation-id="selectedValidationId" />
      </v-navigation-drawer>
    </v-col>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onActivated } from "vue";
import { storeToRefs } from "pinia";
import { useGroupStore } from "@/stores/group";
import { useCloneStore } from "@/stores/clone";
import { useSpeciesStore } from "@/stores/species";
import { useFilterStore } from '@/stores/useFilterStore'
import { useValidationStore } from "@/stores/validation";
import { exportCsv } from '@/utils/exporters';
import { applicationNameToId } from "@/utils/enums";
import FilterField from '@/components/FilterField.vue'
import FilterSummary from '@/components/FilterSummary.vue'
import { getStatusColor } from '@/utils/converters'
import CloneDetailsView from "@/views/main/group/clones/CloneDetailsView.vue";
import CloneExpandedView from '@/views/main/group/clones/CloneExpandedView.vue'
import ValidationDetailsView from "@/views/main/group/validations/ValidationDetailsView.vue";
import { useServerTablePagination } from "@/composables/useServerTablePagination";
import type { CloneDto, CloneView } from "@/modules/clone/types";
import type { ValidationDto } from "@/modules/validation/types";
import {
  applicationToString,
} from '@/utils/converters'
import { useClones } from '@/composables/useClones'

const groupStore = useGroupStore();
const cloneStore = useCloneStore();
const { page, limit } = storeToRefs(cloneStore);
const speciesStore = useSpeciesStore();
const filterStore = useFilterStore()
const validationStore = useValidationStore();
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit);
function normalizeFilterValues(value: unknown): number[] {
  if (Array.isArray(value)) {
    return value
      .map((item) => Number(item))
      .filter((item) => !Number.isNaN(item))
  }

  if (value === '' || value === null || value === undefined) {
    return []
  }

  const parsed = Number(value)
  return Number.isNaN(parsed) ? [] : [parsed]
}

const filteredCloneValidationMap = computed<Record<number, ValidationDto[]>>(() => {
  const applicationFilterValues = normalizeFilterValues(filterStore.filters.validationApplication)
  const statusFilterValues = normalizeFilterValues(filterStore.filters.validationStatus)

  return Object.fromEntries(
    Object.entries(validationStore.cloneValidationMap).map(([cloneId, validations]) => [
      Number(cloneId),
      validations.filter((validation) => {
        const matchesApplication =
          applicationFilterValues.length === 0 || applicationFilterValues.includes(validation.application)
        const matchesStatus =
          statusFilterValues.length === 0 || statusFilterValues.includes(validation.status)

        return matchesApplication && matchesStatus
      }),
    ]),
  )
})

const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'id', order: 'desc' },
])
const globalSearch = computed({
  get: () => String(filterStore.filters.cloneGlobalSearch ?? ''),
  set: (val: string | null) => filterStore.setFilter('cloneGlobalSearch', val ?? ''),
})

const { items, loading, reload } = useClones({ sortBy, globalFilter: globalSearch })
const drawer = ref(false);
const validationDrawer = ref(false);
const detailsItem = ref<CloneDto | null>(null);
const selectedValidationId = ref<number | null>(null);
const applicationMap = applicationNameToId;
const expanded = ref<string[]>([])

const rawHeaders = [
  { title: 'Id', key: 'id', sortable: true },
  { title: 'Protein', key: 'protein', sortable: true },
  { title: 'Name', key: 'name', sortable: true },
  { title: 'Host', key: 'species', sortable: true },
  { title: 'Phospho', key: 'isPhospho', sortable: true },
  { title: 'Application', key: 'application', sortable: true },
  { title: 'Validations', key: 'validations', sortable: true },
  { title: 'Actions', key: 'action', sortable: false, width: "130", },
  { title: '', key: 'data-table-expand', sortable: false },
]


const headers = rawHeaders

const reactivityFilter = computed({
  get: () => filterStore.filters.reactivity,
  set: (val) => filterStore.setFilter('reactivity', val),
})

const speciesOptions = computed(() =>
  speciesStore.species.map(s => ({ label: s.name, value: s.id }))
)

watch(() => groupStore.activeGroupId, async (groupId) => {
  if (typeof groupId === 'number') {
    await speciesStore.getGroupSpecies(groupId)
  }
}, { immediate: true })

async function refreshList() {
  const groupId = groupStore.activeGroupId
  if (typeof groupId !== 'number') {
    return
  }

  await speciesStore.getGroupSpecies(groupId)
  await reload()
}

onMounted(refreshList)
onActivated(refreshList)

const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);

function exportFile() {
  const csv = cloneStore.getCsv(items.value);
  exportCsv(csv, 'clones.csv');
}

function showValidation(id: number) {
  selectedValidationId.value = id;
  validationDrawer.value = true;
}

function showDetails(item: CloneView) {
  detailsItem.value = cloneStore.getClone(item.id) ?? null
  drawer.value = true
}

</script>
