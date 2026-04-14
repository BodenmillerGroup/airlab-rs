<template>
  <v-card tile>
    <v-toolbar flat dense class="mb-2">
      <v-text-field v-model="search" label="Search conjugates" single-line hide-details clearable dense>
        <template #append>
          <v-icon dense>mdi-magnify</v-icon>
        </template>
      </v-text-field>
      <v-spacer />
      <v-switch v-model="showEmpty" label="Show empty" hide-details inset dense class="ml-2" style="width: 200px" />
      <v-select
        v-model="sortBy"
        flat
        solo
        hide-details
        :items="sortByOptions"
        item-value="id"
        item-text="title"
        prepend-inner-icon="mdi-sort"
        label="Sort by"
        dense
      />
      <v-btn-toggle v-model="sortDesc" mandatory dense>
        <v-btn depressed :value="false" x-small>
          <v-icon x-small>mdi-arrow-up</v-icon>
        </v-btn>
        <v-btn depressed :value="true" x-small>
          <v-icon x-small>mdi-arrow-down</v-icon>
        </v-btn>
      </v-btn-toggle>
    </v-toolbar>

    <v-card-text>
      <v-data-iterator
        :value="selectedConjugates ?? []"
        :items="filteredConjugates"
        item-key="id"
        :items-per-page="-1"
        hide-default-footer
        disable-pagination
        disable-filtering
        disable-sort
        @item-selected="handleItemSelected"
      >
        <template #default="{ items }">
          <v-row>
            <v-sheet
              v-for="item in items"
              :key="item.raw.id"
              tile
              width="180"
              class="ma-1 pa-0"
              @click.prevent="
                () => {
                  const nextSelected = !isConjugateSelected(item.raw.id);
                  onSelected(item.raw.tagId, item.raw.conjugate, nextSelected);
                }
              "
              :color="getConjugateColor(item.raw.conjugate, isConjugateSelected(item.raw.id))"
            >
              <div class="content">
                <div><span class="subheader">Tag:</span> {{ item.raw.tagMw ? item.raw.tagName + item.raw.tagMw : item.raw.tagName }}</div>
                <div><span class="subheader">Tube:</span> {{ item.raw.tubeNumber }}</div>
                <div><span class="subheader">Protein:</span> {{ item.raw.proteinName }}</div>
                <div><span class="subheader">Clone:</span> {{ item.raw.cloneName }}</div>
                <div>
                  <span class="subheader">Reactivity:</span>
                  <v-chip
                    v-for="r of item.raw.reactivity"
                    :key="r"
                    x-small
                    label
                    class="chip"
                  >
                    {{ speciesMap.get(r)?.acronym ?? "?" }}
                  </v-chip>
                </div>
                <div v-if="item.raw.validations?.length">
                  <span class="subheader">Validations:</span>
                  <v-chip
                    v-for="v in item.raw.validations"
                    :key="v.id"
                    :color="getStatusColor(v)"
                    class="mr-1"
                    x-small
                    dark
                    @click.stop="showValidation(v.id)"
                  >
                    {{ applicationToString(v.application) }}
                  </v-chip>
                </div>
              </div>
            </v-sheet>
          </v-row>
        </template>
      </v-data-iterator>
    </v-card-text>

    <v-navigation-drawer v-model="drawer" right fixed temporary width="600">
      <ValidationDetailsView v-if="drawer" :validation-id="selectedValidationId" />
    </v-navigation-drawer>
  </v-card>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import type { TagDto } from '@/modules/tag/types';
import type { SpeciesDto } from '@/modules/species/types';
import type { ConjugateDto } from '@/modules/conjugate/types';
import { ConjugateStatus } from '@/modules/conjugate/ConjugateStatus';
import { getStatusColor } from '@/utils/converters';
import { applicationToString } from '@/utils/converters'

import { useSpeciesStore } from '@/stores/species';
import { useTagConjugates } from '@/composables/useTagConjugates';
import { useConjugateStore } from '@/stores/conjugate';

