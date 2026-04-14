
import { describe, it, expect, beforeEach, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useConjugateStore } from "@/stores/conjugate";
import { useMainStore } from "@/stores/main";
import { useLotStore } from "@/stores/lot";
import { useCloneStore } from "@/stores/clone";
import { useProteinStore } from "@/stores/protein";
import { useUserStore } from "@/stores/user";
import { useMemberStore } from "@/stores/member";
import { useTagStore } from "@/stores/tag";
import { useValidationStore } from "@/stores/validation";
import { useStorageStore } from "@/stores/storage";

vi.mock("@/modules/json/api", () => ({
  rpc: vi.fn(),
  rpcSearch: vi.fn(),
}));

import * as jsonApi from "@/modules/json/api";

describe("useConjugateStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  function mockMainStore() {
    const main = useMainStore();
    vi.spyOn(main, "addNotification").mockImplementation(() => { });
    vi.spyOn(main, "checkApiError").mockResolvedValue(undefined as any)
    return main;
  }

  it("adds and normalizes conjugates via setEntity", () => {
    const store = useConjugateStore();

    store.setEntity({ id: 1, tagId: 10, tubeNumber: 5 } as any);
    store.setEntity({ id: 2, tagId: 10, tubeNumber: 6 } as any);

    expect(store.ids).toEqual([1, 2]);
    expect(store.entities[1].tubeNumber).toBe(5);
  });

  it("computes getters correctly", () => {
    const store = useConjugateStore();

    store.setEntity({ id: 1, tagId: 99, tubeNumber: 3 } as any);
    store.setEntity({ id: 2, tagId: 99, tubeNumber: 7 } as any);
    store.setEntity({ id: 3, tagId: 5, tubeNumber: 9 } as any);

    expect(store.hasConjugate(1)).toBe(true);
    expect(store.getConjugatesForTag(99)).toHaveLength(2);
    expect(store.tubeNumberMap[2]).toBe(7);
  });

  it("creates conjugate via RPC and stores it", async () => {
    mockMainStore();

    (jsonApi.rpc as any).mockResolvedValueOnce({
      id: 10,
      tagId: 3,
      tubeNumber: 12,
    });
    (jsonApi.rpc as any).mockResolvedValueOnce({
      items: [{
        id: 10,
        tagId: 3,
        tubeNumber: 12,
      }],
      total: 1,
    });

    const store = useConjugateStore();

    await store.createConjugate({} as any);

    expect(jsonApi.rpc).toHaveBeenCalledTimes(2);
    expect(store.ids).toContain(10);
    expect(store.entities[10].tubeNumber).toBe(12);
  });

  it("deletes conjugate correctly", async () => {
    mockMainStore();

    (jsonApi.rpc as any).mockResolvedValueOnce(undefined);

    const store = useConjugateStore();
    store.setEntity({ id: 5, tagId: 1, tubeNumber: 2 } as any);

    await store.deleteConjugate(5);

    expect(jsonApi.rpc).toHaveBeenCalledOnce();
    expect(store.ids).not.toContain(5);
    expect(store.entities[5]).toBeUndefined();
  });

  it("fetchByIds only fetches missing conjugates", async () => {
    mockMainStore();

    (jsonApi.rpc as any).mockResolvedValueOnce({
      items: [
        { id: 2, tagId: 10, tubeNumber: 8 },
        { id: 3, tagId: 10, tubeNumber: 9 },
      ],
      total: 2,
    });

    const store = useConjugateStore();

    store.setEntity({ id: 1, tagId: 10, tubeNumber: 5 } as any);

    const res = await store.fetchByIds(1, [1, 2, 3]);

    expect(jsonApi.rpc).toHaveBeenCalledOnce();
    expect(store.ids.sort()).toEqual([1, 2, 3]);
    expect(res).toHaveLength(2);
  });

  it("loadListQuery stores visible ids and ensures related data in dependent stores", async () => {
    vi.spyOn(jsonApi, "rpcSearch").mockResolvedValue({
      items: [1, 2],
      search_total: 2,
    } as any);

    vi.spyOn(jsonApi, "rpc").mockResolvedValue({
      items: [
        { id: 1, groupId: 7, tagId: 101, lotId: 201, labeledBy: 301, tubeNumber: 8 },
        { id: 2, groupId: 7, tagId: 102, lotId: 202, labeledBy: 302, tubeNumber: 9 },
      ],
    } as any);

    const store = useConjugateStore();
    const lotStore = useLotStore();
    const cloneStore = useCloneStore();
    const proteinStore = useProteinStore();
    const userStore = useUserStore();
    const memberStore = useMemberStore();
    const tagStore = useTagStore();
    const validationStore = useValidationStore();
    const storageStore = useStorageStore();
    const main = useMainStore();

    vi.spyOn(main, "checkApiError").mockResolvedValue(undefined as any);
    vi.spyOn(memberStore, "fetchByIds").mockResolvedValue([
      { id: 301, userId: 401 },
      { id: 302, userId: 402 },
    ] as any);
    vi.spyOn(memberStore, "getMemberById").mockImplementation((id: number) =>
      id === 301 ? ({ id: 301, userId: 401 } as any) : ({ id: 302, userId: 402 } as any)
    );
    const ensureUsersSpy = vi.spyOn(userStore, "fetchByIds").mockResolvedValue([] as any);
    const ensureTagsSpy = vi.spyOn(tagStore, "fetchByIds").mockResolvedValue([] as any);
    const ensureStoragesSpy = vi.spyOn(storageStore, "fetchByIds").mockResolvedValue([] as any);
    vi.spyOn(lotStore, "fetchByIds").mockResolvedValue([
      { id: 201, cloneId: 501 },
      { id: 202, cloneId: 502 },
    ] as any);
    vi.spyOn(lotStore, "getLotById").mockImplementation((id: number) =>
      id === 201 ? ({ id: 201, cloneId: 501 } as any) : ({ id: 202, cloneId: 502 } as any)
    );
    const ensureClonesSpy = vi.spyOn(cloneStore, "fetchByIds").mockResolvedValue([] as any);
    vi.spyOn(cloneStore, "getClone").mockImplementation((id: number) =>
      id === 501 ? ({ id: 501, proteinId: 601 } as any) : ({ id: 502, proteinId: 602 } as any)
    );
    const ensureProteinsSpy = vi.spyOn(proteinStore, "fetchByIds").mockResolvedValue([] as any);
    const ensureValidationsSpy = vi.spyOn(validationStore, "fetchByCloneIds").mockResolvedValue([] as any);

    const result = await store.loadListQuery({
      groupId: 7,
      filters: [],
      order: { table: "Conjugate", field: "id", direction: "asc" },
    } as any);

    expect(result.map((c) => c.id)).toEqual([1, 2]);
    expect(store.listIds).toEqual([1, 2]);
    expect(store.listConjugates.map((c) => c.id)).toEqual([1, 2]);
    expect(ensureUsersSpy).toHaveBeenCalledWith([401, 402]);
    expect(ensureTagsSpy).toHaveBeenCalledWith(7, [101, 102]);
    expect(ensureStoragesSpy).toHaveBeenCalledWith([]);
    expect(ensureClonesSpy).toHaveBeenCalledWith([501, 502]);
    expect(ensureProteinsSpy).toHaveBeenCalledWith(7, [601, 602]);
    expect(ensureValidationsSpy).toHaveBeenCalledWith(7, [501, 502]);
  });

  it("updateConjugate updates the entity immediately for storage changes", async () => {
    const main = mockMainStore();
    const storageStore = useStorageStore();
    const store = useConjugateStore();

    vi.spyOn(jsonApi, "rpcSearch").mockResolvedValue({
      items: [5],
      search_total: 1,
    } as any);

    store.setEntity({
      id: 5,
      groupId: 7,
      tagId: 10,
      lotId: 20,
      labeledBy: 30,
      storageId: 1,
      tubeNumber: 9,
    } as any);

    (jsonApi.rpc as any).mockResolvedValueOnce({
      items: [{
        id: 5,
        groupId: 7,
        tagId: 10,
        lotId: 20,
        labeledBy: 30,
        storageId: 2,
        tubeNumber: 9,
      }],
      total: 1,
    });
    await store.loadListQuery({
      groupId: 7,
      filters: [],
      order: { table: "Conjugate", field: "id", direction: "asc" },
    } as any);

    (jsonApi.rpc as any).mockResolvedValueOnce({});
    (jsonApi.rpc as any).mockResolvedValueOnce({
      items: [{
        id: 5,
        groupId: 7,
        tagId: 10,
        lotId: 20,
        labeledBy: 30,
        storageId: 2,
        tubeNumber: 9,
      }],
      total: 1,
    });
    (jsonApi.rpc as any).mockResolvedValueOnce({
      items: [{
        id: 5,
        groupId: 7,
        tagId: 10,
        lotId: 20,
        labeledBy: 30,
        storageId: 2,
        tubeNumber: 9,
      }],
      total: 1,
    });

    vi.spyOn(storageStore, "fetchByIds").mockResolvedValue([] as any);
    vi.spyOn(main, "checkApiError").mockResolvedValue(undefined as any);

    await store.updateConjugate({
      id: 5,
      data: { storageId: 2 } as any,
    });

    expect(store.entities[5].storageId).toBe(2);
    expect(jsonApi.rpc).toHaveBeenCalledWith(expect.objectContaining({
      operation: "Update",
      return_type: "Conjugate",
      id: 5,
    }));
    expect(jsonApi.rpc).toHaveBeenCalledWith(expect.objectContaining({
      operation: "Get",
      return_type: "Conjugate",
      filters: [{ field: "id", op: "in", value: [5] }],
    }));
  });

  it("resets state correctly", () => {
    const store = useConjugateStore();

    store.setEntity({ id: 1, tagId: 1, tubeNumber: 1 } as any);
    store.reset();

    expect(store.ids).toEqual([]);
    expect(store.entities).toEqual({});
    expect(store.page).toBe(1);
    expect(store.total).toBe(0);
  });
});
