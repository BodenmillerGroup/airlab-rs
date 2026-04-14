export const ConjugateStatus = {
  Stock: 0,
  Low: 1,
  Finished: 2,
} as const

export type ConjugateStatus = typeof ConjugateStatus[keyof typeof ConjugateStatus]

