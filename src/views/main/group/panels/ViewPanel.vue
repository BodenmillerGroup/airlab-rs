<template>
  <v-container v-if="panel" fluid>
    <v-tooltip left>
        <template #activator="{ props }">
        <v-btn v-bind="props" v-scroll="onScroll" v-show="fab" fab fixed bottom right color="primary" @click="toTop">
          <v-icon>mdi-chevron-up</v-icon>
        </v-btn>
      </template>
      <span>Scroll to top</span>
    </v-tooltip>

    <v-toolbar dense class="toolbar">
      <v-toolbar-title class="toolbar-item-margin">Panel View</v-toolbar-title>
      <v-spacer />
      <v-text-field
        v-model.number="totalVolume"
        label="Total Volume"
        type="number"
        outlined
        dense
        class="toolbar-text-field toolbar-item-margin"
      />
      <v-text-field
        :value="diluentVolume"
        label="Diluent Volume"
        outlined
        dense
        disabled
        class="toolbar-text-field toolbar-item-margin"
      />
      <v-switch
        v-model="excludeEmpty"
        label="Exclude volume from empty tubes"
        class="toolbar-item-margin"
      />
      <v-toolbar-items>
        <v-divider vertical />
        <v-menu offset-y>
          <template #activator="{ props }">
            <v-btn text v-bind="props" color="primary">Export</v-btn>
          </template>
          <v-list>
            <v-list-item @click="exportCsv"><v-list-item-title>Export CSV</v-list-item-title></v-list-item>
            <v-divider />
            <v-list-item @click="exportCyTOF1"><v-list-item-title>Template for CyTOF1</v-list-item-title></v-list-item>
            <v-list-item @click="exportCyTOF2"><v-list-item-title>Template for CyTOF2</v-list-item-title></v-list-item>
            <v-list-item @click="exportHelios"><v-list-item-title>Template for Helios</v-list-item-title></v-list-item>
            <v-list-item @click="exportHeliosCsv"><v-list-item-title>CSV for Helios</v-list-item-title></v-list-item>
          </v-list>
        </v-menu>
        <v-divider vertical />
        <v-btn
          text
          color="primary"
          :to="{
            name: 'main-group-panels-edit',
            params: { groupId: activeGroupId, id: panel?.id }
          }"
        >Edit</v-btn>
        <v-divider vertical />
        <v-btn @click="cancel" text color="primary">Cancel</v-btn>
        <v-btn @click="reset" text color="primary">Reset</v-btn>
        <v-btn @click="submit" text color="primary">Save</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-card>
      <v-card-title primary-title>
        <div class="text-h5 primary--text">{{ panel.name }}</div>
      </v-card-title>
      <v-data-table
        :headers="headers"
        :items="items"
        :loading="loading"
        v-model:sort-by="sortBy"
        :items-per-page="-1"
        hide-default-footer
        disable-filtering
        disable-pagination
      >
        <template v-slot:item.tubeNumber="{ item }">
          <router-link
            class="link"
            :to="{
              name: 'main-group-conjugates-edit',
              params: {
                groupId: activeGroupId,
                id: item.id,
              },
            }"
          >
            <span :class="item.status === 2 ? 'empty' : item.status === 1 ? 'low' : ''">{{ item.tubeNumber }}</span>
          </router-link>
        </template>
        <template #item.tag="{ item }">
          <router-link
            class="link"
            :to="{
              name: 'main-group-tags-edit',
              params: {
                groupId: activeGroupId,
                id: item.tagId,
              },
            }"
          >
            <span :class="item.status === 2 ? 'empty' : item.status === 1 ? 'low' : ''">{{item.tagName}}</span>
          </router-link>
        </template>
        <template v-slot:item.protein="{ item }">
          <router-link
            v-if="item.proteinId"
            class="link"
            :to="{
              name: 'main-group-proteins-edit',
              params: {
                groupId: activeGroupId,
                id: item.proteinId,
              },
            }"
          >
            <span :class="item.status === 2 ? 'empty' : item.status === 1 ? 'low' : ''">{{
              item.proteinName
            }}</span>
          </router-link>
        </template>
        <template v-slot:item.lot="{ item }">
          <router-link
            v-if="item.lotId"
            class="link"
            :to="{
              name: 'main-group-lots-edit',
              params: {
                groupId: activeGroupId,
                id: item.lotId,
              },
            }"
          >
            <span :class="item.status === 2 ? 'empty' : item.status === 1 ? 'low' : ''">{{ item.lotNumber }}</span>
          </router-link>
        </template>
        <template v-slot:item.clone="{ item }">
          <router-link
            v-if="item.cloneId"
            class="link"
            :to="{
              name: 'main-group-clones-edit',
              params: {
                groupId: activeGroupId,
                id: item.cloneId,
              },
            }"
          >
            <span :class="item.status === 2 ? 'empty' : item.status === 1 ? 'low' : ''">{{ item.cloneName }}</span>
          </router-link>
        </template>
        <template v-slot:item.validations="{ item }">
          <v-chip
            v-for="validation in item.validations"
            :key="validation.id"
            :color="getStatusColor(validation)"
            class="mr-1"
            x-small
            dark
            @click="showValidation(validation.id)"
          >
            {{ applicationToString(validation.application) }}

          </v-chip>
        </template>
        <template v-slot:item.actualConcentration="{ item }">
          <div class="inline-concentration-cell">
            <v-text-field
              v-if="editingConcentrationId === item.id"
              v-model.number="item.actualConcentration"
              class="inline-concentration-input"
              label="Concentration"
              type="number"
              density="compact"
              hide-details
              autofocus
              @blur="stopEditingConcentration"
              @keydown.enter="stopEditingConcentration"
              @keydown.esc="stopEditingConcentration"
            />
            <span
              v-else
              class="cursor-pointer"
              @click="startEditingConcentration(item.id)"
            >
              {{ item.actualConcentration }}
            </span>
          </div>
        </template>
        <template v-slot:item.dilutionType="{ item }">
          <v-menu activator="parent">
            <template #default="{ isActive }">
              <v-btn-toggle v-model.number="item.dilutionType" mandatory>
                <v-btn :value="0">µg/mL</v-btn>
                <v-btn :value="1">1/__</v-btn>
              </v-btn-toggle>
            </template>
          </v-menu>

          <span class="cursor-pointer">
            {{ item.dilutionType === 1 ? "1/__" : "µg/mL" }}
          </span>
        </template>
        <template v-slot:item.pipet="{ item }">
          <span :class="item.status === 2 ? 'empty' : item.status === 1 ? 'low' : ''">{{
            getAmountAntibodyText(item)
          }}</span>
        </template>
      </v-data-table>
    </v-card>

    <v-navigation-drawer v-model="drawer" right fixed temporary width="600">
      <ValidationDetailsView v-if="drawer" :validation-id="selectedValidationId" />
    </v-navigation-drawer>
  </v-container>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useGroupStore } from '@/stores/group'
