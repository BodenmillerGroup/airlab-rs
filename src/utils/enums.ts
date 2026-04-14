export const applicationEnum = [
  { value: 0, text: "SMC" },
  { value: 1, text: "IMC" },
  { value: 2, text: "FC" },
  { value: 3, text: "IF" },
  { value: 4, text: "IHC" },
  { value: 5, text: "IHC-F" },
  { value: 6, text: "WB" },
];

export const applicationNameToId = {
  sMC: 0,
  iMC: 1,
  FC: 2,
  IF: 3,
  IHC: 4,
  IHCF: 5,
  WB: 6,
};

export const applicationIdToName = {
  0: "SMC",
  1: "IMC",
  2: "FC",
  3: "IF",
  4: "IHC",
  5: "IHC-F",
  6: "WB",
};

export const statusEnum = [
  { value: 0, text: "Yes" },
  { value: 1, text: "So-So" },
  { value: 2, text: "No" },
  { value: 3, text: "Undefined" },
];

export const roleEnum = [
  { value: 100, text: "Admin" },
  { value: 10, text: "Standard" },
  { value: 0, text: "Guest" },
];

export const tagTypeEnum = [
  { value: 0, text: "Metal" },
  { value: 1, text: "Fluorophore" },
];

export const tagStatusEnum = [
  { value: 0, text: "Stock" },
  { value: 1, text: "Low" },
  { value: 2, text: "Finished" },
];

export const conjugateStatusEnum = [
  { value: 0, text: "Stock" },
  { value: 1, text: "Low" },
  { value: 2, text: "Finished" },
];

export const lotStatusEnum = [
  { value: 0, text: "Requested" },
  { value: 1, text: "Approved" },
  { value: 2, text: "Rejected" },
  { value: 3, text: "Ordered" },
  { value: 4, text: "Stock" },
  { value: 5, text: "Low" },
  { value: 6, text: "Finished" },
];

export const antigenRetrievalTypes = ["HIER Buffer", "Sodium Citrate", "2-step Retrieval", "RNAScope"];
