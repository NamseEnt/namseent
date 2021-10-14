import { Socket } from "luda-editor-common";

const webSocket = new WebSocket(`ws://${window.location.hostname}:8001`);
export const socket = new Socket({
  send: webSocket.send.bind(webSocket),
  setOnMessage: (callback: (data: string) => void) => {
    console.log("hi");
    webSocket.addEventListener("message", (event) => {
      console.log("message", event);
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
