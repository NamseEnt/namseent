import { IMouseButtonManager } from "./IMouseButtonManager";

export class MouseButtonManager implements IMouseButtonManager {
  private readonly mouseDownButtonSet: Set<number> = new Set();

  constructor() {
    document.body.addEventListener("mousedown", (event) => {
      this.mouseDownButtonSet.add(event.button);
    });
    document.body.addEventListener("mouseup", (event) => {
      this.mouseDownButtonSet.delete(event.button);
    });
  }

  public get isLeftMouseButtonDown(): boolean {
    return this.mouseDownButtonSet.has(0);
  }
  public get isRightMouseButtonDown(): boolean {
    return this.mouseDownButtonSet.has(1);
  }
}
