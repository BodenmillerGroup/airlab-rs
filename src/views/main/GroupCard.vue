<template>
  <v-card tile class="group-card">
    <div class="group-card__layout">
      <div class="group-card__primary-action">
        <v-btn
          v-if="isMember || user.isAdmin"
          class="group-card__action-button"
          color="primary"
          variant="outlined"
          :to="{ name: 'main-group', params: { groupId: group.id } }"
        >
          Open
        </v-btn>

        <v-btn
          v-else-if="group.isOpen"
          class="group-card__action-button"
          color="primary"
          variant="outlined"
          @click="joinGroup"
        >
          Request Access
        </v-btn>

        <div v-else class="group-card__action-spacer" aria-hidden="true" />
      </div>

      <div class="group-card__content">
        <div class="group-card__title text-h5">{{ group.name }}</div>
        <span v-if="group.url" class="text-caption group-card__link">
          <v-icon small>mdi-link</v-icon>{{ group.url }}
        </span>
        <div v-if="group.institution" class="group-card__institution">
          {{ group.institution }}
        </div>
      </div>
    </div>
  </v-card>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import type { GroupView } from '@/modules/group/types'
import type { ProfileDto } from '@/modules/user/types'
import { useGroupStore } from '@/stores/group'
import { useMemberStore } from '@/stores/member'

// ✅ Props
const props = defineProps<{
  user: ProfileDto
  group: GroupView
}>()

// ✅ Pinia Store
const groupStore = useGroupStore()
const memberStore = useMemberStore()

const isMember = computed(() => {
  const members = memberStore.getMembersForGroup(props.group.id) || []
  const activeMemberIds = members
    .filter((m: any) => m.isActive)
    .map((m: any) => m.userId)

  return activeMemberIds.includes(props.user.id)
})

// ✅ Call Pinia action to join group
const joinGroup = async () => {
  await groupStore.joinGroup(props.group.id)
}
</script>

<style scoped>
.group-card {
  width: 100%;
  padding: 12px 16px;
}

.group-card__layout {
  display: flex;
  align-items: stretch;
  gap: 16px;
}

.group-card__primary-action {
  flex: 0 0 140px;
  width: 140px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.group-card__content {
  min-width: 0;
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 6px;
}

.group-card__action-button {
  min-width: 108px;
  background: white;
}

.group-card__action-spacer {
  width: 100%;
  height: 1px;
}

.group-card__title {
  line-height: 1.2;
}

.group-card__link {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  word-break: break-word;
}

.group-card__institution {
  color: rgba(0, 0, 0, 0.7);
}
</style>
