<template>
  <div class="mx-2 mt-2">
    <v-tooltip left>
      <template #activator="{ props }">
        <v-btn v-bind="props" v-scroll="onScroll" v-show="fab" fab fixed bottom right color="primary" @click="toTop">
          <v-icon>mdi-chevron-up</v-icon>
        </v-btn>
      </template>
      <span>Scroll to top</span>
    </v-tooltip>

    <v-toolbar dense>
      <v-toolbar-title>Create Panel</v-toolbar-title>
      <v-spacer />
      <v-switch v-model="alternateView" label="Alternate View" hide-details class="toolbox-item-margin" />
      <v-switch v-model="showOverview" label="Overview" hide-details />
      <v-toolbar-items>
        <v-btn @click="cancel" text color="primary">Cancel</v-btn>
        <v-btn @click="reset" text color="primary">Reset</v-btn>
        <v-btn @click="submit" text :disabled="!valid" color="primary">Save</v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <v-expansion-panels class="mt-4" v-model="expanded">
      <v-expansion-panel>
        <v-expansion-panel-header>Details</v-expansion-panel-header>
        <v-expansion-panel-content>
          <v-form v-model="valid" ref="form">
            <v-text-field label="Name" v-model="name" :rules="nameRules" />
            <v-text-field label="Description" v-model="description" />
            <div class="text-subtitle-1">Application</div>
            <v-btn-toggle v-model="application">
              <v-btn small value="0">SMC</v-btn>
              <v-btn small value="1">IMC</v-btn>
              <v-btn small value="2">FC</v-btn>
              <v-btn small value="3">IF</v-btn>
              <v-btn small value="4">IHC</v-btn>
              <v-btn small value="5">IHCF</v-btn>
              <v-btn small value="6">WB</v-btn>
            </v-btn-toggle>
            <v-checkbox label="Fluorophore" v-model="isFluorophore" />
            <v-checkbox label="Locked" v-model="isLocked" />
          </v-form>
        </v-expansion-panel-content>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-row v-if="canConfigureElements" dense class="mt-1">
      <v-col v-if="!alternateView" :cols="showOverview ? 2 : 3">
        <PanelTagsView :expanded="expanded" />
      </v-col>
      <v-col :cols="alternateView ? (showOverview ? 8 : 12) : showOverview ? 6 : 9">
        <AllConjugatesView v-if="alternateView" :on-selected="conjugateSelected" />
        <TagConjugatesView
          v-else-if="activePanelTag"
          :tag="activePanelTag"
          :on-selected="conjugateSelected"
          :selected-conjugates="selectedForActiveTag"
        />
      </v-col>
      <v-col v-show="showOverview" cols="4">
        <PanelPreview
          :conjugates="selectedTagConjugateViews"
          :expanded="expanded"
          :on-select-conjugate="focusSelectedConjugate"
        />
      </v-col>
    </v-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { required } from '@/utils/validators';
import { useGroupStore } from '@/stores/group';
import { usePanelStore } from '@/stores/panel';
import { useConjugateStore } from '@/stores/conjugate';
import { useTagStore } from '@/stores/tag';
import { useSpeciesStore } from '@/stores/species';
import { useValidationStore } from '@/stores/validation';
import type { CreatePanelDto } from '@/modules/panel/types';
import type { PanelElementDataDto } from '@/modules/panel/types';
import type { ConjugateDto } from '@/modules/conjugate/types';
import { useSelectedConjugateViews } from '@/composables/useSelectedConjugateViews';

// Components
import PanelTagsView from '@/views/main/group/panels/PanelTagsView.vue';
import TagConjugatesView from '@/views/main/group/panels/TagConjugatesView.vue';
import AllConjugatesView from '@/views/main/group/panels/AllConjugatesView.vue';
import PanelPreview from '@/views/main/group/panels/PanelPreview.vue';

