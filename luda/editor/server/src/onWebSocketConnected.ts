import { WebSocket } from "ws";
import { Socket, ToClientSocket } from "luda-editor-common";
import { toServerRpcHandler } from "./rpcHandlers/toServerRpcHandler";

export function onWebSocketConnected(webSocket: WebSocket) {
  const socket: ToClientSocket<{}> = new Socket({
    send: webSocket.send.bind(webSocket),
    setOnMessage: (callback: (data: string) => void) => {
      webSocket.on("message", (data) => {
        const stringData = typeof data !== "string" ? data.toString() : data;
        callback(stringData);
      });
    },
    onError: (callback: (error: Error) => void): void => {
      webSocket.on("error", callback);
    },
    onClose: (callback: () => void): void => {
      webSocket.on("close", callback);
    },
  });

  socket.setHandler({}, toServerRpcHandler);
}
