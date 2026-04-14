// packages/frontend/src/modules/protein/types.ts
export interface ProteinView {
  id: number;
  groupId: number;
  createdBy: number;
  name: string;
  description?: string;
  meta?: Record<string, unknown>;
  createdAt?: string;
}
export interface ProteinDto {
  id: number;
  groupId: number;
  createdBy: number;
  name: string;
  description?: string;
  meta?: Record<string, unknown>;
  createdAt?: string;
}

export interface CreateProteinDto {
  createdBy?: number;
  groupId: number;
  name: string;
  description?: string;
}

export interface UpdateProteinDto {
  name: string;
  description?: string;
}
