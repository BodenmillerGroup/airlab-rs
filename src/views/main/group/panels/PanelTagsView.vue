<template>
  <v-card tile>
    <v-toolbar dense flat>
      <v-text-field
        v-model="search"
        label="Search tags"
        single-line
        hide-details
        clearable
        dense
      >
        <template #append>
          <v-icon dense>mdi-magnify</v-icon>
        </template>
      </v-text-field>
    </v-toolbar>

    <v-toolbar dense flat>
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
    </v-toolbar>

    <v-toolbar dense flat>
      <v-switch v-model="showOnlyMetals" label="Metals" hide-details inset dense />
      <v-spacer />
      <v-btn-toggle v-model="sortDesc" mandatory dense>
        <v-btn depressed :value="false" x-small>
          <v-icon x-small>mdi-arrow-up</v-icon>
        </v-btn>
        <v-btn depressed :value="true" x-small>
          <v-icon x-small>mdi-arrow-down</v-icon>
        </v-btn>
      </v-btn-toggle>
    </v-toolbar>

    <v-list dense class="overflow-y-auto" :height="height" nav>
      <v-list-item
        v-for="tag in filteredTags"
        :key="tag.id"
        :active="selectedTag === tag.id"
        color="primary"
        @click="selectedTag = tag.id"
      >
        <template #prepend>
          <v-avatar :color="getColor(tag)" size="30">
            {{ tag.isMetal ? 'M' : tag.isFluorophore ? 'F' : 'O' }}
          </v-avatar>
        </template>
        <v-list-item-title>
          {{ tag.isMetal ? tag.name + tag.mw : tag.name }}
        </v-list-item-title>
      </v-list-item>
    </v-list>
  </v-card>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { TagDto } from '@/modules/tag/types';
import { useTagStore } from '@/stores/tag';
import { usePanelStore } from '@/stores/panel';
import { useResponsiveStore } from '@/stores/responsive';

const props = defineProps<{
  expanded?: number;
}>();

// Stores
const tagStore = useTagStore();
const panelStore = usePanelStore();
const responsiveStore = useResponsiveStore();

// UI State
const search = ref<string | null>(null);
const sortBy = ref<'name' | 'mass'>('name');
const sortDesc = ref(false);
const showOnlyMetals = ref(false);

// Sort options
const sortByOptions = [
  { id: 'name', title: 'Name' },
  { id: 'mass', title: 'Mass' },
];

// Tag selection binding
const selectedTag = computed({
  get: () => panelStore.activePanelTagId,
  set: (value: number | null) => {
    panelStore.activePanelTagId = value ?? null;
  },
});

// Computed list height
const height = computed(() =>
  props.expanded === 0
    ? responsiveStore.height - 686
    : responsiveStore.height - 326
);

// Filtering + Sorting
const filteredTags = computed(() => {
  let items = showOnlyMetals.value
    ? tagStore.tags.filter((t) => t.isMetal)
    : tagStore.tags;

  if (search.value) {
    const s = search.value.toLowerCase().trim();
    items = items.filter((t) => {
      const label = t.isMetal ? `${t.name}${t.mw}` : t.name;
      return label.toLowerCase().includes(s);
    });
  }

  return [...items].sort((a, b) => {
    if (sortBy.value === 'name') {
      return sortDesc.value
        ? b.name.localeCompare(a.name)
        : a.name.localeCompare(b.name);
    }

    const aMw = a.mw ?? Infinity;
    const bMw = b.mw ?? Infinity;
    return sortDesc.value ? bMw - aMw : aMw - bMw;
  });
});

function getColor(tag: TagDto) {
  if (tag.isMetal) return 'blue lighten-3';
  if (tag.isFluorophore) return 'purple lighten-4';
  return 'grey lighten-3';
}
</script>
