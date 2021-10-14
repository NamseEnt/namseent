import { MouseEventCallback } from "../../../type";

export interface IMouseEventManager {
  onMouseDown(callback: MouseEventCallback): void;
  onMouseUp(callback: MouseEventCallback): void;
  onMouseOut(callback: MouseEventCallback): void;
}
