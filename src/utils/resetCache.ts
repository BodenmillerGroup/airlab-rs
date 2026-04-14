export async function resetCache(): Promise<void> {
  if ("caches" in self && self.caches) {
    const keyList = await self.caches.keys();
    await Promise.all(keyList.map((key) => self.caches.delete(key)));
  }
}
