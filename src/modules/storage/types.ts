export interface StorageDto {
  readonly id: number;
  readonly name: string;
  readonly type: string;
  readonly location: string;
  readonly temperature_c: number;
  readonly active: boolean;
  readonly created_at: string;
  readonly updated_at: string;
}

export interface CreateStorageDto {
  readonly name: string;
  readonly type: string;
  readonly location: string;
  readonly temperature_c: number;
  readonly active: boolean;
}

export interface UpdateStorageDto {
  readonly name?: string;
  readonly type?: string;
  readonly location?: string;
  readonly temperature_c?: number;
  readonly active?: boolean;
}
