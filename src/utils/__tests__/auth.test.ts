import { describe, it, expect, beforeEach } from "vitest";
import { getLocalToken, saveLocalToken, removeLocalToken } from "@/utils/auth";

describe("auth utils", () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it("saves and reads token from localStorage", () => {
    saveLocalToken("ABC123");

    expect(localStorage.getItem("token")).toBe("ABC123");
    expect(getLocalToken()).toBe("ABC123");
  });

  it("removes token from localStorage", () => {
    saveLocalToken("ABC123");
    removeLocalToken();

    expect(localStorage.getItem("token")).toBeNull();
    expect(getLocalToken()).toBeNull();
  });
});
