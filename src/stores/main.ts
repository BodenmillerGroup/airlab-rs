import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import router from '@/router'

import { api } from '@/modules/user/api'
import { ApiManager } from '@/utils/api'
import { getLocalToken, removeLocalToken, saveLocalToken } from '@/utils/auth'

import type { ProfileDto, UpdateProfileDto, UpdatePasswordDto } from '@/modules/user/types'
import type { HTTPError } from 'ky'

interface Notification {
  content: string
  color?: string
  timeout?: number
}

export const useMainStore = defineStore('main', () => {
  // 📦 State
  const token = ref('')
  const isLoggedIn = ref<boolean | null>(null)
  const logInError = ref(false)
  const mfaPending = ref(false)
  const tempToken = ref('')
  const userProfile = ref<ProfileDto | null>(null)

  const dashboardMiniDrawer = ref(false)
  const dashboardShowDrawer = ref(true)

  const notifications = ref<Notification[]>([])
  const error = ref<string | null>(null)

  const processing = ref(false)
  const processingProgress = ref(0)
  const proteinLimit = ref(5)

  // 🧠 Getters
  const isAdmin = computed(() => userProfile.value?.isAdmin ?? false)
  const firstNotification = computed(() => notifications.value[0] ?? null)

  // 🔔 Notifications
  function addNotification(notification: Notification) {
    notifications.value.push({
      timeout: 5000,
      ...notification,
    })
  }

  function clearNotifications() {
    notifications.value = []
  }

  function setDashboardShowDrawer(value: boolean) {
    dashboardShowDrawer.value = value
  }

  // ❌ Centralized API error handler
  async function checkApiError(err: unknown) {
    let message = 'Unknown error occurred'

    if (typeof err === 'string') {
      message = err
    } else if (err instanceof Error) {
      const httpError = err as HTTPError
      if (httpError.response) {
        message = `${httpError.response.status}: ${httpError.response.statusText}`
      } else {
        message = httpError.message
      }
    }

    error.value = message
    addNotification({ content: message, color: 'error' })
  }

  // 🔐 Auth
  async function logIn({ username, password }: { username: string; password: string }) {
    try {
      const data: any = await api.logInGetToken(username, password);
      const { token: receivedToken, mfaRequired } = data;
      if (receivedToken) {
        if (mfaRequired) {
          tempToken.value = receivedToken;
          mfaPending.value = true;
          logInError.value = false;
          return;
        } else {
          completeLogin(receivedToken);
        }
      } else {
        await logOut();
      }
    } catch (error) {
      logInError.value = true;
      await checkApiError(error)
      await logOut();
    }
  }

  async function verifyOtp(code: string): Promise<boolean> {
    try {
      const success = await api.verifyOtp(tempToken.value, code)
      if (success) {
        await completeLogin(tempToken.value)
        return true
      }
      return false
    } catch {
      return false
    }
  }

  async function completeLogin(newToken: string) {
    saveLocalToken(newToken)
    ApiManager.init(newToken)

    token.value = newToken
    isLoggedIn.value = true
    logInError.value = false
    mfaPending.value = false

    await getUserProfile()
    routeLoggedIn()
    addNotification({ content: 'Logged in', color: 'success' })
  }

  async function logOut() {
    await removeLogIn()
    routeLogOut()
  }

  async function userLogOut() {
    await logOut()
    addNotification({ content: 'Logged out', color: 'success' })
  }

  async function removeLogIn() {
    removeLocalToken()
    token.value = ''
    isLoggedIn.value = false
    userProfile.value = null
  }

  async function checkLoggedIn() {
    if (!isLoggedIn.value) {
      let localToken = token.value

      if (!localToken) {
        const stored = getLocalToken()
        if (stored) {
          token.value = stored
          ApiManager.init(stored)
          localToken = stored
        }
      }

      if (localToken) {
        try {
          const data = await api.getMe()
          userProfile.value = data
          isLoggedIn.value = true
        } catch {
          await removeLogIn()
        }
      } else {
        await removeLogIn()
      }
    }
  }

  // 👤 Profile
  async function getUserProfile() {
    try {
      const data = await api.getMe()
      userProfile.value = data
    } catch (error) {
      await checkApiError(error)
    }
  }

  async function updateUserProfile(payload: UpdateProfileDto) {
    try {
      const data = await api.updateMe(payload)
      userProfile.value = data
      addNotification({ content: 'Profile successfully updated', color: 'success' })
    } catch (error) {
      await checkApiError(error)
    }
  }

  async function updatePassword(payload: UpdatePasswordDto) {
    try {
      await api.updatePassword(payload)
      addNotification({ content: 'Password successfully updated', color: 'success' })
    } catch (error) {
      await checkApiError(error)
    }
  }

  async function passwordRecovery(email: string) {
    try {
      await api.passwordRecovery(email)
      addNotification({ content: 'Password recovery email sent', color: 'success' })
      await logOut()
    } catch (error) {
      await checkApiError(error)
    }
  }

  async function resetPassword({ password, token }: { password: string; token: string }) {
    try {
      await api.resetPassword(password, token)
      addNotification({ content: 'Password successfully reset', color: 'success' })
      await logOut()
    } catch (error) {
      await checkApiError(error)
    }
  }

  // ⛳ Navigation
  function routeLogOut() {
    router.push('/login')
    addNotification({ content: 'You have been logged out', color: 'warning' })
  }

  function routeLoggedIn() {
    if (['/', '/login'].includes(router.currentRoute.value.path)) {
      router.push('/main')
    }
  }

  return {
    // State
    token,
    isLoggedIn,
    logInError,
    mfaPending,
    tempToken,
    userProfile,
    dashboardMiniDrawer,
    dashboardShowDrawer,
    notifications,
    error,
    processing,
    processingProgress,
    proteinLimit,

    // Getters
    isAdmin,
    firstNotification,

    // Actions
    addNotification,
    clearNotifications,
    checkApiError,

    // Auth
    //logIn,
    verifyOtp,
    completeLogin,
    logIn,
    logOut,
    userLogOut,
    checkLoggedIn,
    removeLogIn,

    // Profile
    getUserProfile,
    updateUserProfile,
    updatePassword,
    passwordRecovery,
    resetPassword,

    // UI
    setDashboardShowDrawer,

    // Navigation
    routeLogOut,
    routeLoggedIn,
  }
})
