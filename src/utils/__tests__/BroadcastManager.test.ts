import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { BroadcastManager } from "@/utils/BroadcastManager";

// --- mocks ---
let lastChannel: any;

class MockBroadcastChannel {
  name: string;
  onmessage: ((ev: any) => void) | null = null;
  onmessageerror: ((ev: any) => void) | null = null;

  postMessage = vi.fn();
  close = vi.fn();

  constructor(name: string) {
    this.name = name;
    lastChannel = this;
  }
}

vi.stubGlobal("BroadcastChannel", MockBroadcastChannel as any);

// mock stores
const settingsSpy = vi.fn();

vi.mock("@/stores/settings", () => ({
  useSettingsStore: () => ({
    resetSettings: settingsSpy,
  }),
}));

vi.mock("@/stores/main", () => ({
  useMainStore: () => ({}),
}));

vi.mock("@/stores/group", () => ({
  useGroupStore: () => ({}),
}));

// --- tests ---
describe("BroadcastManager", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    settingsSpy.mockClear();
    lastChannel = null;
  });

  afterEach(() => {
    BroadcastManager.close();
  });

  it("initializes a BroadcastChannel", () => {
    BroadcastManager.init();

    expect(lastChannel).toBeTruthy();
    expect(lastChannel.name).toBe("AirLab");
  });

  it("dispatches message to settings store method", () => {
    BroadcastManager.init();

    lastChannel.onmessage({
      data: {
        method: "resetSettings",
        payload: { foo: "bar" },
      },
    });

    expect(settingsSpy).toHaveBeenCalledOnce();
    expect(settingsSpy).toHaveBeenCalledWith(
      expect.objectContaining({
        foo: "bar",
        suppressBroadcast: true,
      })
    );
  });

  it("ignores unknown methods safely", () => {
    BroadcastManager.init();

    expect(() => {
      lastChannel.onmessage({
        data: { method: "doesNotExist", payload: {} },
      });
    }).not.toThrow();
  });

  it("postMessage forwards to BroadcastChannel", () => {
    BroadcastManager.init();

    BroadcastManager.postMessage({ method: "resetSettings", payload: {} });

    expect(lastChannel.postMessage).toHaveBeenCalledOnce();
  });

  it("close shuts down the channel", () => {
    BroadcastManager.init();
    const channel = lastChannel;

    BroadcastManager.close();

    expect(channel.close).toHaveBeenCalledOnce();
  });
});
