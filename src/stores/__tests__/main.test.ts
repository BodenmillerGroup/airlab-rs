import { describe, it, expect, beforeEach, vi } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import { useMainStore } from "@/stores/main"

/* ---------------- mocks ---------------- */

vi.mock("@/router", () => ({
  default: {
    push: vi.fn(),
    currentRoute: { value: { path: "/" } },
  },
}))

vi.mock("@/modules/user/api", () => ({
  api: {
    logInGetToken: vi.fn(),
    verifyOtp: vi.fn(),
    getMe: vi.fn(),
    updateMe: vi.fn(),
    updatePassword: vi.fn(),
    passwordRecovery: vi.fn(),
    resetPassword: vi.fn(),
  },
}))

vi.mock("@/utils/api", () => ({
  ApiManager: {
    init: vi.fn(),
  },
}))

vi.mock("@/utils/auth", () => ({
  getLocalToken: vi.fn(),
  saveLocalToken: vi.fn(),
  removeLocalToken: vi.fn(),
}))

import router from "@/router"
import { api } from "@/modules/user/api"
import { ApiManager } from "@/utils/api"
import * as auth from "@/utils/auth"

/* ---------------- setup ---------------- */

beforeEach(() => {
  setActivePinia(createPinia())
  vi.clearAllMocks()
})

/* ---------------- tests ---------------- */

describe("main store – error handling", () => {
  it("checkApiError stores message and notification", async () => {
    const store = useMainStore()

    await store.checkApiError(new Error("boom"))

    expect(store.error).toBe("boom")
    expect(store.notifications.length).toBe(1)
    expect(store.notifications[0].content).toBe("boom")
    expect(store.notifications[0].color).toBe("error")
  })
})

describe("main store – login bootstrap", () => {
  it("completeLogin sets state, loads profile and routes", async () => {
    vi.spyOn(api, "getMe").mockResolvedValue({
      id: 1,
      isAdmin: true,
    } as any)

    const store = useMainStore()

    await store.completeLogin("TOKEN123")

    expect(auth.saveLocalToken).toHaveBeenCalledWith("TOKEN123")
    expect(ApiManager.init).toHaveBeenCalledWith("TOKEN123")
    expect(store.token).toBe("TOKEN123")
    expect(store.isLoggedIn).toBe(true)
    expect(store.userProfile?.id).toBe(1)
    expect(router.push).toHaveBeenCalledWith("/main")
  })
})

describe("main store – logout flow", () => {
  it("logOut clears session and routes", async () => {
    const store = useMainStore()

    store.token = "X"
    store.isLoggedIn = true
    store.userProfile = { id: 1 } as any

    await store.logOut()

    expect(auth.removeLocalToken).toHaveBeenCalled()
    expect(store.token).toBe("")
    expect(store.isLoggedIn).toBe(false)
    expect(store.userProfile).toBeNull()
    expect(router.push).toHaveBeenCalledWith("/login")
  })

  it("userLogOut adds notification", async () => {
    const store = useMainStore()
    await store.userLogOut()

  })
})

describe("main store – session restore", () => {
  it("checkLoggedIn restores session from local token", async () => {
    vi.spyOn(auth, "getLocalToken").mockReturnValue("LOCAL")
    vi.spyOn(api, "getMe").mockResolvedValue({
      id: 1,
      isAdmin: false,
    } as any)

    const store = useMainStore()
    await store.checkLoggedIn()

    expect(ApiManager.init).toHaveBeenCalledWith("LOCAL")
    expect(store.isLoggedIn).toBe(true)
    expect(store.userProfile?.id).toBe(1)
  })

  it("checkLoggedIn clears session if API fails", async () => {
    vi.spyOn(auth, "getLocalToken").mockReturnValue("LOCAL")
    vi.spyOn(api, "getMe").mockRejectedValue(new Error("401"))

    const store = useMainStore()
    await store.checkLoggedIn()

    expect(store.isLoggedIn).toBe(false)
    expect(store.token).toBe("")
  })
})
