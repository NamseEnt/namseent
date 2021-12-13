import { IManagerInternal } from "../../../managers/IManager";
import {
  IMouseButtonManager,
  IMouseButtonManagerInternal,
} from "./IMouseButtonManager";

export class MouseButtonManager
  implements IMouseButtonManager, IManagerInternal, IMouseButtonManagerInternal
{
  private readonly mouseDownButtonSet: Set<number> = new Set();

  constructor() {}
  resetBeforeRender(): void {
    return;
  }
  destroy(): void {
    this.mouseDownButtonSet.clear();
  }

  public get isLeftMouseButtonDown(): boolean {
    return this.mouseDownButtonSet.has(0);
  }
  public get isRightMouseButtonDown(): boolean {
    return this.mouseDownButtonSet.has(2);
  }
  public set isLeftMouseButtonDown(isDown: boolean) {
    if (isDown) {
      this.mouseDownButtonSet.add(0);
    } else {
      this.mouseDownButtonSet.delete(0);
    }
  }
  public set isRightMouseButtonDown(isDown: boolean) {
    if (isDown) {
      this.mouseDownButtonSet.add(2);
    } else {
      this.mouseDownButtonSet.delete(2);
    }
  }
}
