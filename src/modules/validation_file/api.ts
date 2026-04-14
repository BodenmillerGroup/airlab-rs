import { ApiManager } from '@/utils/api'
import type { ValidationFileDto } from '@/modules/validation_file/types'

export const api = {
  async uploadValidationFile(validationId: number, formData: FormData) {
    return ApiManager.api
      .post(`validations/${validationId}/validation_files`, {
        body: formData,
        timeout: false,
      })
      .json<ValidationFileDto>()
  },
}
