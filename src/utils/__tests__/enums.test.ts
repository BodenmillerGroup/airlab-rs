import { describe, it, expect } from "vitest";
import {
  applicationEnum,
  applicationNameToId,
  applicationIdToName,
  statusEnum,
  roleEnum,
  tagTypeEnum,
  tagStatusEnum,
  antigenRetrievalTypes,
} from "@/utils/enums";

describe("enums", () => {
  it("application enums are consistent (id ↔ name)", () => {
    for (const { value, text } of applicationEnum) {
      expect(applicationIdToName[value]).toBe(text);

      // normalize like your map does (IHC-F → IHCF, SMC → sMC etc if needed)
      const key = text.replace("-", "");
      const found = Object.entries(applicationNameToId).find(
        ([, v]) => v === value
      );
      expect(found).toBeTruthy();
    }
  });

  it("application ids are unique", () => {
    const values = applicationEnum.map(e => e.value);
    expect(new Set(values).size).toBe(values.length);
  });

  it("status enum is stable", () => {
    expect(statusEnum).toEqual([
      { value: 0, text: "Yes" },
      { value: 1, text: "So-So" },
      { value: 2, text: "No" },
      { value: 3, text: "Undefined" },
    ]);
  });

  it("role enum is stable", () => {
    expect(roleEnum).toEqual([
      { value: 100, text: "Admin" },
      { value: 10, text: "Standard" },
      { value: 0, text: "Guest" },
    ]);
  });

  it("tag enums are well-formed", () => {
    expect(tagTypeEnum.map(x => x.text)).toEqual(["Metal", "Fluorophore"]);
    expect(tagStatusEnum.map(x => x.text)).toEqual(["Stock", "Low", "Finished"]);
  });

  it("antigen retrieval types are defined", () => {
    expect(antigenRetrievalTypes.length).toBeGreaterThan(0);
    expect(antigenRetrievalTypes).toContain("HIER Buffer");
    expect(antigenRetrievalTypes).toContain("RNAScope");
  });
});
