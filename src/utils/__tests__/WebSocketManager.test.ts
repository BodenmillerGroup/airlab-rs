
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { initWebSocket, connectWebSocket, closeWebSocket } from "@/utils/WebSocketManager";
import { WebSocketMessage } from "@/utils/WebSocketMessage";

/* --------------------------------------------
   Mock WebSocketMessage
--------------------------------------------- */
vi.mock("@/utils/WebSocketMessage", () => {
  return {
    WebSocketMessage: vi.fn().mockImplementation((data) => ({ wrapped: data }))
  };
});

/* --------------------------------------------
   Fake WebSocket implementation
--------------------------------------------- */
class MockWebSocket {
  static instances: MockWebSocket[] = [];

  url: string;
  onopen: any = null;
  onclose: any = null;
  onmessage: any = null;
  onerror: any = null;
  close = vi.fn();

  constructor(url: string) {
    this.url = url;
    MockWebSocket.instances.push(this);
  }

  triggerOpen() {
    this.onopen?.();
  }

  triggerMessage(data: any) {
    this.onmessage?.({ data: JSON.stringify(data) });
  }

  triggerClose(reason = "test") {
    this.onclose?.({ reason });
  }

  triggerError(err = "err") {
    this.onerror?.(err);
  }
}

/* --------------------------------------------
   Test suite
--------------------------------------------- */
describe("WebSocketManager", () => {
  beforeEach(() => {
    MockWebSocket.instances = [];
    vi.stubGlobal("WebSocket", MockWebSocket);
    vi.stubGlobal("location", { hostname: "example.com" });
    vi.stubGlobal("self", { location: { protocol: "http:" } });
    vi.useFakeTimers();
  });

  afterEach(() => {
    closeWebSocket();
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it("does not connect if token is missing", () => {
    connectWebSocket(1);
    expect(MockWebSocket.instances.length).toBe(0);
  });

  it("initializes and opens a websocket with correct url", () => {
    initWebSocket("ABC123");
    connectWebSocket(42);

    expect(MockWebSocket.instances.length).toBe(1);
    expect(MockWebSocket.instances[0].url)
      .toBe("ws://example.com/ws/42?token=ABC123");
  });

  it("uses wss when page is https", () => {
    (self as any).location.protocol = "https:";

    initWebSocket("SECURE");
    connectWebSocket(5);

    expect(MockWebSocket.instances[0].url)
      .toBe("wss://example.com/ws/5?token=SECURE");
  });

  it("wraps incoming messages using WebSocketMessage", () => {
    initWebSocket("TOKEN");
    connectWebSocket(7);

    const socket = MockWebSocket.instances[0];
    socket.triggerMessage({ hello: "world" });

    expect(WebSocketMessage).toHaveBeenCalledOnce();
    expect(WebSocketMessage).toHaveBeenCalledWith({ hello: "world" });
  });

  it("reconnects automatically when closed", () => {
    initWebSocket("TOKEN");
    connectWebSocket(3);

    const first = MockWebSocket.instances[0];
    first.triggerClose("boom");

    expect(MockWebSocket.instances.length).toBe(1);

    vi.advanceTimersByTime(1000);

    expect(MockWebSocket.instances.length).toBe(2);
    expect(MockWebSocket.instances[1].url)
      .toBe("ws://example.com/ws/3?token=TOKEN");
  });

  it("cleans up and closes socket", () => {
    initWebSocket("TOKEN");
    connectWebSocket(9);

    const socket = MockWebSocket.instances[0];
    closeWebSocket();

    expect(socket.close).toHaveBeenCalledOnce();
    expect(socket.onopen).toBe(null);
    expect(socket.onclose).toBe(null);
    expect(socket.onmessage).toBe(null);
    expect(socket.onerror).toBe(null);
  });

  it("handles socket error without crashing", () => {
    initWebSocket("TOKEN");
    connectWebSocket(9);

    const socket = MockWebSocket.instances[0];

    expect(() => socket.triggerError("boom")).not.toThrow();
  });
});
