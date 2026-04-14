<template>
  <v-card flat>
    <v-card-title>Tag Details</v-card-title>
    <v-card-text>
      <v-tabs v-model="tab" density="comfortable" grow>
        <v-tab :value="0">Info</v-tab>
      </v-tabs>

      <v-tabs-window v-model="tab">
        <v-tabs-window-item :value="0">
          <TagView :tag-id="tag.id" />
        </v-tabs-window-item>
      </v-tabs-window>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import TagView from "@/views/main/group/tags/TagView.vue";
import type { TagDto } from "@/modules/tag/types";

// Props
const props = defineProps<{
  tag: TagDto;
}>();

const tab = ref(0);

</script>
