import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  exportCsv,
  exportXml,
  exportPanelCsv,
  exportCSVCyTOF201608,
  exportCyTOF1Panel,
  exportCyTOF2Panel,
  exportHeliosPanel,
} from "@/utils/exporters";

/* ------------------ mocks ------------------ */

// exporters.test.ts

class MockBlob {
  parts: any[];
  type?: string;

  constructor(parts: any[], opts?: { type?: string }) {
    this.parts = parts;
    this.type = opts?.type;
  }

  async text() {
    return this.parts.join("");
  }
}

(globalThis as any).Blob = MockBlob;

const saveAsSpy = vi.fn();
vi.mock("file-saver", () => ({
  saveAs: (...args: any[]) => saveAsSpy(...args),
}));

vi.mock("@/utils/IsotopeMasses", () => ({
  getMassForIsotope: (k: string) => "159",
}));

vi.mock("@/utils/templates", () => ({
  TemplateCyTOF1: "A{panelName}-----+++B{isotopeMass}{protName}{atom}{atomShortMass}{orderNumber}-----+++C",
  TemplateCyTOF2: "A-----+++B{isotopeMass}{protName}{atom}{atomShortMass}{orderNumber}-----+++C",
  TemplateHelios: "A-----+++B{isotopeMass}{protName}{atom}{atomShortMass}{orderNumber}-----+++C",
}));

/* ------------------ fixtures ------------------ */

const panel: any = { name: "TestPanel" };

const items: any[] = [
  {
    id: 1,
    tubeNumber: "5",
    concentration: 10,
    actualConcentration: 2,
    dilutionType: 1,
    pipet: 3,
    tag: { name: "Ir", mw: "191" },
    lot: {
      clone: {
        id: 42,
        name: "CloneA",
        isPhospho: true,
        protein: { name: "STAT3" },
      },
    },
  },
];

/* ------------------ tests ------------------ */

describe("exporters", () => {
  beforeEach(() => {
    saveAsSpy.mockClear();
  });

  it("exports CSV", () => {
    exportCsv("hello", "a.csv");

    expect(saveAsSpy).toHaveBeenCalledOnce();
    const [blob, name] = saveAsSpy.mock.calls[0];

    expect(name).toBe("a.csv");
    expect(blob).toBeInstanceOf(Blob);
  });

  it("exports XML", () => {
    exportXml("<a/>", "a.xml");

    const [blob, name] = saveAsSpy.mock.calls[0];
    expect(name).toBe("a.xml");
    expect(blob.type).toBe("text/xml");
  });

  it("exports panel CSV", () => {
    exportPanelCsv(panel, items, 100);

    const [blob, name] = saveAsSpy.mock.calls[0];
    expect(name).toBe("TestPanel.csv");

    return blob.text().then(text => {
      expect(text).toContain("Total Volume: 100uL");
      expect(text).toContain("STAT3");
      expect(text).toContain("Ir191");
      expect(text).toContain("1/2,50");
    });
  });

  it("exports CyTOF CSV", () => {
    exportCSVCyTOF201608(panel, items);

    const [blob] = saveAsSpy.mock.calls[0];

    return blob.text().then(text => {
      expect(text).toContain("AnalyteName");
      expect(text).toContain("Ir(191)");
      expect(text).toContain("STAT3");
    });
  });

  it("exports CyTOF1 panel", () => {
    exportCyTOF1Panel(panel, items);

    const [blob, name] = saveAsSpy.mock.calls[0];
    expect(name).toBe("TestPanel_TemplateCyTOF1.conf");

    return blob.text().then(text => {
      expect(text).toContain("TestPanel");
      expect(text).toContain("159");
      expect(text).toContain("STAT3");
    });
  });

  it("exports CyTOF2 panel", () => {
    exportCyTOF2Panel(panel, items);
    expect(saveAsSpy.mock.calls[0][1]).toBe("TestPanel_TemplateCyTOF2.conf");
  });

  it("exports Helios panel", () => {
    exportHeliosPanel(panel, items);
    expect(saveAsSpy.mock.calls[0][1]).toBe("TestPanel_TemplateHelios.tem");
  });
});
