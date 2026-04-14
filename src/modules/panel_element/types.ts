import type { ValidationDto } from '@/modules/validation/types'

export interface OldPanelElementView {
  id: number
  panelId: number
  conjugateId: number
  dilutionType: number
  concentration?: number
}

export interface PanelElementView {
  id: number
  conjugateId: number

  tubeNumber: number
  status: number

  tagId: number
  tagName: string
  tagMw?: number | null

  lotId: number
  lotNumber: string

  cloneId: number
  cloneName: string

  proteinId: number
  proteinName: string

  validations: ValidationDto[]

  concentration?: number | null
  actualConcentration: number
  dilutionType: number
  pipet: number
  finishedBy: number
}

export interface PanelElementDto {
  id: number
  panelId: number
  conjugateId: number
  dilutionType: number
  concentration?: number
}
