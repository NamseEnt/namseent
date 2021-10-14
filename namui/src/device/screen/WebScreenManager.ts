import { IManagerInternal } from "../IManager";
import { IScreenManager, VisibilityChangeCallback } from "./IScreenManager";

export class WebScreenManager implements IScreenManager, IManagerInternal {
  private readonly visibilityChangeCallbacks: Set<VisibilityChangeCallback> =
    new Set();
  private readonly visibilitychangeEventListener = () => {
    this.visibilityChangeCallbacks.forEach((callback) => {
      callback(document.hidden);
    });
  };
  constructor() {
    document.addEventListener(
      "visibilitychange",
      this.visibilitychangeEventListener,
    );
  }
  destroy(): void {
    document.removeEventListener(
      "visibilitychange",
      this.visibilitychangeEventListener,
    );
  }
  onVisibilityChange(callback: VisibilityChangeCallback): void {
    this.visibilityChangeCallbacks.add(callback);
  }
  resetBeforeRender(): void {
    this.visibilityChangeCallbacks.clear();
  }
}
