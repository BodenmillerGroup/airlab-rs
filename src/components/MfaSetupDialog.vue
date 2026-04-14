<template>
  <v-dialog :model-value="modelValue" @update:model-value="closeDialog" max-width="500">
    <v-card>
      <v-card-title>
        <span class="text-h6">Enable MFA</span>
      </v-card-title>

      <v-card-text>
        <div v-if="loading">Generating secret...</div>

        <div v-else>
          <p>Scan this QR code with Microsoft Authenticator:</p>

          <div v-html="qrCodeSvg" />

          <v-text-field
            class="mt-4"
            label="One-Time Code"
            v-model="otp"
            maxlength="6"
            required
          />

          <div
            v-if="message"
            :class="{ 'text-success': isSuccess, 'text-error': !isSuccess }"
          >
            {{ message }}
          </div>
        </div>
      </v-card-text>

      <v-card-actions>
        <v-spacer />
        <v-btn text @click="emit('update:modelValue', false)">Cancel</v-btn>
        <v-btn color="primary" @click="verifyOtp" :disabled="!otp || loading">
          Verify
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

const props = defineProps<{
  email: string;
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
}>();

const secret = ref("");
const qrCodeSvg = ref("");
const otp = ref("");
const message = ref("");
const isSuccess = ref(false);
const loading = ref(false);

watch(
  () => props.modelValue,
  async (val) => {
    if (val) {
      await generateMfaSecret();
    } else {
      resetState();
    }
  }
);

async function generateMfaSecret() {
  loading.value = true;
  message.value = "";
  otp.value = "";

  try {
    const response = await fetch("/api/v1/users/setupmfa", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email: props.email }),
    });

    if (!response.ok) throw new Error("Failed to generate secret");

    const data = await response.json();

    secret.value = data.secret;
    qrCodeSvg.value = data.qr_code; // 👈 backend must return raw SVG string
  } catch (err) {
    message.value = "Failed to set up MFA.";
  } finally {
    loading.value = false;
  }
}

async function verifyOtp() {
  loading.value = true;
  message.value = "";

  try {
    const response = await fetch("/api/v1/users/verifymfa", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        email: props.email,
        code: otp.value,
        secret: secret.value,
      }),
    });

    const result = await response.json();

    isSuccess.value = result.success;
    message.value = result.message;

    if (result.success) {
      setTimeout(() => emit("update:modelValue", false), 1500);
    }
  } catch (err) {
    message.value = "Failed to verify OTP.";
    isSuccess.value = false;
  } finally {
    loading.value = false;
  }
}

function resetState() {
  secret.value = "";
  qrCodeSvg.value = "";
  otp.value = "";
  message.value = "";
  isSuccess.value = false;
  loading.value = false;
}

function closeDialog() {
  emit("update:modelValue", false);
}
</script>
