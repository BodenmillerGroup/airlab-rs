import type { ValidationDto } from "@/modules/validation/types";
import type { ConjugateDto } from "@/modules/conjugate/types";
import { LotStatus } from "@/modules/lot/LotStatus";
import type { TagDto } from "@/modules/tag/types";
import { applicationIdToName } from "@/utils/enums";

export function applicationToString(value: number): string {
  return applicationIdToName[value];
}

export function stringToUTCString(value: string): string {
  return new Date(value).toUTCString();
}

const validationStatusMap = {
  0: "Yes",
  1: "So-So",
  2: "No",
  3: "Undefined",
};

export function validationStatusToString(value: number): string {
  return validationStatusMap[value];
}

const roleMap = {
  100: "Admin",
  10: "Standard",
  0: "Guest",
};

export function roleToString(value: number): string {
  if (value === -1) {
    return "";
  }
  return roleMap[value];
}

export function dilutionTypeToString(value: number): string {
  return value === 0 ? "µg/mL" : "1/__";
}

export function getStatusColor(validation: ValidationDto) {
  switch (validation.status) {
    case 0:
      return "green lighten-1";
    case 1:
      return "orange lighten-1";
    case 2:
      return "red lighten-1";
    default:
      return "grey";
  }
}

const conjugateStatusMap = {
  0: "Stock",
  1: "Low",
  2: "Finished",
};

export function conjugateStatusToString(value: number): string {
  return conjugateStatusMap[value];
}

export function getConjugateStatusColor(conjugate: ConjugateDto) {
  switch (conjugate.status) {
    case 0:
      return "green lighten-1";
    case 1:
      return "orange lighten-1";
    case 2:
      return "red lighten-1";
    default:
      return "grey";
  }
}

const lotStatusMap = {
  0: "Requested",
  1: "Approved",
  2: "Rejected",
  3: "Ordered",
  4: "Stock",
  5: "Low",
  6: "Finished",
};

export function lotStatusToString(value: number): string {
  return lotStatusMap[value];
}

export function getLotStatusColor(status: LotStatus) {
  switch (status) {
    case LotStatus.Stock:
      return "green lighten-1";
    case LotStatus.Low:
      return "orange lighten-1";
    case LotStatus.Finished:
      return "red lighten-1";
    default:
      return "grey";
  }
}

const tagStatusMap = {
  0: "Stock",
  1: "Low",
  2: "Finished",
};

export function tagStatusToString(value: number): string {
  return tagStatusMap[value];
}

export function getTagStatusColor(conjugate: TagDto) {
  switch (conjugate.status) {
    case 0:
      return "green lighten-1";
    case 1:
      return "orange lighten-1";
    case 2:
      return "red lighten-1";
    default:
      return "grey";
  }
}
