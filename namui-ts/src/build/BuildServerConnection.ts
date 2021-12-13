import { ErrorMessage } from "./type";

export interface IBuildServerConnection {
  addEventListener(eventName: "reload", callback: () => void): void;
  addEventListener(
    eventName: "error",
    callback: (event: { errorMessages: ErrorMessage[] }) => void
  ): void;
  removeEventListener(eventName: "reload" | "error", callback: Function): void;
}

export class BuildServerConnection implements IBuildServerConnection {
  private readonly eventListenersMap: Map<string, Set<Function>> = new Map();
  private readonly webSocket = new WebSocket(
    BuildServerConnection.getBuildServerUrl()
  );
  constructor() {
    this.webSocket.onmessage = this.onMessage.bind(this);
  }
  private onMessage(event: MessageEvent) {
    const data = JSON.parse(event.data);
    switch (data.type) {
      case "reload":
        return this.notifyListeners("reload");
      case "error":
        return this.notifyListeners("error", data);
      default:
        throw new Error(`Unknown message type ${data.type}`);
    }
  }
  notifyListeners(eventName: "reload"): void;
  notifyListeners(eventName: "error", event: { errorMessages: string[] }): void;
  notifyListeners(
    eventName: "reload" | "error",
    event?: { errorMessages: string[] }
  ): void {
    const eventListeners = this.eventListenersMap.get(eventName);
    eventListeners?.forEach((callback) => callback(event));
  }
  static getBuildServerUrl(): URL {
    const url = new URL("", window.location.href);
    url.protocol = url.protocol.replace("http", "ws");
    return url;
  }
  addEventListener(eventName: "reload", callback: () => void): void;
  addEventListener(
    eventName: "error",
    callback: (event: { errorMessages: ErrorMessage[] }) => void
  ): void;
  addEventListener(eventName: any, callback: any): void {
    const eventListeners = this.eventListenersMap.get(eventName) || new Set();
    if (!this.eventListenersMap.has(eventName)) {
      this.eventListenersMap.set(eventName, eventListeners);
    }
    eventListeners.add(callback);
  }
  removeEventListener(eventName: "reload" | "error", callback: Function): void {
    const eventListeners = this.eventListenersMap.get(eventName);
    if (!eventListeners) {
      throw new Error(`Event Listeners ${eventName} does not exist`);
    }
    if (!eventListeners.has(callback)) {
      throw new Error(`Event Listener ${eventName} was not added`);
    }
    eventListeners.delete(callback);
  }
}
