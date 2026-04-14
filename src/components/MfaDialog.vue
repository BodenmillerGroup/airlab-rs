<template>
  <v-dialog v-model="dialogVisible" persistent max-width="400px">
    <v-card>
      <v-card-title>Enter OTP</v-card-title>
      <v-card-text>
        <v-text-field
          label="Authentication Code"
          v-model="otp"
          autofocus
          @keyup.enter="submitOtp"
        />
        <v-alert v-if="otpError" type="error" dense>
          Invalid OTP. Please try again.
        </v-alert>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" @click="submitOtp">Verify</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { watch, ref } from 'vue';
import { useMainStore } from '@/stores/main';

const props = defineProps<{ modelValue: boolean }>();
const emit = defineEmits(['close', 'update:modelValue']);

const store = useMainStore();

const dialogVisible = ref(false);

watch(
  () => props.modelValue,
  (val) => {
    dialogVisible.value = val;
  },
  { immediate: true }
);

function closeDialog() {
  dialogVisible.value = false;
  emit('update:modelValue', false);
  emit('close');
}

const otp = ref('');
const otpError = ref(false);

async function submitOtp() {
  try {
    const success = await store.verifyOtp(otp.value);
    if (success) {
      closeDialog();
    } else {
      otpError.value = true;
    }
  } catch {
    otpError.value = true;
  }
}
</script>
