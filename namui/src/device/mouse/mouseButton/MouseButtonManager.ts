import { IManagerInternal } from "../../../managers/IManager";
import { IMouseButtonManager } from "./IMouseButtonManager";

export class MouseButtonManager
  implements IMouseButtonManager, IManagerInternal
{
  private readonly mouseDownButtonSet: Set<number> = new Set();
  private readonly eventListenerTuples = [
    [
      "mousedown",
      (event: MouseEvent) => {
        this.mouseDownButtonSet.add(event.button);
      },
    ],
    [
      "mouseup",
      (event: MouseEvent) => {
        this.mouseDownButtonSet.delete(event.button);
      },
    ],
  ] as const;

  constructor() {
    this.eventListenerTuples.forEach(([eventName, listener]) => {
      document.addEventListener(eventName, listener);
    });
  }
  resetBeforeRender(): void {
    return;
  }
  destroy(): void {
    this.eventListenerTuples.forEach(([eventName, listener]) => {
      document.removeEventListener(eventName, listener);
    });
  }

  public get isLeftMouseButtonDown(): boolean {
    return this.mouseDownButtonSet.has(0);
  }
  public get isRightMouseButtonDown(): boolean {
    return this.mouseDownButtonSet.has(2);
  }
}
