export const TagStatus = {
  Stock: 0,
  Low: 1,
  Finished: 2,
} as const

export type TagStatus = typeof TagStatus[keyof typeof TagStatus]
