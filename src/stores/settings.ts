import { defineStore } from "pinia";

export const useSettingsStore = defineStore("settings", () => {
  // State (empty now, but scalable later)
  // You could define something like theme, locale, etc. here if needed

  // Actions
  async function resetSettings() {
    if ("caches" in self) {
      const keyList = await caches.keys();
      await Promise.all(keyList.map((key) => caches.delete(key)));
    }
  }

  return {
    resetSettings,
  };
});