// Stores
const groupStore = useGroupStore();
const panelStore = usePanelStore();
const conjugateStore = useConjugateStore();
const tagStore = useTagStore();
const speciesStore = useSpeciesStore();
const validationStore = useValidationStore();

// Router
const router = useRouter();
const route = useRoute();
const groupId = computed(() => Number(route.params.groupId));
const canConfigureElements = computed(() => {
  const panelId = Number(route.params.id);
  return Number.isFinite(panelId) && panelId > 0;
});

// Form
const valid = ref(false);
const form = ref<any | null>(null);
const name = ref('');
const description = ref('');
const application = ref<string | null>(null);
const isFluorophore = ref(false);
const isLocked = ref(false);
const nameRules = [required];

// UI controls
const fab = ref(false);
const expanded = ref(0);
const showOverview = ref(true);
const alternateView = ref(false);

// Conjugate selections
const selectedTagConjugates = ref<Map<number, Set<ConjugateDto>>>(new Map());
const { selectedTagConjugateViews: selectedTagConjugateViewsComputed } =
  useSelectedConjugateViews(selectedTagConjugates);

const selectedTagConjugateViews = selectedTagConjugateViewsComputed;

// Panel tag state
const activePanelTagId = computed(() => panelStore.activePanelTagId);
const activePanelTag = computed(() =>
  activePanelTagId.value ? tagStore.getTag(activePanelTagId.value) : null
);
const selectedForActiveTag = computed(() => {
  const tagId = activePanelTagId.value;
  if (!tagId) return [];
  const set = selectedTagConjugates.value.get(tagId);
  return set ? [...set] : [];
});

// Scroll to top
function toTop() {
  window.scrollTo({ top: 0, behavior: 'smooth' });
}

function onScroll(e: Event) {
  const scrollTop = window.pageYOffset || (e.target as HTMLElement).scrollTop || 0;
  fab.value = scrollTop > 20;
}

// Track selected conjugates
function conjugateSelected(tagId: number, conjugate: ConjugateDto, isSelected: boolean) {
  const currentSet = selectedTagConjugates.value.get(tagId) ?? new Set<ConjugateDto>();
  if (isSelected) {
    currentSet.add(conjugate);
  } else {
    currentSet.delete(conjugate);
  }
  selectedTagConjugates.value.set(tagId, currentSet);
  selectedTagConjugates.value = new Map(selectedTagConjugates.value); // trigger reactivity
}

function focusSelectedConjugate(conjugate: { tagId: number }) {
  panelStore.activePanelTagId = conjugate.tagId;
  alternateView.value = false;
}

function reset() {
  name.value = '';
  description.value = '';
  application.value = '';
  isFluorophore.value = false;
  isLocked.value = false;
  selectedTagConjugates.value = new Map();
  form.value?.resetValidation();
}

function cancel() {
  router.back();
}

async function submit() {
  if (form.value?.validate()) {
    const currentMemberId =
      groupStore.myMember?.groupId === groupId.value ? groupStore.myMember.id : null;
    if (!currentMemberId) return;

    const elements: PanelElementDataDto[] = [];
    selectedTagConjugates.value.forEach((set) => {
      set.forEach((conjugate) => {
        elements.push({
          conjugateId: conjugate.id,
          dilutionType: 0,
        });
      });
    });

    const payload: CreatePanelDto = {
      createdBy: currentMemberId,
      groupId: groupId.value,
      name: name.value,
      description: description.value,
      application: application.value ? Number(application.value) : null,
      isFluorophore: isFluorophore.value,
      isLocked: isLocked.value,
      elements,
    };

    await panelStore.createPanel(payload);
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  panelStore.activePanelTagId = null;
  await Promise.all([
    groupStore.getMyMember(groupId.value),
    conjugateStore.getGroupConjugates(groupId.value),
    tagStore.getGroupTags(groupId.value),
    speciesStore.getGroupSpecies(groupId.value),
    validationStore.getGroupValidations(groupId.value),
  ]);
});
</script>
