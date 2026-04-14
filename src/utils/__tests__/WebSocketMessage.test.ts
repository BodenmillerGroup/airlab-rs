import { describe, it, expect } from "vitest";
import { WebSocketMessage } from "@/utils/WebSocketMessage";

describe("WebSocketMessage", () => {
  it("maps fields correctly from json", () => {
    const input = {
      groupId: 42,
      type: "CLONE_UPDATED",
      payload: { id: 7, name: "Clone A" },
    };

    const msg = new WebSocketMessage(input);

    expect(msg.groupId).toBe(42);
    expect(msg.type).toBe("CLONE_UPDATED");
    expect(msg.payload).toEqual({ id: 7, name: "Clone A" });
  });

  it("allows missing fields without throwing", () => {
    const input = {};

    const msg = new WebSocketMessage(input as any);

    expect(msg.groupId).toBeUndefined();
    expect(msg.type).toBeUndefined();
    expect(msg.payload).toBeUndefined();
  });

  it("does not mutate the input object", () => {
    const input = {
      groupId: 1,
      type: "TEST",
      payload: { x: 1 },
    };

    const copy = structuredClone(input);
    new WebSocketMessage(input);

    expect(input).toEqual(copy);
  });

  it("accepts extra fields but ignores them", () => {
    const input = {
      groupId: 5,
      type: "X",
      payload: { ok: true },
      extra: "noise",
    };

    const msg = new WebSocketMessage(input as any);

    expect((msg as any).extra).toBeUndefined();
    expect(msg.groupId).toBe(5);
  });
});
