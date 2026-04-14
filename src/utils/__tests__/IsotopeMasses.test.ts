import { describe, it, expect } from "vitest";
import { getMassForIsotope } from "@/utils/IsotopeMasses";

describe("IsotopeMasses", () => {
  it("returns correct mass for known isotope", () => {
    expect(getMassForIsotope("Ir-191")).toBe("190.960");
    expect(getMassForIsotope("Tb-159")).toBe("158.925");
    expect(getMassForIsotope("Ho-165")).toBe("164.930");
  });

  it("matches partial isotope keys (current behavior)", () => {
    // function uses: if (i.indexOf(isotope) > -1)
    expect(getMassForIsotope("Ir-19")).toBe("190.960");
    expect(getMassForIsotope("Tb")).toBe("158.925");
  });

  it("returns 'undefined' for unknown isotope", () => {
    expect(getMassForIsotope("Xx-999")).toBe("undefined");
  });

  it("never returns undefined JS value (always string)", () => {
    const result = getMassForIsotope("Ir-191");
    expect(typeof result).toBe("string");
  });
});
