import { WheelEventCallback } from "../../type";
import { IManagerInternal } from "../../managers/IManager";
import { IWheelManager } from "./IWheelManager";

export class WebWheelManager implements IWheelManager, IManagerInternal {
  private readonly wheelCallbacks: Set<WheelEventCallback> = new Set();
  private readonly wheelEventListener = (event: WheelEvent) => {
    this.wheelCallbacks.forEach((callback) => callback(event));
  };
  constructor() {
    document.addEventListener("wheel", this.wheelEventListener);
  }
  destroy(): void {
    document.removeEventListener("wheel", this.wheelEventListener);
  }
  resetBeforeRender(): void {
    this.wheelCallbacks.clear();
  }
  onWheel(callback: WheelEventCallback): void {
    this.wheelCallbacks.add(callback);
  }
}
