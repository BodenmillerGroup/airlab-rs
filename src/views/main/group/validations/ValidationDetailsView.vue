<template>
  <v-card flat>
    <v-card-title>Validation Details</v-card-title>
    <v-card-text>
      <ValidationView
        v-if="validation"
        :validation="validation"
        :group-id="activeGroupId"
        :api-url="apiUrl"
      />
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed, onMounted, watch } from "vue";
import { useGroupStore } from "@/stores/group";
import { useValidationStore } from "@/stores/validation";
import { useCloneStore } from "@/stores/clone";
import { useProteinStore } from "@/stores/protein";
import { useLotStore } from "@/stores/lot";
import { useConjugateStore } from "@/stores/conjugate";
import { useSpeciesStore } from "@/stores/species";
import { useMemberStore } from "@/stores/member";
import { useUserStore } from "@/stores/user";
import ValidationView from "@/views/main/group/validations/ValidationView.vue";
import { apiUrl } from "@/env";
import { useValidationFileStore } from "@/stores/validation_file";
import type { ValidationDto, ValidationView as ValidationViewDto } from "@/modules/validation/types";

// Props
const props = defineProps<{
  validationId: number;
}>();

// Stores
const groupStore = useGroupStore();
const validationStore = useValidationStore();
const cloneStore = useCloneStore();
const proteinStore = useProteinStore();
const lotStore = useLotStore();
const conjugateStore = useConjugateStore();
const speciesStore = useSpeciesStore();
const memberStore = useMemberStore();
const userStore = useUserStore();
const validationFileStore = useValidationFileStore();

// Computed
const activeGroupId = computed(() => groupStore.activeGroupId);
const validationItem = computed<ValidationDto | null>(() => {
  return validationStore.getValidation(props.validationId) ?? null;
});
const validation = computed(() => {
  if (!validationItem.value) return null;
  const item = validationItem.value;
  const clone = cloneStore.getClone(item.cloneId);
  const protein = clone?.proteinId ? proteinStore.getProtein(clone.proteinId) : undefined;
  const lot = item.lotId ? lotStore.getLot(item.lotId) : undefined;
  const species = item.speciesId ? speciesStore.getSpecies(item.speciesId) : undefined;
  const conjugate = item.conjugateId ? conjugateStore.getConjugate(item.conjugateId) : undefined;
  const member = memberStore.getMemberById(item.createdBy);
  const user = member?.userId ? userStore.getUserById(member.userId) : undefined;

  return {
    ...item,
    cloneName: clone?.name ?? "—",
    proteinId: protein?.id ?? 0,
    proteinName: protein?.name ?? "—",
    lotName: lot?.name ?? "-",
    userId: user?.id ?? 0,
    userName: user?.name ?? "—",
    lotNumber: lot?.number ?? "—",
    tubeNumber: conjugate?.tubeNumber ?? "",
    speciesName: species?.name ?? "-",
    validationFiles: validationFileStore.byValidationId[props.validationId] ?? [],
  } satisfies ValidationViewDto;
});

async function loadValidationDetails(id: number) {
  const item = await validationStore.getValidationById(id);
  if (!item) {
    return;
  }

  await Promise.all([
    cloneStore.fetchByIds([item.cloneId]),
    item.conjugateId ? conjugateStore.fetchByIdsV2([item.conjugateId]) : Promise.resolve([]),
    item.lotId ? lotStore.fetchByIdsV2([item.lotId]) : Promise.resolve([]),
    item.speciesId ? speciesStore.fetchByIdsV2([item.speciesId]) : Promise.resolve([]),
    memberStore.fetchByIds([item.createdBy]),
  ]);

  const clone = cloneStore.getClone(item.cloneId);
  if (clone?.proteinId) {
    await proteinStore.fetchByIdsV2([clone.proteinId]);
  }

  const member = memberStore.getMemberById(item.createdBy);
  if (member?.userId) {
    await userStore.fetchByIds([member.userId]);
  }
}

// Load data
onMounted(async () => {
  await Promise.all([
    loadValidationDetails(props.validationId),
    validationFileStore.getValidationFiles(props.validationId),
  ]);
});

watch(
  () => props.validationId,
  async (id) => {
    await Promise.all([
      loadValidationDetails(id),
      validationFileStore.getValidationFiles(id),
    ]);
  }
);
</script>
