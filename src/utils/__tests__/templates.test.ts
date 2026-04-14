import { describe, it, expect } from "vitest";
import {
  TemplateCyTOF1,
  TemplateCyTOF2,
  TemplateHelios,
} from "@/utils/templates";

function expectPlaceholders(template: string, placeholders: string[]) {
  for (const p of placeholders) {
    expect(template.includes(p)).toBe(true);
  }
}

describe("CyTOF templates", () => {
  it("TemplateCyTOF1 has required structure and placeholders", () => {
    expect(TemplateCyTOF1).toContain("-----+++");
    expect(TemplateCyTOF1).toContain("<SampleAcquisitionTemplate");
    expect(TemplateCyTOF1).toContain("</SampleAcquisitionTemplate>");

    expectPlaceholders(TemplateCyTOF1, [
      "{panelName}",
      "{protName}",
      "{protDescription}",
      "{isotopeMass}",
      "{atom}",
      "{atomShortMass}",
      "{orderNumber}",
    ]);
  });

  it("TemplateCyTOF2 has required structure and placeholders", () => {
    expect(TemplateCyTOF2).toContain("-----+++");
    expect(TemplateCyTOF2).toContain("<SampleAcquisitionTemplate");
    expect(TemplateCyTOF2).toContain("</SampleAcquisitionTemplate>");

    expectPlaceholders(TemplateCyTOF2, [
      "{panelName}",
      "{protName}",
      "{protDescription}",
      "{isotopeMass}",
      "{atom}",
      "{atomShortMass}",
      "{orderNumber}",
    ]);
  });

  it("TemplateHelios has required structure and placeholders", () => {
    expect(TemplateHelios).toContain("-----+++");
    expect(TemplateHelios).toContain("<MethodSchema");
    expect(TemplateHelios).toContain("</MethodSchema>");

    expectPlaceholders(TemplateHelios, [
      "{panelName}",
      "{protDescription}",
      "{atom}",
      "{atomShortMass}",
    ]);
  });

  it("templates are non-empty and reasonably large", () => {
    expect(TemplateCyTOF1.length).toBeGreaterThan(200);
    expect(TemplateCyTOF2.length).toBeGreaterThan(200);
    expect(TemplateHelios.length).toBeGreaterThan(200);
  });
});
