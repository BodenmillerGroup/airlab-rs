<template>
  <span>
    <v-btn @click="trigger" color="primary" text>
      <v-icon small left>mdi-cloud-upload</v-icon>
      Upload
    </v-btn>
    <input
      :multiple="multiple"
      class="visually-hidden"
      type="file"
      @change="onFiles"
      ref="fileInput"
    />
  </span>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useGroupStore } from '@/stores/group';

interface Props {
  multiple?: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: 'files', files: FileList): void;
}>();

const fileInput = ref<HTMLInputElement | null>(null);
const groupStore = useGroupStore();

const trigger = () => {
  fileInput.value?.click();
};

const onFiles = async (e: Event) => {
  const input = e.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  const formData = new FormData();
  formData.append('file', file, file.name);

  input.value = ''; // reset file input
  await groupStore.importGroupData(formData);
  emit('files', input.files as FileList);
};
</script>

<style scoped>
.visually-hidden {
  position: absolute !important;
  height: 1px;
  width: 1px;
  overflow: hidden;
  clip: rect(1px, 1px, 1px, 1px);
}
</style>
