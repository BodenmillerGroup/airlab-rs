import { ApiManager } from "@/utils/api";

export type Operation = "Get" | "Insert" | "Update" | "Delete" | "Reorder" | "Duplicate" | "UploadFile";

export type ReturnType = "User" | "Member" | "Clone" | "Group" | "Species" | "Protein" | "Validation" | "Conjugate" | "Lot" | "Panel" | "Tag" | "Provider" | "PanelElement" | "ValidationFile" | "Storage" | "Collection";

export type Operator = "=" | ">" | "<" | "like" | "in" | "eq" | "contains";

export interface Filter {
  field: string;
  op: Operator;
  value: any;
}

export interface RpcRequest {
  operation: Operation;
  return_type: ReturnType;
  id?: number;
  filters?: Filter[];
  payload?: any;
  limit?: number;
  page?: number;
}

export async function rpc<T>(req: RpcRequest): Promise<T> {
  const response = await ApiManager.api
    .post("json", { json: req })
    .json<T>();

  return response;
}

export interface SearchFilter {
  table: ReturnType;
  field: string;
  op: Operator;
  value: any;
}

export type SearchFilterInput = readonly [ReturnType, string, Operator, unknown];

export type SortDirection = "asc" | "desc";

export interface SearchOrder {
  table: ReturnType;
  field: string;
  direction: SortDirection;
}

export interface SearchRequest {
  return_type: ReturnType;
  filters: SearchFilter[];
  limit?: number;
  page?: number;
  order?: SearchOrder;
  global_filter?: string;
  show_all?: boolean;
}

export interface SearchResponse {
  items: number[];
  search_total: number;
}

export async function rpcSearch(req: SearchRequest): Promise<SearchResponse> {
  const response = await ApiManager.api
    .post("search", { json: req })
    .json<SearchResponse>();

  return response;
}

export function createFilters(
  ...filters: SearchFilterInput[]
): SearchFilter[] {
  return filters
    .filter(([, , , value]) => value !== null && value !== undefined)
    .map(([table, field, op, value]) => ({
      table,
      field,
      op,
      value
    }));
}

export function sf(
  table: ReturnType,
  field: string,
  op: Operator,
  value: unknown
): SearchFilterInput {
  return [table, field, op, value];
}

export function createOrder(
  table: ReturnType,
  field: string,
  direction: 'asc' | 'desc' = 'desc'
): SearchOrder {
  return {
    table,
    field,
    direction,
  };
}

export type FilterType = "select" | "text" | "checkbox" | "number"; // add more as needed

export interface FilterConfig {
  key: string; // used in v-model
  label: string;
  table: ReturnType;
  field: string;
  op: Operator;
  type: FilterType;
  multiple?: boolean;
  clearable?: boolean;
  options?: { label: string; value: any }[]; // for selects
}
