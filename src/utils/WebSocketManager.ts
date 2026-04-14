import { WebSocketMessage } from "@/utils/WebSocketMessage";

let socket: WebSocket | null = null;
let token: string;
let protocol: string;

export function initWebSocket(tokenParam: string) {
  token = tokenParam;
  protocol = self.location.protocol === "https:" ? "wss:" : "ws:";
}

export function connectWebSocket(groupId: number) {
  if (!token) {
    console.warn("[WebSocketManager] Token not set.");
    return;
  }

  closeWebSocket();

  const url = `${protocol}//${location.hostname}/ws/${groupId}?token=${token}`;
  socket = new WebSocket(url);

  socket.onopen = () => {};

  socket.onclose = (event) => {
    console.warn("[WebSocket] Closed, reconnecting…", event.reason);
    setTimeout(() => connectWebSocket(groupId), 1000);
  };

  socket.onmessage = (event) => {
    const data = JSON.parse(event.data);
    new WebSocketMessage(data);
  };

  socket.onerror = (event) => {
    console.error("[WebSocket] Error:", event);
  };
}

export function closeWebSocket() {
  if (socket) {
    socket.onopen = null;
    socket.onclose = null;
    socket.onmessage = null;
    socket.onerror = null;
    socket.close();
    socket = null;
  }
}