import ValidationDetailsView from '@/views/main/group/validations/ValidationDetailsView.vue';

const props = defineProps<{
  tag: TagDto;
  selectedConjugates?: ConjugateDto[];
  onSelected: (tagId: number, item: ConjugateDto, selected: boolean) => void;
}>();

const onSelected = props.onSelected;

const speciesStore = useSpeciesStore();
const conjugateStore = useConjugateStore();

const sortBy = ref<'protein' | 'clone' | 'tube'>('protein');
const sortDesc = ref(false);
const search = ref<string | null>(null);
const showEmpty = ref(false);

const drawer = ref(false);
const selectedValidationId = ref<number | null>(null);

const sortByOptions = [
  { id: 'protein', title: 'Protein' },
  { id: 'clone', title: 'Clone' },
  { id: 'tube', title: 'Tube' },
];

const speciesMap = computed(() => {
  const map = new Map<number, SpeciesDto>();
  speciesStore.species.forEach((s) => map.set(s.id, s));
  return map;
});

const { items: conjugates } = useTagConjugates({ tagId: computed(() => props.tag.id) });

const filteredConjugates = computed(() => {
  let items = conjugates.value.slice();
  if (!showEmpty.value) {
    items = items.filter(i => i.status !== ConjugateStatus.Finished);
  }

  if (search.value) {
    const term = search.value.toLowerCase().trim();
    items = items.filter((i) =>
      i.cloneName.toLowerCase().includes(term) ||
      i.proteinName.toLowerCase().includes(term)
    );
  }

  // Sorting
  switch (sortBy.value) {
    case 'tube':
      items.sort((a, b) => sortDesc.value ? b.tubeNumber - a.tubeNumber : a.tubeNumber - b.tubeNumber);
      break;
    case 'protein':
      items.sort((a, b) =>
        sortDesc.value
          ? b.proteinName.localeCompare(a.proteinName)
          : a.proteinName.localeCompare(b.proteinName)
      );
      break;
    case 'clone':
      items.sort((a, b) =>
        sortDesc.value
          ? b.cloneName.localeCompare(a.cloneName)
          : a.cloneName.localeCompare(b.cloneName)
      );
      break;
  }

  return items;
});

const selectedConjugateIds = computed(() =>
  new Set((props.selectedConjugates ?? []).map((conjugate) => conjugate.id))
);

function isConjugateSelected(conjugateId: number): boolean {
  return selectedConjugateIds.value.has(conjugateId);
}

function getConjugateColor(conjugate: ConjugateDto, isSelected: boolean): string {
  const isOver = conjugate.status === ConjugateStatus.Finished;
  const isLow = conjugate.status === ConjugateStatus.Low;

  if (isSelected) {
    if (isOver) return 'red';
    if (isLow) return 'yellow';
    return 'blue lighten-2';
  }

  if (isOver) return 'red lighten-5';
  if (isLow) return 'yellow lighten-5';
  return 'default';
}

function showValidation(id: number) {
  selectedValidationId.value = id;
  drawer.value = true;
}

function handleItemSelected({ item, value }: { item: { raw: { tagId: number; conjugate: ConjugateDto } }, value: boolean }) {
  const conjugate = getCanonicalConjugate(item.raw.conjugate);
  onSelected(item.raw.tagId, conjugate, value);
}

function getCanonicalConjugate(conjugate: ConjugateDto) {
  return conjugateStore.getConjugate(conjugate.id) ?? conjugate;
}

onMounted(() => {
  document.onkeydown = (evt) => {
    if (drawer.value && evt.key === 'Escape') {
      drawer.value = false;
    }
  };
});
</script>

<style scoped>
.content {
  font-size: small;
  margin: 8px;
  padding: 0;
}
.subheader {
  font-weight: bold;
}
.chip {
  margin-right: 5px;
}
</style>
