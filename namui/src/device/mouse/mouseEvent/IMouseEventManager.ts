import { MouseEventCallback } from "../../../type";

export interface IMouseEventManager {
  onMouseDown(callback: MouseEventCallback): void;
  onMouseUp(callback: MouseEventCallback): void;
  onMouseMoveOut(callback: MouseEventCallback): void;
}
