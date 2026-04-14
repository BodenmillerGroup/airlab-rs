import { saveAs } from "file-saver";
import { getMassForIsotope } from "@/utils/IsotopeMasses";
import type { PanelDto } from "@/modules/panel/types";
import { TemplateCyTOF1, TemplateCyTOF2, TemplateHelios } from "@/utils/templates";

type ExportItem = {
  id?: number
  tubeNumber?: number
  tag?: { name?: string; mw?: number | string | null }
  tagName?: string
  tagMw?: number | string | null
  lot?: { clone?: { id?: number; name?: string; isPhospho?: boolean; protein?: { name?: string } } }
  cloneId?: number
  cloneName?: string
  proteinName?: string
  concentration?: number | null
  actualConcentration?: number | null
  dilutionType?: number | null
  pipet?: number | null
}

function getPipetVolume(item: ExportItem, totalVolume: number): number {
  const actualConcentration = Number(item.actualConcentration ?? 0)
  const stockConcentration = Number(item.concentration ?? 0)

  if (item.dilutionType === 1) {
    return actualConcentration === 0 ? 0 : totalVolume / actualConcentration
  }

  if (stockConcentration === 0) {
    return 0
  }

  return totalVolume * (actualConcentration / stockConcentration)
}

function getTagName(item: ExportItem) {
  return item.tag?.name ?? item.tagName ?? "UnknownTag"
}

function getTagMw(item: ExportItem) {
  return item.tag?.mw ?? item.tagMw ?? ""
}

function getProteinName(item: ExportItem) {
  return item.lot?.clone?.protein?.name ?? item.proteinName
}

function getCloneName(item: ExportItem) {
  return item.lot?.clone?.name ?? item.cloneName
}

function getCloneId(item: ExportItem) {
  return item.lot?.clone?.id ?? item.cloneId ?? 0
}

function isClonePhospho(item: ExportItem) {
  return item.lot?.clone?.isPhospho ?? false
}

export function exportCsv(text: string, filename: string) {
  const blob = new Blob([text], { type: "text/csv" });
  saveAs(blob, filename);
}

export function exportXml(text: string, filename: string) {
  const blob = new Blob([text], { type: "text/xml" });
  saveAs(blob, filename);
}

function addComponentsPanelToTemplate(
  panel: PanelDto,
  txtTemplate: string,
  items: ExportItem[],
  templateName: string,
  version: number
) {
  const comps = txtTemplate.split("-----+++");
  if (comps.length == 3 && version < 3) {
    let firstPart = comps[0];

    while (firstPart.indexOf("{panelName}") > -1) {
      firstPart = firstPart.replace("{panelName}", panel.name);
    }

    if (items) {
      const used: any[] = [];
      const secondPart = comps[1];
      for (const item of items) {
        let aSecond = secondPart.replace("{panelName}", panel.name);
        const tagName = getTagName(item);
        const tagMw = getTagMw(item);
        const keyForMass = tagName + "-" + tagMw;
        let found = false;
        for (const u in used) {
          if (parseInt(String(tagMw)) == parseInt(used[u])) {
            found = true;
          }
        }
        if (found) continue;
        else used.push(tagMw);

        const mass = getMassForIsotope(keyForMass);

        aSecond = aSecond.replace("{isotopeMass}", mass);

        const proteinName = getProteinName(item);
        if (proteinName) {
          let proName = proteinName;
          proName = proName.replace(" ", "").replace("_", "").replace("/", "").replace("\\", "");
          let descrp = proName.substring(0, proName.length > 7 ? 7 : proName.length);
          if (isClonePhospho(item)) descrp += "_phospho";
          descrp += "_";
          descrp += getCloneId(item);
          descrp += "((" + item.id + "))";
          descrp += tagName + tagMw;
          aSecond = aSecond.replace("{protName}", descrp);
          aSecond = aSecond.replace("{protDescription}", descrp);
        } else {
          aSecond = aSecond.replace("{protName}", "UnknownProtein");
          aSecond = aSecond.replace("{protDescription}", "UnknownClone");
        }

        aSecond = aSecond.replace("{atom}", tagName);
        aSecond = aSecond.replace("{atom}", tagName);
        aSecond = aSecond.replace("{atomShortMass}", String(tagMw));
        aSecond = aSecond.replace("{orderNumber}", (3000 + parseInt(String(item.tubeNumber))).toString(10));
        firstPart += aSecond;
      }
    }
    firstPart += comps[2];
    exportXml(firstPart, panel.name + "_" + templateName + ".conf");
  } else if (comps.length == 3 && version > 2) {
    let firstPart = comps[0];

    if (items) {
      const used: any[] = [];
      const secondPart = comps[1];
      for (const item of items) {
        let aSecond = secondPart.replace("{panelName}", panel.name);
        const tagName = getTagName(item);
        const tagMw = getTagMw(item);
        const keyForMass = tagName + "-" + tagMw;
        let found = false;
        for (const u in used) {
          if (parseInt(String(tagMw)) == parseInt(used[u])) {
            found = true;
          }
        }
        if (found) continue;
        else used.push(tagMw);
        const mass = getMassForIsotope(keyForMass);
        aSecond = aSecond.replace("{isotopeMass}", mass);
        const proteinName = getProteinName(item);
        if (proteinName) {
          let proName = proteinName;
          proName = proName.replace(" ", "").replace("_", "").replace("/", "").replace("\\", "");
          let descrp = proName.substring(0, proName.length > 7 ? 7 : proName.length);
          if (isClonePhospho(item)) descrp += "_phospho";
          descrp += "_";
          descrp += getCloneId(item);
          descrp += "((" + item.id + "))";
          descrp += tagName + tagMw;
          aSecond = aSecond.replace("{protName}", descrp);
          aSecond = aSecond.replace("{protDescription}", descrp);
        } else {
          aSecond = aSecond.replace("{protName}", "UnknownProtein");
          aSecond = aSecond.replace("{protDescription}", "UnknownClone");
        }
        aSecond = aSecond.replace("{atom}", tagName);
        aSecond = aSecond.replace("{atom}", tagName);
        aSecond = aSecond.replace("{atom}", tagName);
        aSecond = aSecond.replace("{atomShortMass}", String(tagMw));
        aSecond = aSecond.replace("{atomShortMass}", String(tagMw));
        aSecond = aSecond.replace("{atomShortMass}", String(tagMw));
        aSecond = aSecond.replace("{orderNumber}", (3000 + parseInt(String(item.tubeNumber))).toString(10));
        firstPart += aSecond;
      }
    }
    let thirdPart = comps[2];
    while (thirdPart.indexOf("{panelName}") > -1) {
      thirdPart = thirdPart.replace("{panelName}", panel.name);
    }
    firstPart += thirdPart;
    exportXml(firstPart, panel.name + "_" + templateName + ".tem");
  } else {
    alert("Something went wrong and a template can't be generated");
  }
}

