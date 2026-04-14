import { defineStore } from 'pinia';

export const useResponsiveStore = defineStore('responsive', {
  state: () => ({
    width: 0,
    height: 0
  }),
  actions: {
    setResponsive({ width, height }: { width: number; height: number }) {
      this.width = width;
      this.height = height;
    }
  }
});
