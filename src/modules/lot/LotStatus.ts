export const LotStatus = {
  Requested: 0,
  Approved: 1,
  Rejected: 2,
  Ordered: 3,
  Stock: 4,
  Low: 5,
  Finished: 6,
} as const

export type LotStatus = typeof LotStatus[keyof typeof LotStatus]

