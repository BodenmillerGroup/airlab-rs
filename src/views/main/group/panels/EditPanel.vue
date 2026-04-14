<template>
  <div class="mx-2 mt-2">
    <v-tooltip left>
      <template #activator="{ props }">
        <v-btn v-bind="props" v-show="fab" fab fixed bottom right color="primary" @click="toTop">
          <v-icon>mdi-chevron-up</v-icon>
        </v-btn>
      </template>
      <span>Scroll to top</span>
    </v-tooltip>

    <v-toolbar dense>
      <v-toolbar-title>Edit Panel</v-toolbar-title>
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
        <v-expansion-panel-title>Details</v-expansion-panel-title>
        <v-expansion-panel-text>
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
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-row dense class="mt-1">
      <v-col v-if="!alternateView" :cols="showOverview ? 2 : 3">
        <PanelTagsView :expanded="expanded" />
      </v-col>

      <v-col :cols="alternateView ? (showOverview ? 8 : 12) : showOverview ? 6 : 9">
        <AllConjugatesView
          v-if="alternateView"
          :on-selected="congugateSelected"
          :selected-conjugates="getInitialState()"
        />
        <TagConjugatesView
          v-else-if="activePanelTag"
          :tag="activePanelTag"
          :on-selected="congugateSelected"
          :selected-conjugates="getInitialState()"
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
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { required } from '@/utils/validators';

import { usePanelStore } from '@/stores/panel';
import { useTagStore } from '@/stores/tag';
import { useConjugateStore } from '@/stores/conjugate';
import { useSpeciesStore } from '@/stores/species';
import { useValidationStore } from '@/stores/validation';

import PanelTagsView from '@/views/main/group/panels/PanelTagsView.vue';
import TagConjugatesView from '@/views/main/group/panels/TagConjugatesView.vue';
import PanelPreview from '@/views/main/group/panels/PanelPreview.vue';
import AllConjugatesView from '@/views/main/group/panels/AllConjugatesView.vue';

import type { ConjugateDto } from '@/modules/conjugate/types';
import type { UpdatePanelDto, PanelElementDataDto } from '@/modules/panel/types';
import { useSelectedConjugateViews } from '@/composables/useSelectedConjugateViews';

const route = useRoute();
const router = useRouter();

const panelStore = usePanelStore();
const tagStore = useTagStore();
const conjugateStore = useConjugateStore();
const speciesStore = useSpeciesStore();
const validationStore = useValidationStore();

// UI state
const fab = ref(false);
const expanded = ref(0);
const valid = ref(false);
const name = ref('');
const description = ref('');
const application = ref<string | null>(null);
const isFluorophore = ref(false);
const isLocked = ref(false);
const alternateView = ref(false);
const showOverview = ref(true);

// data sets
const selectedTagConjugates = ref(new Map<number, Set<ConjugateDto>>());
const conjugatePanelData = ref(new Map<number, { dilutionType: number; concentration?: number }>());
const panelElements = ref<PanelElementDataDto[]>([]);

// form validation
const nameRules = [required];
const form = ref();

// ID
const panelId = +route.params.id;
const groupId = +route.params.groupId;

// active tag
const activePanelTagId = computed(() => panelStore.activePanelTagId);
const activePanelTag = computed(() =>
  activePanelTagId.value ? tagStore.getTag(activePanelTagId.value) : null
);

// panel
const panel = computed(() => panelStore.getPanel(panelId));
const { selectedTagConjugateViews: selectedTagConjugateViewsComputed } =
  useSelectedConjugateViews(selectedTagConjugates);
const selectedTagConjugateViews = selectedTagConjugateViewsComputed;

function getInitialState(): ConjugateDto[] {
  if (alternateView.value) {
    return [...selectedTagConjugates.value.values()].flatMap((set) => [...set]);
  }
  const items = selectedTagConjugates.value.get(activePanelTagId.value!);
  return items ? [...items] : [];
}

function onScroll() {
  const top = window.pageYOffset || 0;
  fab.value = top > 20;
}

function toTop() {
  (window as any).$vuetify.goTo(0);
}

function congugateSelected(tagId: number, conjugate: ConjugateDto, isSelected: boolean) {
  let set = selectedTagConjugates.value.get(tagId);
  if (!set) {
    set = new Set<ConjugateDto>();
    selectedTagConjugates.value.set(tagId, set);
  }
  isSelected ? set.add(conjugate) : set.delete(conjugate);
  selectedTagConjugates.value = new Map(selectedTagConjugates.value);
  if (isSelected && !conjugatePanelData.value.has(conjugate.id)) {
    conjugatePanelData.value.set(conjugate.id, { dilutionType: 0 });
  }
}

function focusSelectedConjugate(conjugate: { tagId: number }) {
  panelStore.activePanelTagId = conjugate.tagId;
  alternateView.value = false;
}

function cancel() {
  router.back();
}

function reset() {
  if (form.value) form.value.resetValidation();

  if (panel.value) {
    name.value = panel.value.name;
    description.value = panel.value.description;
    application.value = panel.value.application?.toString() || '';
    isFluorophore.value = panel.value.isFluorophore;
    isLocked.value = panel.value.isLocked;

    selectedTagConjugates.value = new Map();
    conjugatePanelData.value = new Map();

    for (const element of panelElements.value ?? []) {
      const conjugate = conjugateStore.getConjugate(element.conjugateId);
      if (!conjugate) continue;
      const tagId = conjugate.tagId;
      if (!selectedTagConjugates.value.has(tagId)) {
        selectedTagConjugates.value.set(tagId, new Set());
      }
      selectedTagConjugates.value.get(tagId)!.add(conjugate);
      conjugatePanelData.value.set(conjugate.id, {
        dilutionType: element.dilutionType,
        concentration: element.concentration,
      });
    }
  }
}

async function submit() {
  if (form.value?.validate() && panel.value) {
    const elements: PanelElementDataDto[] = [];
    const selectedViews = [...selectedTagConjugateViews.value.values()].flatMap((set) => [...set]);
    for (const view of selectedViews) {
      const data = conjugatePanelData.value.get(view.id);
      elements.push({
        conjugateId: view.id,
        dilutionType: data?.dilutionType ?? 0,
        concentration: data?.concentration,
      });
    }
    const payload: UpdatePanelDto = {
      name: name.value,
      description: description.value,
      application: application.value ? Number(application.value) : null,
      isFluorophore: isFluorophore.value,
      isLocked: isLocked.value,
      elements,
    };

    await panelStore.updatePanel({ id: panel.value.id, data: payload });
    router.push(route.path.replace("/create", "").replace(/\/[^/]+\/edit/, ""));
  }
}

onMounted(async () => {
  window.addEventListener('scroll', onScroll, { passive: true });
  onScroll();
  panelStore.activePanelTagId = null;

  const elements = await panelStore.getPanelElements(panelId);
  panelElements.value = elements;

  await Promise.all([
    panelStore.getPanelById(panelId),
    conjugateStore.getGroupConjugates(groupId),
    tagStore.getGroupTags(groupId),
    speciesStore.getGroupSpecies(groupId),
    validationStore.getGroupValidations(groupId),
  ]);

  reset();
});

onBeforeUnmount(() => {
  window.removeEventListener('scroll', onScroll);
});
</script>

<style scoped>
.toolbox-item-margin {
  margin-right: 16px;
}
</style>
