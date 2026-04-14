export interface CollectionDto {
  readonly id: number;
  readonly name: string;
  readonly description: string | null;
  readonly created_at: string;
  readonly created_by: number;
}

export interface CreateCollectionDto {
  readonly name: string;
  readonly description?: string | null;
  readonly created_by: number;
}

export interface UpdateCollectionDto {
  readonly name?: string;
  readonly description?: string | null;
}