import { usePanelStore } from '@/stores/panel'
import { useMainStore } from '@/stores/main'
import { useTagStore } from '@/stores/tag'
import { useLotStore } from '@/stores/lot'
import { useConjugateStore } from '@/stores/conjugate'
import { useCloneStore } from '@/stores/clone'
import { useValidationStore } from '@/stores/validation'
import { usePanelElementStore } from '@/stores/panel_element'
import {
  exportCSVCyTOF201608,
  exportCyTOF1Panel,
  exportCyTOF2Panel,
  exportHeliosPanel,
  exportPanelCsv
} from '@/utils/exporters'
import { getStatusColor } from '@/utils/converters'
import type { PanelElementDto } from '@/modules/panel_element/types';
import type { ConjugateDto } from '@/modules/conjugate/types';
import type { LotDto } from '@/modules/lot/types';
import type { TagDto } from '@/modules/tag/types';
import type { ValidationDto } from '@/modules/validation/types';
import type { UpdatePanelDto } from '@/modules/panel/types';
import { applicationToString } from '@/utils/converters'
import ValidationDetailsView from '@/views/main/group/validations/ValidationDetailsView.vue'


const route = useRoute()
const router = useRouter()
const groupStore = useGroupStore()
const panelStore = usePanelStore()
const cloneStore = useCloneStore()
const tagStore = useTagStore()
const conjugateStore = useConjugateStore()
const lotStore = useLotStore()
const validationStore = useValidationStore()
const panelElementStore = usePanelElementStore()

const activeGroupId = computed(() => groupStore.activeGroupId)
const panelId = computed(() => {
  const id = Number(route.params.id)
  return Number.isFinite(id) ? id : null
})

const fab = ref(false)
const excludeEmpty = ref(false)
const totalVolume = ref(100)
const drawer = ref(false)
const selectedValidationId = ref<number | null>(null)
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([])
const editingConcentrationId = ref<number | null>(null)

