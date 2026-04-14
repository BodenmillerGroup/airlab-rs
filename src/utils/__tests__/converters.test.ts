import { describe, it, expect } from "vitest";

import {
  applicationToString,
  stringToUTCString,
  validationStatusToString,
  roleToString,
  dilutionTypeToString,
  getStatusColor,
  conjugateStatusToString,
  getConjugateStatusColor,
  lotStatusToString,
  getLotStatusColor,
  tagStatusToString,
  getTagStatusColor,
} from "@/utils/converters";

import { LotStatus } from "@/modules/lot/LotStatus";

describe("utils/converters", () => {
  it("applicationToString maps application ids", () => {
    expect(applicationToString(0)).toBeTypeOf("string");
  });

  it("stringToUTCString converts to UTC string", () => {
    const out = stringToUTCString("2024-01-01T00:00:00Z");
    expect(out).toContain("GMT");
  });

  it("validationStatusToString maps validation status", () => {
    expect(validationStatusToString(0)).toBe("Yes");
    expect(validationStatusToString(1)).toBe("So-So");
    expect(validationStatusToString(2)).toBe("No");
    expect(validationStatusToString(3)).toBe("Undefined");
  });

  it("roleToString maps roles correctly", () => {
    expect(roleToString(100)).toBe("Admin");
    expect(roleToString(10)).toBe("Standard");
    expect(roleToString(0)).toBe("Guest");
    expect(roleToString(-1)).toBe("");
  });

  it("dilutionTypeToString formats dilution types", () => {
    expect(dilutionTypeToString(0)).toBe("µg/mL");
    expect(dilutionTypeToString(1)).toBe("1/__");
  });

  it("getStatusColor returns correct validation colors", () => {
    expect(getStatusColor({ status: 0 } as any)).toBe("green lighten-1");
    expect(getStatusColor({ status: 1 } as any)).toBe("orange lighten-1");
    expect(getStatusColor({ status: 2 } as any)).toBe("red lighten-1");
    expect(getStatusColor({ status: 99 } as any)).toBe("grey");
  });

  it("conjugateStatusToString maps conjugate status", () => {
    expect(conjugateStatusToString(0)).toBe("Stock");
    expect(conjugateStatusToString(1)).toBe("Low");
    expect(conjugateStatusToString(2)).toBe("Finished");
  });

  it("getConjugateStatusColor maps colors", () => {
    expect(getConjugateStatusColor({ status: 0 } as any)).toBe("green lighten-1");
    expect(getConjugateStatusColor({ status: 1 } as any)).toBe("orange lighten-1");
    expect(getConjugateStatusColor({ status: 2 } as any)).toBe("red lighten-1");
    expect(getConjugateStatusColor({ status: 99 } as any)).toBe("grey");
  });

  it("lotStatusToString maps lot status", () => {
    expect(lotStatusToString(0)).toBe("Requested");
    expect(lotStatusToString(4)).toBe("Stock");
    expect(lotStatusToString(6)).toBe("Finished");
  });

  it("getLotStatusColor maps lot colors", () => {
    expect(getLotStatusColor(LotStatus.Stock)).toBe("green lighten-1");
    expect(getLotStatusColor(LotStatus.Low)).toBe("orange lighten-1");
    expect(getLotStatusColor(LotStatus.Finished)).toBe("red lighten-1");
    expect(getLotStatusColor(999 as any)).toBe("grey");
  });

  it("tagStatusToString maps tag status", () => {
    expect(tagStatusToString(0)).toBe("Stock");
    expect(tagStatusToString(1)).toBe("Low");
    expect(tagStatusToString(2)).toBe("Finished");
  });

  it("getTagStatusColor maps tag colors", () => {
    expect(getTagStatusColor({ status: 0 } as any)).toBe("green lighten-1");
    expect(getTagStatusColor({ status: 1 } as any)).toBe("orange lighten-1");
    expect(getTagStatusColor({ status: 2 } as any)).toBe("red lighten-1");
    expect(getTagStatusColor({ status: 99 } as any)).toBe("grey");
  });
});
