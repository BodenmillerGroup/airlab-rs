<template>
  <v-col>
    <v-toolbar density="compact">
      <v-toolbar-title>Tags</v-toolbar-title>
      <v-spacer />
      <v-btn :to="`/main/groups/${activeGroupId}/tags/create`" variant="text" color="primary">
        Create Tag
      </v-btn>
    </v-toolbar>

    <v-expansion-panels>
      <v-expansion-panel>
        <v-expansion-panel-title>
          <FilterSummary />
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <FilterField key-name="tagName" class="mb-1" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-text-field
      v-model="globalSearch"
      label="Search all tag fields"
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
        :loading="tagStore.loading"
        :items-length="tagStore.total"
        v-model:items-per-page="tableItemsPerPage"
        v-model:page="tablePage"
        :footer-props="tableFooterProps"
        show-expand
        v-model:expanded="expanded"
        v-model:sort-by="sortBy"
        item-value="id"
      >

        <template #item.isMetal="{ item }">
          <v-icon v-if="item.isMetal">mdi-check</v-icon>
        </template>
        <template #item.isFluorophore="{ item }">
          <v-icon v-if="item.isFluorophore">mdi-check</v-icon>
        </template>
        <template #item.isEnzyme="{ item }">
          <v-icon v-if="item.isEnzyme">mdi-check</v-icon>
        </template>
        <template #item.isBiotin="{ item }">
          <v-icon v-if="item.isBiotin">mdi-check</v-icon>
        </template>
        <template #item.isOther="{ item }">
          <v-icon v-if="item.isOther">mdi-check</v-icon>
        </template>

        <template #item.status="{ item }">
          <v-chip :color="getTagStatusColor(item)" small dark label>
            {{ tagStatusToString(item.status) }}
          </v-chip>
        </template>

        <template #item.action="{ item }">
          <v-menu>
            <template #activator="{ props }">
              <v-btn icon v-bind="props">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
            <v-list>
              <v-list-item
                :to="{ name: 'main-group-tags-edit', params: { groupId: activeGroupId, id: item.id } }"
              >
                <v-list-item-title>Edit</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>

          <v-tooltip activator="parent" location="bottom">Show details</v-tooltip>
          <v-btn icon @click.stop="showDetails(item)">
            <v-icon>mdi-information-outline</v-icon>
          </v-btn>
        </template>

        <template #expanded-row="{ columns, item }">
          <td :colspan="columns.length">
            <TagExpandedView :tag="item" />
          </td>
        </template>
      </v-data-table-server>
    </v-card>

    <v-navigation-drawer v-model="drawer" location="right" temporary width="400">
      <TagDetailsView v-if="drawer && detailsItem" :tag="detailsItem" />
    </v-navigation-drawer>
  </v-col>
</template>
<script lang="ts" setup>
import { ref, computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useTagStore } from "@/stores/tag";
import { useGroupStore } from "@/stores/group";
import { useFilterStore } from '@/stores/useFilterStore'
import TagDetailsView from "@/views/main/group/tags/TagDetailsView.vue";
import TagExpandedView from "@/views/main/group/tags/TagExpandedView.vue";
import FilterField from '@/components/FilterField.vue'
import FilterSummary from '@/components/FilterSummary.vue'
import { getTagStatusColor, tagStatusToString } from "@/utils/converters";
import type { TagDto } from "@/modules/tag/types";
import { storeToRefs } from 'pinia'
import { useServerTablePagination } from '@/composables/useServerTablePagination'

// 🧠 Store setup
const tagStore = useTagStore();
const groupStore = useGroupStore();
const filterStore = useFilterStore()
const { page, limit } = storeToRefs(tagStore)
const route = useRoute();

// 🔢 Data
const drawer = ref(false);
const search = ref("");
const detailsItem = ref<TagDto | null>(null);
const expanded = ref<any[]>([]);
const sortBy = ref<Array<{ key: string; order?: 'asc' | 'desc' }>>([
  { key: 'name', order: 'asc' },
]);

// 📦 State
const activeGroupId = computed(() => groupStore.activeGroupId);
const isGroupAdmin = computed(() => groupStore.isGroupAdmin);
const globalSearch = computed({
  get: () => filterStore.filters.tagGlobalSearch ?? '',
  set: (value: string) => filterStore.setFilter('tagGlobalSearch', value),
})
import { useTags } from '@/composables/useTags'
const { items, loading, reload } = useTags({ sortBy, globalFilter: globalSearch })
const { tablePage, tableItemsPerPage, tableFooterProps } = useServerTablePagination(page, limit)

// 🧠 Headers (data-table)
const rawHeaders = [
  { title: "Name", key: "name", align: "start", sortable: true },
  { title: "Mass", key: "mw", align: "end", sortable: true },
  { title: "Metal", key: "isMetal", sortable: true },
  { title: "Fluorophore", key: "isFluorophore", sortable: true },
  { title: "Enzyme", key: "isEnzyme", sortable: true },
  { title: "Biotin", key: "isBiotin", sortable: true },
  { title: "Other", key: "isOther", sortable: true },
  { title: "Description", key: "description", sortable: true },
  { title: "Emission", key: "emission", align: "end", sortable: true },
  { title: "Excitation", key: "excitation", align: "end", sortable: true },
  { title: "Status", key: "status", sortable: true },
  { title: 'Actions', key: 'action', sortable: false, width: "130", },
  { title: "", key: "data-table-expand" }
] as const


const headers = rawHeaders.map(h => ({
  sortable: h.key !== 'action' && h.key !== 'data-table-expand',
  ...h,
}))

// 👁️ Show drawer with tag details
function showDetails(tag: TagDto) {
  detailsItem.value = tag;
  drawer.value = true;
}

// 🧹 Delete (kept for future)
async function deleteTag(id: number) {
  if (confirm("Are you sure you want to delete the tag?")) {
    if (confirm("All children entities will be deleted!")) {
      await tagStore.deleteTag(id);
    }
  }
}

</script>
const { page, limit } = storeToRefs(tagStore)
