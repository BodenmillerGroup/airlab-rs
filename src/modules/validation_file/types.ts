export interface ValidationFileView {
  id: number
  validationId: number
  createdBy: number
  hash: string
  size?: number
  name?: string
  extension: string
  description?: string
}
export interface ValidationFileDto {
  id: number
  validationId: number
  createdBy: number
  hash: string
  size?: number
  name?: string
  extension: string
  description?: string
}
