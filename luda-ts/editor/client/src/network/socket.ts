import { ISocketInternal, Socket } from "luda-editor-common";

const webSocket = new WebSocket(`ws://${window.location.hostname}:8001`);
webSocket.binaryType = "arraybuffer";

const queued: Parameters<ISocketInternal["send"]>[0][] = [];

webSocket.onopen = () => {
  queued.forEach((data) => {
    webSocket.send(data);
  });
};

export const socket = new Socket({
  send(data) {
    if (webSocket.readyState === WebSocket.OPEN) {
      return webSocket.send(data);
    }
    queued.push(data);
  },
  setOnMessage: (callback: (data: ArrayBuffer) => void) => {
    webSocket.addEventListener("message", (event) => {
      callback(event.data);
    });
  },
  onError: (callback: (error: Error) => void): void => {
    webSocket.addEventListener("error", (event) => {
      callback(event as any);
    });
  },
  onClose: (callback: () => void): void => {
    webSocket.addEventListener("close", callback);
  },
});
