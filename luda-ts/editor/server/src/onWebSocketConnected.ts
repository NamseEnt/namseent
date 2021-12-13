import { WebSocket } from "ws";
import { Socket, ToClientSocket } from "luda-editor-common";
import { toServerRpcHandler } from "./rpcHandlers/toServerRpcHandler";

export function onWebSocketConnected(webSocket: WebSocket) {
  webSocket.binaryType = "arraybuffer";
  const socket: ToClientSocket<{}> = new Socket({
    send: webSocket.send.bind(webSocket),
    setOnMessage: (callback: (data: ArrayBuffer) => void) => {
      webSocket.on("message", (data) => {
        if (!(data instanceof ArrayBuffer)) {
          throw new Error("Expected ArrayBuffer");
        }
        callback(data);
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
