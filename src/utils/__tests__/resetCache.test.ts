import { describe, it, expect, vi, beforeEach } from "vitest";
import { resetCache } from "@/utils/resetCache";

describe("resetCache", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("deletes all caches when Cache API is available", async () => {
    const deleteSpy = vi.fn();
    const keysSpy = vi.fn().mockResolvedValue(["a", "b", "c"]);

    vi.stubGlobal("caches", {
      keys: keysSpy,
      delete: deleteSpy,
    });

    await resetCache();

    expect(keysSpy).toHaveBeenCalledOnce();
    expect(deleteSpy).toHaveBeenCalledTimes(3);
    expect(deleteSpy).toHaveBeenCalledWith("a");
    expect(deleteSpy).toHaveBeenCalledWith("b");
    expect(deleteSpy).toHaveBeenCalledWith("c");
  });

  it("does nothing if Cache API is not available", async () => {
    // remove caches from global scope
    vi.stubGlobal("caches", undefined);

    await expect(resetCache()).resolves.not.toThrow();
  });
});
