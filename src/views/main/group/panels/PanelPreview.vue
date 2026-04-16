<template>
  <v-sheet elevation="2">
    <v-data-table
      :headers="headers"
      :items="items"
      :items-per-page="-1"
      hide-default-footer
      disable-filtering
      disable-pagination
      density="compact"
      fixed-header
      :height="height"
      @click:row="clickRow"
    >
      <template #item.tag="{ item }">
        <span :class="getClass(unwrapItem(item)?.status ?? 0)">
          {{ unwrapItem(item)?.tagName ?? '' }}
        </span>
      </template>
      <template #item.mass="{ item }">
        <span :class="getClass(unwrapItem(item)?.status ?? 0)">
          {{ unwrapItem(item)?.tagMw ?? '' }}
        </span>
      </template>
      <template #item.proteinName="{ item }">
        <span :class="getClass(unwrapItem(item)?.status ?? 0)">
          {{ unwrapItem(item)?.proteinName ?? '' }}
        </span>
      </template>
      <template #item.cloneName="{ item }">
        <span :class="getClass(unwrapItem(item)?.status ?? 0)">
          {{ unwrapItem(item)?.cloneName ?? '' }}
        </span>
      </template>
      <template #item.lotCollectionName="{ item }">
        <span :class="getClass(unwrapItem(item)?.status ?? 0)">
          {{ unwrapItem(item)?.lotCollectionName ?? '' }}
        </span>
      </template>
      <template #item.tubeNumber="{ item }">
        <span :class="getClass(unwrapItem(item)?.status ?? 0)">
          {{ unwrapItem(item)?.tubeNumber ?? '' }}
        </span>
      </template>
    </v-data-table>
  </v-sheet>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { ConjugateView } from '@/modules/conjugate/types';
import { usePanelStore } from '@/stores/panel';
import { useResponsiveStore } from '@/stores/responsive';

const props = defineProps<{
  conjugates: Map<number, Set<ConjugateView>>;
  expanded?: number;
  onSelectConjugate?: (conjugate: ConjugateView) => void;
}>();

const panelStore = usePanelStore();
const responsiveStore = useResponsiveStore();

const items = ref<ConjugateView[]>([]);

// Recalculate when conjugates prop changes
watch(
  () => props.conjugates,
  (value) => {
    const merged: ConjugateView[] = [];
    value.forEach((set) => {
      merged.push(...Array.from(set));
    });
    items.value = merged;
  },
  { immediate: true }
);

// Table height based on expansion state
const height = computed(() =>
  props.expanded === 0
    ? responsiveStore.height - 542
    : responsiveStore.height - 182
);

// Row click => update active tag
function clickRow(_: unknown, row: { item: ConjugateView | { raw: ConjugateView } }) {
  const raw = unwrapItem(row.item);
  if (!raw) return;
  panelStore.activePanelTagId = raw.tagId;
  props.onSelectConjugate?.(raw);
}

function isWrappedItem(item: unknown): item is { raw: ConjugateView } {
  return !!item && typeof item === 'object' && 'raw' in item;
}

function unwrapItem(
  item: ConjugateView | { raw: ConjugateView } | undefined
): ConjugateView | undefined {
  if (!item) return undefined;
  return isWrappedItem(item) ? item.raw : item;
}

// Class for styling
function getClass(status: number) {
  if (status === 2) return 'empty';
  if (status === 1) return 'low';
  return '';
}

const rawHeaders = [
  {
    title: 'Tag',
    key: 'tagName',
    sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? ''),
  },
  {
    title: 'Mass',
    key: 'tagMw',
    align: 'end',
    sort: (a: number, b: number) => (a ?? 0) - (b ?? 0),
  },
  {
    title: 'Protein',
    key: 'proteinName',
    sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? ''),
  },
  {
    title: 'Clone',
    key: 'cloneName',
    sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? ''),
  },
  {
    title: 'Collection',
    key: 'lotCollectionName',
    sort: (a: string, b: string) => (a ?? '').localeCompare(b ?? ''),
  },
  {
    title: 'Tube',
    key: 'tubeNumber',
    align: 'end',
  },
] as const


const headers = rawHeaders.map(h => ({ sortable: false, ...h }))
</script>

<style scoped>
.low {
  color: red;
}
.empty {
  text-decoration: line-through;
}
</style>
