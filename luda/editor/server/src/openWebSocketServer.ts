import { WebSocket, WebSocketServer } from "ws";

export function openWebSocketServer({
  port,
  onConnected,
  onDisconnected,
}: {
  port: number;
  onConnected: (socket: WebSocket) => void;
  onDisconnected: (socket: WebSocket) => void;
}) {
  const wss = new WebSocketServer({ port });

  wss.on("connection", (ws) => {
    onConnected(ws);
    ws.on("close", () => {
      onDisconnected(ws);
    });
  });
}
