import { useMainStore } from "@/stores/main";
import { useGroupStore } from "@/stores/group";
import { useSettingsStore } from "@/stores/settings";

// Define the shape of a message your app sends/receives
interface BroadcastMessage {
  method: string;
  payload: any;
}

export class BroadcastManager {
  private static channel: BroadcastChannel | null = null;

  static init() {
    BroadcastManager.close();

    const mainStore = useMainStore();
    const groupStore = useGroupStore();
    const settingsStore = useSettingsStore();

    BroadcastManager.channel = new BroadcastChannel("AirLab");

    BroadcastManager.channel.onmessage = (ev: MessageEvent<BroadcastMessage>) => {
      const { method, payload } = ev.data;

      if (settingsStore[method] && typeof settingsStore[method] === "function") {
        try {
          payload.suppressBroadcast = true;
          (settingsStore as any)[method](payload); // 👈 dynamic call
        } catch (err) {
          console.error(`[BroadcastManager] Failed to invoke method: ${method}`, err);
        }
      } else {
        console.warn(`[BroadcastManager] Unknown method: ${method}`);
      }
    };

    BroadcastManager.channel.onmessageerror = (ev) => {
      console.error("[BroadcastManager] Message error:", ev);
    };
  }

  static postMessage(message: BroadcastMessage) {
    if (BroadcastManager.channel) {
      BroadcastManager.channel.postMessage(message);
    }
  }

  static close() {
    if (BroadcastManager.channel) {
      BroadcastManager.channel.close();
      BroadcastManager.channel = null;
    }
  }
}

