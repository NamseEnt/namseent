import { MouseEvent as NamuiMouseEvent } from "../../type";

export function toNamuiMouseEvent(mouseEvent: MouseEvent): NamuiMouseEvent {
  return {
    x: mouseEvent.offsetX,
    y: mouseEvent.offsetY,
    isLeftButtonDown: mouseButtonManager.isLeftMouseButtonDown,
    isRightButtonDown: mouseButtonManager.isRightMouseButtonDown,
  };
}