export function exportPanelCsv(panel: PanelDto, items: ExportItem[], totalVolume: number) {
  let txt = `Total Volume: ${totalVolume}uL,,,,,,\rTube Number,Metal Tag,Target,Antibody Clone,Stock Concentration,Final Concentration / Dilution,uL to add\r`;
  for (const item of items) {
    txt += item.tubeNumber + ",";
    txt += getTagName(item) + getTagMw(item) + ",";
    const proteinName = getProteinName(item) ?? "UnknownProtein";
    const cloneName = getCloneName(item) ?? "UnknownClone";
    txt += proteinName.replace(/,/g, "-") + ",";
    txt += cloneName.replace(/,/g, "-") + ",";
    txt += item.concentration + ",";
    if (item.dilutionType === 1) {
      txt += "1/" + item.actualConcentration + ",";
    } else {
      txt += item.actualConcentration + " ug/mL,";
    }
    txt += getPipetVolume(item, totalVolume) + "\n";
  }
  exportCsv(txt, panel.name + ".csv");
}

export function exportCSVCyTOF201608(panel: PanelDto, items: ExportItem[]) {
  let txt = "AnalyteName,Label,Target\r";
  for (const item of items) {
    const tagName = getTagName(item);
    const tagMw = getTagMw(item);
    txt += tagName + "(" + tagMw + "),";
    txt += tagMw + tagName + ",";
    const proteinName = getProteinName(item);
    if (proteinName) {
      let proName = proteinName;
      proName = proName.replace(" ", "").replace("_", "").replace("/", "").replace("\\", "");
      let descrp = proName.substring(0, proName.length > 7 ? 7 : proName.length);
      if (isClonePhospho(item)) descrp += "_phospho";
      descrp += "_";
      descrp += getCloneId(item);
      descrp += "((" + item.id + "))";
      descrp += tagName + tagMw;
      txt += descrp;
    } else txt += "UnknownProtein";
    txt += "\n";
  }
  exportCsv(txt, panel.name + ".csv");
}

export function exportCyTOF1Panel(panel: PanelDto, items) {
  addComponentsPanelToTemplate(panel, TemplateCyTOF1, items, "TemplateCyTOF1", 1);
}

export function exportCyTOF2Panel(panel: PanelDto, items) {
  addComponentsPanelToTemplate(panel, TemplateCyTOF2, items, "TemplateCyTOF2", 2);
}

export function exportHeliosPanel(panel: PanelDto, items) {
  addComponentsPanelToTemplate(panel, TemplateHelios, items, "TemplateHelios", 3);
}