const panel = computed(() => panelStore.getPanel(panelId.value))
const validations = computed(() => validationStore.validations)
const elements = ref<PanelElementDto[]>([])
const conjugates = ref<ConjugateDto[]>([])
const lots = ref<LotDto[]>([])
const tags = ref<TagDto[]>([])
import { usePanelElements } from '@/composables/usePanelElements'
const { items, loading, reload } =
  usePanelElements(panelId, activeGroupId)
const rawHeaders = [
  { title: "Id", key: "id", sortable: true },
  { title: "Tube Number", key: "tubeNumber", align: "end", sortable: true, sort: (a: number, b: number) => Number(a ?? 0) - Number(b ?? 0) },
  { title: "Tag", key: "tagName", sortable: true, sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? '') },
  { title: "Mass", key: "tagMw", align: "end", width: 100, sortable: true, sort: (a: number | null, b: number | null) => Number(a ?? 0) - Number(b ?? 0) },
  { title: "Protein", key: "proteinName", sortable: true, sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? '') },
  { title: "Clone", key: "cloneName", sortable: true, sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? '') },
  { title: "Lot", key: "lotNumber", sortable: true, sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? '') },
  { title: "Validations", key: "validations", sortable: false },
  { title: "Staining Concentration", key: "actualConcentration", sortable: true, sort: (a: number | null, b: number | null) => Number(a ?? 0) - Number(b ?? 0) },
  { title: "Type", key: "dilutionType", sortable: false },
  { title: "µL to pipet", key: "pipet", sortable: true, sort: (a: number, b: number) => Number(a ?? 0) - Number(b ?? 0) },
] as const

const headers = rawHeaders

function getAmountAntibody(item: any) {
  if (item.dilutionType === 1) {
    return item.actualConcentration === 0 ? 0 : totalVolume.value / item.actualConcentration
  } else {
    return totalVolume.value * (item.actualConcentration / item.concentration)
  }
}

function getAmountAntibodyText(item: any) {
  const amount = getAmountAntibody(item)
  return amount == null ? "" : amount.toFixed(2)
}


const diluentVolume = computed(() => {
  let cum = 0.0
  for (const item of items.value) {
    if (!item.actualConcentration) continue
    if (excludeEmpty.value && Number(item.finishedBy) > 0) continue
    const add = getAmountAntibody(item)
    if (!isNaN(add)) cum += add
  }
  return (totalVolume.value - cum).toFixed(2)
})

function closeEditDialog() {
  excludeEmpty.value = !excludeEmpty.value
  excludeEmpty.value = !excludeEmpty.value
}

function startEditingConcentration(id: number) {
  editingConcentrationId.value = id
}

function stopEditingConcentration() {
  editingConcentrationId.value = null
}

function onScroll(e: any) {
  const top = window.pageYOffset || e.target.scrollTop || 0
  fab.value = top > 20
}

function toTop() {
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

function cancel() {
  router.back()
}

async function reset() {
  if (typeof panelId.value === 'number') {
    await panelStore.getPanelById(panelId.value)
  }
}

async function submit() {
  const elements = items.value.map((item) => ({
    conjugateId: Number(item.id),
    dilutionType: Number(item.dilutionType),
    concentration: item.actualConcentration
  }))
  await panelStore.updatePanel({ id: panelId.value, data: elements as unknown as UpdatePanelDto})

  router.back()
}

function exportCsv() {
  exportPanelCsv(panel.value, items.value, totalVolume.value)
}
function exportHeliosCsv() {
  exportCSVCyTOF201608(panel.value, items.value)
}
function exportCyTOF1() {
  exportCyTOF1Panel(panel.value, items.value)
}
function exportCyTOF2() {
  exportCyTOF2Panel(panel.value, items.value)
}
function exportHelios() {
  exportHeliosPanel(panel.value, items.value)
}

function showValidation(id: number) {
  selectedValidationId.value = id
  drawer.value = true
}

onMounted(async () => {
  if (typeof panelId.value === 'number' && typeof activeGroupId.value === 'number') {
    await reload()
  }
})

watch([panelId, activeGroupId], async () => {
  if (typeof panelId.value === 'number' && typeof activeGroupId.value === 'number') {
    await reload()
  }
})

</script>

<style scoped>
.inline-concentration-cell {
  min-height: 40px;
  display: flex;
  align-items: center;
}

.inline-concentration-input {
  max-width: 140px;
}
</style>
