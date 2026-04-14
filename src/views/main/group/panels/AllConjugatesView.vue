<template>
  <v-card tile>
    <v-toolbar flat dense class="mb-2">
      <v-text-field v-model="search" label="Search conjugates" single-line hide-details clearable dense>
        <template #append>
          <v-icon dense>mdi-magnify</v-icon>
        </template>
      </v-text-field>

      <v-switch
        v-model="showEmpty"
        label="Show empty"
        hide-details
        inset
        dense
        class="ml-2"
        style="width: 200px"
      />

      <v-spacer />

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
      <div v-for="[tagId, items] in tagMap">
        <v-card-subtitle class="tag-header">
          {{
            tagStore.getTag(tagId).isMetal
              ? tagStore.getTag(tagId).name + tagStore.getTag(tagId).mw
              : tagStore.getTag(tagId).name
          }}
        </v-card-subtitle>

        <v-data-iterator
          :value="selectedConjugates"
          :items="items"
          item-key="id"
          :items-per-page="-1"
          hide-default-footer
          disable-pagination
          disable-filtering
          disable-sort
          @item-selected="({ item, value }) => onSelectedInternal(item.raw.tagId, item.raw.id, value)"
        >
          <template #default="{ items }">
            <v-row>
              <v-sheet
                v-for="item in items"
                :key="item.raw.id"
                tile
                width="180"
                class="ma-1 pa-0"
                @click.prevent="onSelectedInternal(item.raw.tagId, item.raw.id, !isConjugateSelected(item.raw.id))"
                :color="getConjugateColor(conjugateStore.getConjugate(item.raw.id), isConjugateSelected(item.raw.id))"
              >
                <div class="content">
                  <div><span class="subheader">Tube:</span> {{ item.raw.tubeNumber }}</div>
                  <div><span class="subheader">Protein:</span> {{ item.raw.proteinName }}</div>
                  <div><span class="subheader">Clone:</span> {{ item.raw.cloneName }}</div>
                </div>
              </v-sheet>
            </v-row>
          </template>
        </v-data-iterator>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import type { ConjugateDto, ConjugateView } from '@/modules/conjugate/types';
import { ConjugateStatus } from '@/modules/conjugate/ConjugateStatus';

import { useConjugateStore } from '@/stores/conjugate';
import { useTagStore } from '@/stores/tag';
import { useConjugates } from '@/composables/useConjugates';

const props = defineProps<{
  selectedConjugates?: ConjugateDto[];
  onSelected: (tagId: number, conjugate: ConjugateDto, selected: boolean) => void;
}>();

const conjugateStore = useConjugateStore();
const tagStore = useTagStore();
const { items: conjugates } = useConjugates()

const sortByOptions = [
  { id: 'protein', title: 'Protein' },
  { id: 'clone', title: 'Clone' },
  { id: 'tube', title: 'Tube' },
];

const sortBy = ref<'protein' | 'clone' | 'tube'>('protein');
const sortDesc = ref(false);
const search = ref<string | null>(null);
const showEmpty = ref(false);

const items = computed(() => {
  let filtered = showEmpty.value
    ? conjugates.value
    : conjugates.value.filter((i) => i.status !== ConjugateStatus.Finished);

  if (search.value) {
    const term = search.value.toLowerCase().trim();
    filtered = filtered.filter((item) => {
      const cloneName = item.cloneName.toLowerCase();
      const proteinName = item.proteinName.toLowerCase();
      return cloneName.includes(term) || proteinName.includes(term);
    });
  }

  switch (sortBy.value) {
    case 'tube':
      filtered.sort((a, b) =>
        sortDesc.value ? b.tubeNumber - a.tubeNumber : a.tubeNumber - b.tubeNumber
      );
      break;
    case 'protein':
      filtered.sort((a, b) =>
        sortDesc.value
          ? b.proteinName.localeCompare(a.proteinName)
          : a.proteinName.localeCompare(b.proteinName)
      );
      break;
    case 'clone':
      filtered.sort((a, b) =>
        sortDesc.value
          ? b.cloneName.localeCompare(a.cloneName)
          : a.cloneName.localeCompare(b.cloneName)
      );
      break;
  }

  return filtered;
});

const tagMap = computed(() => {
  const map = new Map<number, ConjugateView[]>();
  items.value.forEach((item) => {
    if (!map.has(item.tagId)) {
      map.set(item.tagId, []);
    }
    map.get(item.tagId)!.push(item);
  });

  return new Map(
    [...map.entries()]
      .sort((a, b) => {
        const tagA = tagStore.getTag(a[0]);
        const tagB = tagStore.getTag(b[0]);
        return tagA.name.localeCompare(tagB.name);
      })
      .sort((a, b) => {
        const tagA = tagStore.getTag(a[0]);
        const tagB = tagStore.getTag(b[0]);
        if (tagA.mw === null) return 1;
        if (tagB.mw === null) return -1;
        return tagA.mw - tagB.mw;
      })
  );
});

const selectedConjugateIds = computed(() =>
  new Set((props.selectedConjugates ?? []).map((conjugate) => conjugate.id))
);

function isConjugateSelected(conjugateId: number): boolean {
  return selectedConjugateIds.value.has(conjugateId);
}

function getConjugateColor(conjugate: ConjugateDto | undefined, isSelected: boolean): string {
  if (!conjugate) return 'default';
  const isOver = conjugate.status === ConjugateStatus.Finished;
  const isLow = conjugate.status === ConjugateStatus.Low;

  if (isSelected) {
    return isOver ? 'red' : isLow ? 'yellow' : 'blue lighten-2';
  }

  return isOver ? 'red lighten-5' : isLow ? 'yellow lighten-5' : 'default';
}

function onSelectedInternal(tagId: number, conjugateId: number, selected: boolean) {
  const conjugate = conjugateStore.getConjugate(conjugateId);
  if (!conjugate) return;
  props.onSelected(tagId, conjugate, selected);
}
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
.tag-header {
  background-color: #81d4fa;
  font-weight: bold;
}
</style>
