import { Vector } from "../../../type";
import { IManagerInternal } from "../../IManager";
import { IMousePositionManager } from "./IMousePositionManager";

export class WebMousePositionManager
  implements IMousePositionManager, IManagerInternal
{
  public mousePosition: Vector = new Vector(0, 0);
  private readonly mouseEventNames = [
    "click",
    "contextmenu",
    "dblclick",
    "mousedown",
    "mouseenter",
    "mouseleave",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
  ] as const;
  constructor() {
    this.onMouseEvent = this.onMouseEvent.bind(this);

    this.mouseEventNames.forEach((eventName) =>
      document.addEventListener(eventName, this.onMouseEvent),
    );
  }
  private onMouseEvent(event: MouseEvent): void {
    this.mousePosition = new Vector(event.clientX, event.clientY);
  }
  destroy(): void {
    this.mouseEventNames.forEach((eventName) =>
      document.removeEventListener(eventName, this.onMouseEvent),
    );
  }
  resetBeforeRender(): void {
    // do nothing
  }
}
